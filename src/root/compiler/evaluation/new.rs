use either::{Left, Right};
use itertools::Itertools;

use crate::root::assembler::assembly_builder::{Assembly, AssemblyBuilder};
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::heap::heap_alloc;
use crate::root::compiler::assembly::utils::{copy, copy_to_indirect};
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::evaluation::coerce_self::coerce_self;
use crate::root::compiler::evaluation::reference::compile_evaluable_reference;
use crate::root::compiler::evaluation::{function_only, into, reference, type_only};
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedNotNone, WrongAttributeNameInInit};
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalTable, NameResult};
use crate::root::parser::location::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{
    AddressedTypeRef, FunctionID, Indirection, LocalAddress, TypeRef,
};

/// Evaluates `et` into a new address
pub fn compile_evaluable_new(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(Assembly, Option<AddressedTypeRef>), WErr> {
    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(
                name,
                None,
                containing_class.as_ref(),
                local_variables,
                global_tracker,
            )? {
                NameResult::Function(_) => {
                    // Name was function (no call at the end)
                    return WErr::ne(
                        EvalErrs::FunctionMustBeCalled(name.name().clone()),
                        name.location().clone(),
                    );
                }
                NameResult::Type(_) => {
                    // Name was a type (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvalStandaloneType(name.name().clone()),
                        name.location().clone(),
                    );
                }
                NameResult::File(_) => {
                    // Name was a file (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvaluateStandaloneImportedFile(name.name().clone()),
                        name.location().clone(),
                    );
                }
                NameResult::Variable(address) => {
                    let target = global_table
                        .add_local_variable_unnamed(address.type_ref().clone(), local_variables);
                    // Copy variable into output
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
            let type_id = literal.literal().default_type();
            // TODO: Don't use 1 element
            let address = global_table.add_local_variable_unnamed(
                TypeRef::new(type_id, 1, Indirection(0)),
                local_variables,
            );
            let new_type = global_table.get_type(type_id);

            (
                new_type.instantiate_from_literal(address.local_address(), literal)?,
                Some(address),
            )
        }
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            // Handle assignment
            if op.operator() == &OperatorTokens::Assign {
                let (mut asm, into) =
                    compile_evaluable_new(fid, lhs, local_variables, global_table, global_tracker)?;

                let Some(into) = into else {
                    return WErr::ne(EvalErrs::ExpectedNotNone, lhs.location().clone());
                };

                if !into.type_ref().indirection().has_indirection() {
                    return WErr::ne(
                        EvalErrs::ExpectedReference(global_table.get_type_name(into.type_ref())),
                        lhs.location().clone(),
                    );
                }

                // Create output
                let val = global_table.add_local_variable_unnamed(
                    into.type_ref().minus_one_indirect(),
                    local_variables,
                );
                // Evaluate
                asm += &into::compile_evaluable_into(
                    fid,
                    rhs,
                    val.clone(),
                    local_variables,
                    global_table,
                    global_tracker,
                )?;
                // Copy to output
                asm += &copy_to_indirect(
                    *val.local_address(),
                    *into.local_address(),
                    global_table.get_size(val.type_ref()),
                );
                return Ok((asm, None));
            };

            let lhs_type = type_only::compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let operator_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Infix,
            )?;
            let operator_fn_signature = global_table.get_function_signature(operator_fn);

            let uses_self = operator_fn_signature.self_type().uses_self();

            if operator_fn_signature.args().len() != 2 {
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
                        operator_fn_signature.args().len(),
                    ),
                    op.location().clone(),
                );
            }

            let return_type = operator_fn_signature.return_type().clone();
            // Create output
            let return_into = return_type
                .map(|rt| global_table.add_local_variable_unnamed(rt.clone(), local_variables));

            let (asm, _) = call_function(
                fid,
                operator_fn,
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

            (asm, return_into)
        }
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = type_only::compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            // Handle special cases for referencing and dereferencing
            match op.operator() {
                OperatorTokens::Reference => {
                    let (mut asm, val) = reference::compile_evaluable_reference(
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
                    let target = global_table.add_local_variable_unnamed(
                        val.type_ref().plus_one_indirect(),
                        local_variables,
                    );
                    asm += &set_reference(op.location(), val, target.clone(), global_table)?;
                    return Ok((asm, Some(target)));
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
                    if !val.type_ref().indirection().has_indirection() {
                        return WErr::ne(
                            EvalErrs::ExpectedReference(global_table.get_type_name(val.type_ref())),
                            lhs.location().clone(),
                        );
                    }
                    let target = global_table.add_local_variable_unnamed(
                        val.type_ref().minus_one_indirect(),
                        local_variables,
                    );
                    c += &set_deref(lhs.location(), val, target.clone(), global_table)?;
                    return Ok((c, Some(target)));
                }
                _ => {}
            };

            let operator_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Prefix,
            )?;
            let operator_signature = global_table.get_function_signature(operator_fn);
            let uses_self = operator_signature.self_type().uses_self();

            if operator_signature.args().len() != 1 {
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
                        operator_signature.args().len(),
                    ),
                    op.location().clone(),
                );
            }

            let return_type = operator_signature.return_type().clone();
            let return_into = return_type
                .map(|rt| global_table.add_local_variable_unnamed(rt.clone(), local_variables));

            let (asm, _) = call_function(
                fid,
                operator_fn,
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
            (asm, return_into)
        }
        EvaluableTokens::DynamicAccess {
            parent: inner_eval,
            section: access,
        } => {
            let mut ab = AssemblyBuilder::new();
            let (asm, inner) = compile_evaluable_reference(
                fid,
                inner_eval,
                local_variables,
                global_table,
                global_tracker,
            )?;
            ab.other(&asm);

            let Some(inner) = inner else {
                return WErr::ne(ExpectedNotNone, inner_eval.location().clone());
            };

            let inner = if inner.type_ref().indirection().0 > 1 {
                let (c, inner) =
                    coerce_self(inner, SelfType::RefSelf, global_table, local_variables)?;
                ab.other(&c);
                inner
            } else {
                inner
            };

            let inner_type = global_table.get_type(*inner.type_ref().type_id());
            let inner_attributes = inner_type.get_attributes(access.location())?;

            let mut found = None;

            // Find byte offset of the attribute
            for (offset, name, t) in inner_attributes {
                if name.name() == access.name() {
                    found = Some((*offset, t.clone()));
                }
            }

            let Some((found_offset, t)) = found else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveAttribute(
                        inner_type.name().to_string(),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            };

            let target =
                global_table.add_local_variable_unnamed(t.plus_one_indirect(), local_variables);

            if inner.type_ref().indirection().has_indirection() {
                // If inner is a reference
                // Get inner address
                ab.line(&format!("mov rax, qword {}", inner.local_address()));
                // Add offset
                ab.line(&format!("add rax, {:#018x}", found_offset.0));
                // Write to output
                ab.line(&format!("mov qword {}, rax", target.local_address()));

                // ab.other(&copy_from_indirect_fixed_offset(
                //     LocalAddress(inner.local_address().0),
                //     found_offset,
                //     *target.local_address(),
                //     global_table.get_size(target.type_ref()),
                // ));
            } else {
                // Put a reference to the attribute into the output
                ab.other(&set_reference(
                    &Location::builtin(),
                    AddressedTypeRef::new(
                        LocalAddress(inner.local_address().0 + (found_offset.0 as isize)),
                        t,
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

            (ab.finish(), Some(target))
        }
        EvaluableTokens::StaticAccess {
            parent: _,
            section: n,
        } => {
            // Constant attributes do not exist - if it's a method, it must be called
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(n.name().clone()),
                n.location().clone(),
            );
        } // Accessed methods must be called
        EvaluableTokens::FunctionCall {
            function: inner,
            args,
        } => {
            let mut ab = AssemblyBuilder::new();
            let (slf, function_id, name) = function_only::compile_evaluable_function_only(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let signature = global_table.get_function_signature(function_id);
            let uses_self = signature.self_type().uses_self();
            let return_into = signature
                .return_type()
                .clone()
                .map(|r| global_table.add_local_variable_unnamed(r, local_variables));

            // Compile self, if it exists (i.e. dynamic function)
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
                // Right is compiled arguments
                v.push(Right(slf));
                v
            } else {
                Vec::with_capacity(args.len())
            };

            // Left is arguments to be evaluated by call_function
            args.iter().for_each(|a| n_args.push(Left(a)));

            let (asm, _) = call_function(
                fid,
                function_id,
                uses_self,
                et.location(),
                &name,
                &n_args,
                return_into.clone(),
                global_table,
                local_variables,
                global_tracker,
            )?;
            ab.other(&asm);
            (ab.finish(), return_into)
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let struct_type_ref = global_table.resolve_to_type_ref(struct_init.name(), None)?;
            let size = global_table.get_size(&struct_type_ref);

            let mut asm = AssemblyBuilder::new();

            let target = // if struct_init.heap_alloc() {
            //     let (c, ref_target) = heap_alloc(t.clone(), global_table, local_variables);
            //     code.other(&c);
            //     ref_target
            // } else {
                global_table.add_local_variable_unnamed(struct_type_ref.clone(), local_variables)
            /* } */;

            let struct_type = global_table.get_type(*struct_type_ref.type_id());
            let attributes = struct_type
                .get_attributes(struct_init.location())
                .map_err(|_| {
                    WErr::n(
                        EvalErrs::TypeCannotBeInitialised(struct_type.name().to_string()),
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

            // Create all attributes in the correct place in struct
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
                asm.other(&into::compile_evaluable_into(
                    fid,
                    val,
                    new_addr,
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
            }

            if *struct_init.heap_alloc() {
                let (c, ref_target) = heap_alloc(struct_type_ref, global_table, local_variables);
                asm.other(&c);
                asm.other(&copy_to_indirect(
                    *target.local_address(),
                    *ref_target.local_address(),
                    size,
                ));
                (asm.finish(), Some(ref_target))
            } else {
                (asm.finish(), Some(target))
            }
        }
        EvaluableTokens::None => (String::new(), None),
    })
}
