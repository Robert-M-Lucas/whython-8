use either::{Left, Right};
use itertools::Itertools;

use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::heap::heap_alloc;
use crate::root::compiler::assembly::utils::{
    copy, copy_to_indirect,
};
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::evaluation::{function_only, into, reference, type_only};
use crate::root::compiler::evaluation::reference::compile_evaluable_reference;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedNotNone, ExpectedReference};
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::shared::common::{
    AddressedTypeRef, FunctionID, Indirection, LocalAddress, TypeRef,
};
use crate::root::shared::types::Type;

/// Evaluates `et` into a new address
pub fn compile_evaluable_new(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(String, Option<AddressedTypeRef>), WErr> {
    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => {
                    return WErr::ne(
                        EvalErrs::FunctionMustBeCalled(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Type(_) => {
                    return WErr::ne(
                        EvalErrs::CannotEvalStandaloneType(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Variable(address) => {
                    let target = global_table.add_local_variable_unnamed_base(
                        address.type_ref().clone(),
                        local_variables,
                    );
                    (
                        copy(
                            *address.local_address(),
                            *target.local_address(),
                            global_table.get_size(target.type_ref()),
                        ),
                        Some(target),
                    )
                }
            }
        }
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            let address = global_table.add_local_variable_unnamed_base(
                TypeRef::new(tid, Indirection(0)),
                local_variables,
            );
            let t = global_table.get_type(tid);

            (
                t.instantiate_from_literal(address.local_address(), literal)?,
                Some(address),
            )
        }
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            match op.operator() {
                OperatorTokens::Assign => {
                    let (mut c, into) = compile_evaluable_new(
                        fid,
                        lhs,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;

                    let Some(into) = into else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone());
                    };
                    if !into.type_ref().indirection().has_indirection() {
                        return WErr::ne(
                            ExpectedReference(global_table.get_type_name(into.type_ref())),
                            lhs.location().clone(),
                        );
                    }

                    let val = global_table.add_local_variable_unnamed_base(
                        into.type_ref().minus_one_indirect(),
                        local_variables,
                    );
                    c += &into::compile_evaluable_into(
                        fid,
                        rhs,
                        val.clone(),
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;

                    c += &copy_to_indirect(
                        *val.local_address(),
                        *into.local_address(),
                        global_table.get_size(val.type_ref()),
                    );
                    return Ok((c, None));
                }
                _ => {}
            };

            let lhs_type = type_only::compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let op_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Infix,
            )?;
            let signature = global_table.get_function_signature(op_fn);

            let uses_self = signature.self_type().uses_self();

            if signature.args().len() != 2 {
                return WErr::ne(
                    EvalErrs::InfixOpWrongArgumentCount(
                        op.operator().to_str().to_string(),
                        global_table
                            .get_type(*lhs_type.type_id())
                            .name()
                            .to_string(),
                        op.operator()
                            .get_method_name(PrefixOrInfixEx::Infix)
                            .unwrap(),
                        signature.args().len(),
                    ),
                    op.location().clone(),
                );
            }

            let return_type = signature.return_type().clone();
            let return_into = return_type.map(|rt| {
                global_table.add_local_variable_unnamed_base(rt.clone(), local_variables)
            });

            let (c, _) = call_function(
                fid,
                op_fn,
                uses_self,
                op.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Infix)
                    .unwrap(),
                &[Left(lhs), Left(rhs)],
                return_into.clone(),
                global_table,
                local_variables,
                global_tracker,
            )?;

            (c, return_into)
        }
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = type_only::compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            match op.operator() {
                OperatorTokens::Reference => {
                    let (mut c, val) = reference::compile_evaluable_reference(
                        fid,
                        lhs,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;
                    let Some(val) = val else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone());
                    };

                    if *val.type_ref() != lhs_type {
                        panic!()
                    }
                    let target = global_table.add_local_variable_unnamed_base(
                        val.type_ref().plus_one_indirect(),
                        local_variables,
                    );
                    c += &set_reference(op.location(), val, target.clone(), global_table)?;
                    return Ok((c, Some(target)));
                }
                OperatorTokens::Multiply => {
                    let (mut c, val) = reference::compile_evaluable_reference(
                        fid,
                        lhs,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;
                    let Some(val) = val else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone());
                    };
                    if !val.type_ref().indirection().has_indirection() {
                        return WErr::ne(
                            ExpectedReference(global_table.get_type_name(val.type_ref())),
                            lhs.location().clone(),
                        );
                    }
                    let target = global_table.add_local_variable_unnamed_base(
                        val.type_ref().minus_one_indirect(),
                        local_variables,
                    );
                    c += &set_deref(lhs.location(), val, target.clone(), global_table)?;
                    return Ok((c, Some(target)));
                }
                _ => {}
            };

            let op_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Prefix,
            )?;
            let signature = global_table.get_function_signature(op_fn);
            let uses_self = signature.self_type().uses_self();

            if signature.args().len() != 1 {
                return WErr::ne(
                    EvalErrs::InfixOpWrongArgumentCount(
                        op.operator().to_str().to_string(),
                        global_table
                            .get_type(*lhs_type.type_id())
                            .name()
                            .to_string(),
                        op.operator()
                            .get_method_name(PrefixOrInfixEx::Prefix)
                            .unwrap(),
                        signature.args().len(),
                    ),
                    op.location().clone(),
                );
            }

            let return_type = signature.return_type().clone();
            let return_into = return_type.map(|rt| {
                global_table.add_local_variable_unnamed_base(rt.clone(), local_variables)
            });

            let (c, _) = call_function(
                fid,
                op_fn,
                uses_self,
                et.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Prefix)
                    .unwrap(),
                &[Left(lhs)],
                return_into.clone(),
                global_table,
                local_variables,
                global_tracker,
            )?;
            (c, return_into)
        }
        EvaluableTokens::DynamicAccess(inner, access) => {
            let mut ab = AssemblyBuilder::new();
            let (c, inner) = compile_evaluable_reference(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;
            ab.other(&c);

            let Some(inner) = inner else { todo!() };

            if inner.type_ref().indirection().0 > 1 {
                todo!()
            }

            let t = global_table.get_type(*inner.type_ref().type_id());
            let attribs = t.get_attributes()?;

            let mut found = None;

            for (offset, name, t) in attribs {
                if name.name() == access.name() {
                    found = Some((*offset, t.clone()));
                }
            }

            let Some((found_offset, t)) = found else {
                todo!()
            };

            // let t = t.plus_one_indirect();

            let target = global_table.add_local_variable_unnamed_base(t.plus_one_indirect(), local_variables);

            if inner.type_ref().indirection().has_indirection() {
                // TODO: Not 64 bit!
                ab.line(&format!("mov rax, {}", inner.local_address()));
                ab.line(&format!("add rax, {}", found_offset.0));
                ab.line(&format!("mov qword {}, rax", target.local_address()));

                // ab.other(&copy_from_indirect_fixed_offset(
                //     LocalAddress(inner.local_address().0),
                //     found_offset,
                //     *target.local_address(),
                //     global_table.get_size(target.type_ref()),
                // ));
            } else {
                ab.other(&set_reference(
                    &Location::builtin(),
                    AddressedTypeRef::new(LocalAddress(inner.local_address().0 + (found_offset.0 as isize)), t),
                    target.clone(),
                    global_table
                )?);

                // ab.other(&copy(
                //     LocalAddress(inner.local_address().0 + found_offset.0 as isize),
                //     *target.local_address(),
                //     global_table.get_size(target.type_ref()),
                // ));
            }

            (ab.finish(), Some(target))
        }
        EvaluableTokens::StaticAccess(_, n) => {
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(n.name().clone()),
                n.location().clone(),
            )
        } // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let mut ab = AssemblyBuilder::new();
            let (slf, ifid, name) = function_only::compile_evaluable_function_only(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let signature = global_table.get_function_signature(ifid);
            let uses_self = signature.self_type().uses_self();
            let return_into = signature
                .return_type()
                .clone()
                .map(|r| global_table.add_local_variable_unnamed_base(r, local_variables));

            let mut n_args = if let Some(slf) = slf.as_ref() {
                let mut v = Vec::with_capacity(args.len() + 1);
                let (c, slf) = compile_evaluable_reference(
                    fid,
                    slf,
                    local_variables,
                    global_table,
                    global_tracker,
                )?;
                ab.other(&c);
                let slf = slf.unwrap();

                v.push(Right(slf));
                v
            } else {
                Vec::with_capacity(args.len())
            };

            args.iter().for_each(|a| n_args.push(Left(a)));

            let (c, _) = call_function(
                fid,
                ifid,
                uses_self,
                et.location(),
                &name,
                &n_args,
                return_into.clone(),
                global_table,
                local_variables,
                global_tracker,
            )?;
            ab.other(&c);
            (ab.finish(), return_into)
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let t = global_table.resolve_to_type_ref(struct_init.name())?;
            let size = global_table.get_size(&t);

            let mut code = AssemblyBuilder::new();

            let target = // if struct_init.heap_alloc() {
            //     let (c, ref_target) = heap_alloc(t.clone(), global_table, local_variables);
            //     code.other(&c);
            //     ref_target
            // } else {
                global_table.add_local_variable_unnamed_base(t.clone(), local_variables)
            /* } */;

            let tt = global_table.get_type(t.type_id().clone());
            let attributes = tt.get_attributes()?.iter().map(|x| x.clone()).collect_vec();
            let give_attrs = struct_init.contents();

            if attributes.len() != give_attrs.len() {
                todo!()
            }

            for ((offset, t_name, t_type), (name, val)) in attributes.iter().zip(give_attrs.iter())
            {
                if t_name.name() != name.name() {
                    todo!()
                }

                let new_addr = AddressedTypeRef::new(
                    LocalAddress(target.local_address().0 + offset.0 as isize),
                    t_type.clone(),
                );
                code.other(&into::compile_evaluable_into(
                    fid,
                    val,
                    new_addr,
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
            }

            if *struct_init.heap_alloc() {
                let (c, ref_target) = heap_alloc(t, global_table, local_variables);
                code.other(&c);
                code.other(&copy_to_indirect(*target.local_address(), *ref_target.local_address(), size));
                (code.finish(), Some(ref_target))
            }
            else {
                (code.finish(), Some(target))
            }
        }
        EvaluableTokens::None => (String::new(), None),
    })
}
