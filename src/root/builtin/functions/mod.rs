mod exit;
mod printnl;

use crate::root::builtin::functions::exit::ExitFunction;
use crate::root::builtin::functions::printnl::PrintNL;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;

pub fn register_functions(global_table: &mut GlobalDefinitionTable) {
    global_table.register_inline_function(&ExitFunction);
    global_table.register_inline_function(&PrintNL);
}
