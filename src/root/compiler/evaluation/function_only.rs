use crate::root::compiler::evaluation::type_only::compile_evaluable_type_only;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::ExpectedFunctionName;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::FunctionID;

/// Evaluates `name` into a `FunctionID`
pub fn compile_evaluable_function_only<'a>(
    fid: FunctionID,
    name: &'a EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(Option<&'a EvaluableToken>, FunctionID, String), WErr> {
    Ok(match name.token() {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(fid) => (None, fid, name.name().clone()),
                _ => return WErr::ne(ExpectedFunctionName, name.location().clone()),
            }
        }
        EvaluableTokens::StaticAccess {
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
            let function =
                global_table.get_impl_function_by_name(*inner_type.type_id(), access.name());
            let Some(function) = function else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveMethod(
                        global_table.get_type_name(&inner_type.immediate()),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            };

            (None, function, access.name().clone())
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
            let function =
                global_table.get_impl_function_by_name(*inner_type.type_id(), access.name());
            let Some(function) = function else {
                return WErr::ne(
                    EvalErrs::TypeDoesntHaveMethod(
                        global_table.get_type_name(&inner_type.immediate()),
                        access.name().clone(),
                    ),
                    access.location().clone(),
                );
            };

            (Some(inner), function, access.name().clone())
        }
        _ => return WErr::ne(ExpectedFunctionName, name.location().clone()),
    })
}
