use std::collections::HashMap;
use crate::root::builtin::register_builtin;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_names::resolve_names;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::shared::types::FunctionID;

pub fn resolve(ast: Vec<TopLevelTokens>) -> (GlobalDefinitionTable, HashMap<FunctionID, FunctionToken>) {
    let mut global_table = GlobalDefinitionTable::new();
    register_builtin(&mut global_table);
    let unprocessed_functions = resolve_names(ast, &mut global_table);

    if !global_table.function_signatures().contains_key(&FunctionID(0)) {
        // NO MAIN!
        todo!()
    }

    (global_table, unprocessed_functions)
}