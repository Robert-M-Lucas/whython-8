use either::{Left, Right};
use itertools::Itertools;

use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::evaluation::coerce_self::coerce_self;
use crate::root::compiler::evaluation::new::compile_evaluable_new;
use crate::root::compiler::evaluation::reference::compile_evaluable_reference;
use crate::root::compiler::evaluation::{function_only, reference, type_only};
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::WrongAttributeNameInInit;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::location::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{AddressedTypeRef, FunctionID, LocalAddress};

/// Evaluates `et` putting the result into `target`
pub fn compile_evaluable_into(
    fid: FunctionID,
    et: &EvaluableToken,
    target: AddressedTypeRef,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker,
) -> Result<String, WErr> {
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
                    if address.type_ref() != target.type_ref() {
                        return WErr::ne(
                            EvalErrs::ExpectedDifferentType(
                                global_table.get_type_name(target.type_ref()),
                                global_table.get_type_name(address.type_ref()),
                            ),
                            name.location().clone(),
                        );
                    }
                    copy(
                        *address.local_address(),
                        *target.local_address(),
                        global_table.get_size(target.type_ref()),
                    )
                }
            }
        }
        EvaluableTokens::Literal(literal) => {
            if target.type_ref().indirection().has_indirection() {
                return WErr::ne(
                    EvalErrs::BadIndirection(target.type_ref().indirection().0, 0),
                    literal.location().clone(),
                );
            }
            let t = global_table.get_type(*target.type_ref().type_id());

            t.instantiate_from_literal(target.local_address(), literal)?
        }
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            if op.operator() == &OperatorTokens::Assign {
                return WErr::ne(
                    EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())),
                    et.location().clone(),
                );
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

            match signature.return_type() {
                None => {
                    return WErr::ne(
                        EvalErrs::OpNoReturn(global_table.get_type_name(target.type_ref())),
                        op.location().clone(),
                    )
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return WErr::ne(
                            EvalErrs::OpWrongReturnType(
                                global_table.get_type_name(target.type_ref()),
                                global_table.get_type_name(rt),
                            ),
                            op.location().clone(),
                        );
                    }
                }
            }

            let (c, _) = call_function(
                fid,
                op_fn,
                signature.self_type().uses_self(),
                et.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Infix)
                    .unwrap(),
                &[Left(lhs), Left(rhs)],
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;

            c
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
                        return WErr::ne(EvalErrs::ExpectedNotNone, lhs.location().clone());
                    };

                    if *val.type_ref() != lhs_type {
                        panic!()
                    }
                    c += &set_reference(op.location(), val, target, global_table)?;
                    return Ok(c);
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
                        return WErr::ne(EvalErrs::ExpectedNotNone, lhs.location().clone());
                    };

                    c += &set_deref(lhs.location(), val, target, global_table)?;
                    return Ok(c);
                }
                _ => {}
            };

            let op_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Prefix,
            )?;
            let signature = global_table.get_function_signature(op_fn);

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

            match signature.return_type() {
                None => {
                    return WErr::ne(
                        EvalErrs::OpNoReturn(global_table.get_type_name(target.type_ref())),
                        op.location().clone(),
                    )
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return WErr::ne(
                            EvalErrs::OpWrongReturnType(
                                global_table.get_type_name(target.type_ref()),
                                global_table.get_type_name(rt),
                            ),
                            op.location().clone(),
                        );
                    }
                }
            }

            let (c, _) = call_function(
                fid,
                op_fn,
                signature.self_type().uses_self(),
                et.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Prefix)
                    .unwrap(),
                &[Left(lhs)],
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;
            c
        }
        EvaluableTokens::DynamicAccess(inner_eval, access) => {
            let mut ab = AssemblyBuilder::new();
            let (c, inner) = compile_evaluable_reference(
                fid,
                inner_eval,
                local_variables,
                global_table,
                global_tracker,
            )?;
            ab.other(&c);

            let Some(inner) = inner else {
                return WErr::ne(EvalErrs::ExpectedNotNone, inner_eval.location().clone());
            };

            let inner = if inner.type_ref().indirection().0 > 1 {
                let (c, inner) =
                    coerce_self(inner, SelfType::RefSelf, global_table, local_variables)?;
                ab.other(&c);
                inner
            } else {
                inner
            };

            let t = global_table.get_type(*inner.type_ref().type_id());
            let attribs = t.get_attributes(access.location())?;
            let mut found_offset = None;

            for (offset, name, t) in attribs {
                if name.name() == access.name() {
                    if &t.plus_one_indirect() != target.type_ref() {
                        return WErr::ne(
                            EvalErrs::ExpectedDifferentType(
                                global_table.get_type_name(target.type_ref()),
                                global_table.get_type_name(&t.plus_one_indirect()),
                            ),
                            access.location().clone(),
                        );
                    }
                    found_offset = Some(*offset);
                }
            }

            let Some(found_offset) = found_offset else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveAttribute(
                        global_table.get_type_name(&t.id().immediate()),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            };

            if inner.type_ref().indirection().has_indirection() {
                ab.line(&format!("mov rax, qword {}", inner.local_address()));
                ab.line(&format!("add rax, {:#018x}", found_offset.0));
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
                    AddressedTypeRef::new(
                        LocalAddress(inner.local_address().0 + (found_offset.0 as isize)),
                        target.type_ref().minus_one_indirect(),
                    ),
                    target.clone(),
                    global_table,
                )?);

                // ab.other(&copy(
                //     LocalAddress(inner.local_address().0 + found_offset.0 as isize),
                //     *target.local_address(),
                //     global_table.get_size(target.type_ref()),
                // ));
            }

            ab.finish()
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
            let self_type = *global_table.get_function_signature(ifid).self_type();

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
                self_type.uses_self(),
                et.location(),
                &name,
                &n_args,
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;
            ab.other(&c);
            ab.finish()
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let t = global_table.resolve_to_type_ref(struct_init.name())?;

            if *struct_init.heap_alloc() {
                if target.type_ref() != &t.plus_one_indirect() {
                    return WErr::ne(
                        EvalErrs::ExpectedDifferentType(
                            global_table.get_type_name(target.type_ref()),
                            global_table.get_type_name(&t.plus_one_indirect()),
                        ),
                        struct_init.location().clone(),
                    );
                }
                let mut ab = AssemblyBuilder::new();
                let (c, sr) =
                    compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?;
                ab.other(&c);
                let sr = sr.unwrap();
                ab.other(&copy(
                    *sr.local_address(),
                    *target.local_address(),
                    global_table.get_size(target.type_ref()),
                ));
                return Ok(ab.finish());
            }
            debug_assert!(!t.indirection().has_indirection());

            if *struct_init.heap_alloc() && &t.plus_one_indirect() != target.type_ref() {
                return WErr::ne(
                    EvalErrs::ExpectedDifferentType(
                        global_table.get_type_name(target.type_ref()),
                        global_table.get_type_name(&t.plus_one_indirect()),
                    ),
                    struct_init.location().clone(),
                );
            }
            if !struct_init.heap_alloc() && &t != target.type_ref() {
                return WErr::ne(
                    EvalErrs::ExpectedDifferentType(
                        global_table.get_type_name(target.type_ref()),
                        global_table.get_type_name(&t),
                    ),
                    struct_init.location().clone(),
                );
            }

            let tt = global_table.get_type(*t.type_id());
            let attributes = tt
                .get_attributes(struct_init.location())
                .map_err(|_| {
                    WErr::n(
                        EvalErrs::TypeCannotBeInitialised(tt.name().to_string()),
                        struct_init.location().clone(),
                    )
                })?
                .iter()
                .cloned()
                .collect_vec();
            let give_attrs = struct_init.contents();

            if attributes.len() != give_attrs.len() {
                return WErr::ne(
                    EvalErrs::WrongAttributeCount(attributes.len(), give_attrs.len()),
                    struct_init.location().clone(),
                );
            }

            let mut code = AssemblyBuilder::new();

            for ((offset, t_name, t_type), (name, val)) in attributes.iter().zip(give_attrs.iter())
            {
                if t_name.name() != name.name() {
                    return WErr::ne(
                        WrongAttributeNameInInit(t_name.name().clone(), name.name().clone()),
                        name.location().clone(),
                    );
                }

                let new_addr = AddressedTypeRef::new(
                    LocalAddress(target.local_address().0 + offset.0 as isize),
                    t_type.clone(),
                );
                code.other(&compile_evaluable_into(
                    fid,
                    val,
                    new_addr,
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
            }

            code.finish()
        }
        EvaluableTokens::None => {
            return WErr::ne(
                EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())),
                et.location().clone(),
            );
        }
    })
}
