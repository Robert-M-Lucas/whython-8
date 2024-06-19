mod exit;

use crate::root::builtin::functions::exit::ExitFunction;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;

pub fn register_functions(global_table: &mut GlobalDefinitionTable) {
    global_table.register_inline_function(&ExitFunction);
}