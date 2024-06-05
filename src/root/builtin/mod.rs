pub mod int;

use crate::root::builtin::int::IntType;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::shared::types::Type;

pub fn register_builtin(global_table: &mut GlobalDefinitionTable) {
    let types: [(String, Box<dyn Type>); 1] = [
        ("int".to_string(), Box::new(IntType{}))
    ];

    for (n, t) in types {
        global_table.register_builtin_type(n, t);
    }
}
