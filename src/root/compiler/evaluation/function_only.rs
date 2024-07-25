use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs::ExpectedFunctionName;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{AddressedTypeRef, FunctionID};

/// Evaluates `name` into a `FunctionID`
pub fn compile_evaluable_function_only(
    fid: FunctionID,
    name: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<(Option<AddressedTypeRef>, FunctionID, String), WErr> {
    Ok(match name.token() {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(fid) => {
                    (None, fid, name.name().clone())
                }
                _ => return WErr::ne(ExpectedFunctionName, name.location().clone()),
            }
        }
        _ => return WErr::ne(ExpectedFunctionName, name.location().clone())
    })
}