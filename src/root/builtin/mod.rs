pub mod int;

use crate::root::builtin::int::IntType;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, ImplNode};
use crate::root::shared::types::Type;

pub fn register_builtin(global_table: &mut GlobalDefinitionTable) {
    let types: [(String, Box<dyn Type>, ImplNode); 1] = [
        ("int".to_string(), Box::new(IntType{}), ImplNode::default())
    ];

    for (n, t, i) in types {
        global_table.register_builtin_type(n, t, i);
    }
}
