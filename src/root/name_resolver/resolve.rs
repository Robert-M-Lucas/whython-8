use std::collections::HashMap;
use crate::root::builtin::register_builtin;
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_names::resolve_names;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::shared::common::FunctionID;

pub fn resolve(ast: Vec<TopLevelTokens>) -> Result<(GlobalDefinitionTable, HashMap<FunctionID, FunctionToken>), WError> {
    let mut global_table = GlobalDefinitionTable::new();
    register_builtin(&mut global_table);
    let unprocessed_functions = resolve_names(ast, &mut global_table)?;

    if !global_table.has_main() {
        return Err(WError::locationless(NRErrors::NoMain))
    }

    Ok((global_table, unprocessed_functions))
}