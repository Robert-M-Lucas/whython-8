use crate::root::compiler::evaluation::function_only;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::ExpectedReference;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};

/// Helper function for correctly handling a `NameResult` when trying to get a `TypeID`
fn handle_name_result(name: &SimpleNameToken, name_result: NameResult) -> Result<TypeRef, WErr> {
    Ok(match name_result {
        NameResult::Function(_) => {
            return WErr::ne(
                EvalErrs::FunctionMustBeCalled(name.name().clone()),
                name.location().clone(),
            )
        }
        NameResult::Type(t) => {
            t.immediate_single()
            // println!("> {}", name.name());
            // std::process::exit(123);
            // return WErr::ne(
            //     EvalErrs::CannotEvalStandaloneType(name.name().clone()),
            //     name.location().clone(),
            // )
        }
        NameResult::Variable(address) => address.type_ref().clone(),
        NameResult::File(_) => {
            return WErr::ne(
                EvalErrs::ExpectedTypeNotImportedFile(name.name().clone()),
                name.location().clone(),
            )
        }
    })
}

/// Evaluates the type `et` evaluates to. Does not generate any assembly.
pub fn compile_evaluable_type_only(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalTable,
    global_tracker: &mut GlobalTracker,
) -> Result<TypeRef, WErr> {
    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => handle_name_result(
            name,
            global_table.resolve_name(
                name,
                None,
                containing_class.as_ref(),
                local_variables,
                global_tracker,
            )?,
        )?,
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            // TODO: Don't use 0 here
            TypeRef::new(tid, 0, Indirection(0))
        }
        EvaluableTokens::InfixOperator(lhs, op, _) => {
            // if op.is_prefix_opt_t() {
            //     return Err(WErr::n(EvalErrs::FoundPrefixNotInfixOp(op.operator().to_str().to_string()), op.location().clone()));
            // }

            // let (mut code, lhs) = compile_evaluable(fid, lhs, local_variables, global_table, global_tracker)?;
            let lhs_type = compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            // code += "\n";
            let operator_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Infix,
            )?;
            let operator_fn_signature = global_table.get_function_signature(operator_fn);
            operator_fn_signature
                .return_type()
                .as_ref()
                .unwrap()
                .clone()
        }
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = compile_evaluable_type_only(
                fid,
                lhs,
                local_variables,
                global_table,
                global_tracker,
            )?;

            // Handle special cases for referencing and dereferencing
            match op.operator() {
                OperatorTokens::Reference => return Ok(lhs_type.plus_one_indirect()),
                OperatorTokens::Multiply => {
                    if !lhs_type.indirection().has_indirection() {
                        return WErr::ne(
                            ExpectedReference(global_table.get_type_name(&lhs_type)),
                            lhs.location().clone(),
                        );
                    }
                    return Ok(lhs_type.minus_one_indirect());
                }
                _ => {}
            };

            // code += "\n";
            let operator_fn = global_table.get_operator_function(
                *lhs_type.type_id(),
                op,
                PrefixOrInfixEx::Prefix,
            )?;
            let operator_fn_signature = global_table.get_function_signature(operator_fn);
            operator_fn_signature
                .return_type()
                .as_ref()
                .unwrap()
                .clone()
        }
        EvaluableTokens::DynamicAccess {
            parent: inner,
            section: access,
        } => {
            let inner_type = compile_evaluable_type_only(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let new_type = global_table.get_type(*inner_type.type_id());
            let inner_attributes = new_type.get_attributes(access.location())?;

            let mut type_found = None;

            // Find type of attribute
            for (_, name, t) in inner_attributes {
                if name.name() == access.name() {
                    type_found = Some(t.clone());
                    break;
                }
            }

            if let Some(type_found) = type_found {
                type_found.plus_one_indirect()
            } else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveAttribute(
                        global_table.get_type_name(&new_type.id().immediate_single()),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            }
        }
        EvaluableTokens::StaticAccess {
            parent: inner,
            section: access,
        } => {
            // Handle reference to other file
            match inner.token() {
                EvaluableTokens::Name(file_name, containing_class) => {
                    // Imported file
                    if let Some(file) = global_table.get_imported_file(file_name, global_tracker) {
                        return handle_name_result(
                            access,
                            global_table.resolve_name(
                                access,
                                Some(file),
                                containing_class.as_ref(),
                                local_variables,
                                global_tracker,
                            )?,
                        );
                    };
                }
                EvaluableTokens::StaticAccess {
                    parent,
                    section: file_name,
                } => {
                    if let EvaluableTokens::Name(folder_name, containing_class) = parent.token() {
                        // Imported folder
                        if let Some(file) = global_table.get_file_from_folder(
                            folder_name.name(),
                            file_name.name(),
                            global_tracker,
                        ) {
                            return handle_name_result(
                                access,
                                global_table.resolve_name(
                                    access,
                                    Some(file),
                                    containing_class.as_ref(),
                                    local_variables,
                                    global_tracker,
                                )?,
                            );
                        }
                    }
                }
                _ => (),
            }

            // If not referring to a type in another file, the only other possibility
            // is a Type::attribute but constant attributes do not exist
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(access.name().clone()),
                access.location().clone(),
            );
        }
        EvaluableTokens::FunctionCall {
            function: inner,
            args: _args,
        } => {
            let (_slf, function_id, _) = function_only::compile_evaluable_function_only(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;

            let signature = global_table.get_function_signature(function_id);
            let Some(return_type) = signature.return_type().clone() else {
                return WErr::ne(EvalErrs::ExpectedNotNone, et.location().clone());
            };
            return_type
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let mut struct_type = global_table.resolve_to_type_ref(struct_init.name(), None)?;
            if *struct_init.heap_alloc() {
                struct_type = struct_type.plus_one_indirect();
            }
            struct_type
        }
        EvaluableTokens::None => {
            return WErr::ne(EvalErrs::ExpectedNotNone, et.location().clone());
        }
    })
}
