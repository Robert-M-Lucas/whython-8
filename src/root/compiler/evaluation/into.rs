use either::{Left, Right};
use itertools::Itertools;

use crate::root::assembler::assembly_builder::{Assembly, AssemblyBuilder};
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::heap::heap_alloc;
use crate::root::compiler::assembly::utils::{copy, copy_to_indirect};
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
use crate::root::name_resolver::name_resolvers::{GlobalTable, NameResult};
use crate::root::parser::location::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{AddressedTypeRef, FunctionID, LocalAddress};

/// Evaluates `et` putting the result into `target`
pub fn compile_evaluable_into(
    fid: FunctionID,
    evaluable: &EvaluableToken,
    target: AddressedTypeRef,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalTable,
    global_tracker: &mut GlobalTracker,
) -> Result<Assembly, WErr> {
    let ets = evaluable.token();

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
                    )
                }
                NameResult::Type(_) => {
                    // Name was a type (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvalStandaloneType(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::File(_) => {
                    // Name was a file (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvaluateStandaloneImportedFile(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Variable(address) => {
                    // Check type
                    if address.type_ref() != target.type_ref() {
                        return WErr::ne(
                            EvalErrs::ExpectedDifferentType(
                                global_table.get_type_name(target.type_ref()),
                                global_table.get_type_name(address.type_ref()),
                            ),
                            name.location().clone(),
                        );
                    }
                    
                    // Copy into output
                    copy(
                        *address.local_address(),
                        *target.local_address(),
                        global_table.get_size(target.type_ref()),
                    )
                }
            }
        }
        EvaluableTokens::Literal(literal) => {
            // Check indirection
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
            // Assignment doesn't return correct value
            if op.operator() == &OperatorTokens::Assign {
                return WErr::ne(
                    EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())),
                    evaluable.location().clone(),
                );
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

            // Operator function incorrectly implemented (by user, hopefully)
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
            
            // Check if return type is correct for target
            match operator_fn_signature.return_type() {
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

            // Call operator function on arguments
            let (asm, _) = call_function(
                fid,
                operator_fn,
                operator_fn_signature.self_type().uses_self(),
                evaluable.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Infix)
                    .unwrap(),
                &[Left(lhs), Left(rhs)],
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;

            asm
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
                    asm += &set_reference(op.location(), val, target, global_table)?;
                    return Ok(asm);
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

            let operator_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Prefix,
            )?;
            let operator_fn_signature = global_table.get_function_signature(operator_fn);

            // Check argument count
            if operator_fn_signature.args().len() != 1 {
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
                        operator_fn_signature.args().len(),
                    ),
                    op.location().clone(),
                );
            }

            // Check return type
            match operator_fn_signature.return_type() {
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

            // Call operator function
            let (asm, _) = call_function(
                fid,
                operator_fn,
                operator_fn_signature.self_type().uses_self(),
                evaluable.location(),
                &op.operator()
                    .get_method_name(PrefixOrInfixEx::Prefix)
                    .unwrap(),
                &[Left(lhs)],
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;
            asm
        }
        EvaluableTokens::DynamicAccess {
            parent: inner_eval,
            section: access,
        } => {
            let mut ab = AssemblyBuilder::new();
            // Evaluate what is being accessed
            let (asm, inner) = compile_evaluable_reference(
                fid,
                inner_eval,
                local_variables,
                global_table,
                global_tracker,
            )?;
            ab.other(&asm);

            let Some(inner) = inner else {
                return WErr::ne(EvalErrs::ExpectedNotNone, inner_eval.location().clone());
            };
            
            // Coerce self, if needed
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
            let mut found_offset = None;
            
            // Find the byte offset of the attribute
            for (offset, name, t) in inner_attributes {
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
            
            // Error if not found
            let Some(found_offset) = found_offset else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveAttribute(
                        global_table.get_type_name(&inner_type.id().immediate_single()),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            };
            
            
            if inner.type_ref().indirection().has_indirection() { // If inner is a reference
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
        EvaluableTokens::StaticAccess {
            parent: _,
            section: n,
        } => {
            // Constant attributes do not exist - if it's a method, it must be called
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(n.name().clone()),
                n.location().clone(),
            )
        }
        EvaluableTokens::FunctionCall {
            function: inner,
            args: args,
        } => {
            let mut ab = AssemblyBuilder::new();
            let (slf, function_id, name) = function_only::compile_evaluable_function_only(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;
            let self_type = *global_table.get_function_signature(function_id).self_type();

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
                self_type.uses_self(),
                evaluable.location(),
                &name,
                &n_args,
                Some(target),
                global_table,
                local_variables,
                global_tracker,
            )?;
            ab.other(&asm);
            ab.finish()
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let struct_type_ref = global_table.resolve_to_type_ref(struct_init.name(), None)?;
            let size = global_table.get_size(&struct_type_ref);
            
            if *struct_init.heap_alloc() {
                // Incorrect reference count
                if target.type_ref() != &struct_type_ref.plus_one_indirect() {
                    return WErr::ne(
                        EvalErrs::ExpectedDifferentType(
                            global_table.get_type_name(target.type_ref()),
                            global_table.get_type_name(&struct_type_ref.plus_one_indirect()),
                        ),
                        struct_init.location().clone(),
                    );
                }
                let mut ab = AssemblyBuilder::new();
                let (asm, sr) =
                    compile_evaluable_new(fid, evaluable, local_variables, global_table, global_tracker)?;
                ab.other(&asm);
                let sr = sr.unwrap();
                ab.other(&copy(
                    *sr.local_address(),
                    *target.local_address(),
                    global_table.get_size(target.type_ref()),
                ));
                return Ok(ab.finish());
            }
            debug_assert!(!struct_type_ref.indirection().has_indirection());
            
            // Incorrect reference count
            if *struct_init.heap_alloc() && &struct_type_ref.plus_one_indirect() != target.type_ref() {
                return WErr::ne(
                    EvalErrs::ExpectedDifferentType(
                        global_table.get_type_name(target.type_ref()),
                        global_table.get_type_name(&struct_type_ref.plus_one_indirect()),
                    ),
                    struct_init.location().clone(),
                );
            }
            // Incorrect type
            if !struct_init.heap_alloc() && &struct_type_ref != target.type_ref() {
                return WErr::ne(
                    EvalErrs::ExpectedDifferentType(
                        global_table.get_type_name(target.type_ref()),
                        global_table.get_type_name(&struct_type_ref),
                    ),
                    struct_init.location().clone(),
                );
            }

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
            let given_attributes = struct_init.contents();

            if attributes.len() != given_attributes.len() {
                return WErr::ne(
                    EvalErrs::WrongAttributeCount(attributes.len(), given_attributes.len()),
                    struct_init.location().clone(),
                );
            }

            let mut asm = AssemblyBuilder::new();
            
            // Create all attributes in correct place in struct
            for ((offset, t_name, t_type), (name, val)) in attributes.iter().zip(given_attributes.iter())
            {
                // Incorrect attribute
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
                asm.other(&compile_evaluable_into(
                    fid,
                    val,
                    new_addr,
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
            }
            
            // TODO: Test
            if *struct_init.heap_alloc() {
                let (c, ref_target) = heap_alloc(struct_type_ref, global_table, local_variables);
                asm.other(&c);
                asm.other(&copy_to_indirect(
                    *target.local_address(),
                    *ref_target.local_address(),
                    size,
                ));
            }

            asm.finish()
        }
        EvaluableTokens::None => {
            return WErr::ne(
                EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())),
                evaluable.location().clone(),
            );
        }
    })
}
