use derive_getters::Getters;
use crate::root::builtin::register_builtin;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_names::resolve_names;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;

#[derive(Getters, Clone)]
pub struct TypeRef {
    type_id: isize,
    indirection: usize
}

impl TypeRef {
    pub fn new(type_id: isize, indirection: usize) -> TypeRef { 
        TypeRef { type_id, indirection }
    }
}

#[derive(Getters, Clone)]
pub struct AddressedTypeRef {
    local_address: isize,
    type_ref: TypeRef
}

impl AddressedTypeRef {
    pub fn new(local_address: isize, type_ref: TypeRef) -> AddressedTypeRef {
        AddressedTypeRef { local_address, type_ref }
    }
}

pub fn resolve(ast: Vec<TopLevelTokens>) -> (GlobalDefinitionTable, Vec<(isize, FunctionToken)>) {
    let mut global_table = GlobalDefinitionTable::new();
    register_builtin(&mut global_table);
    let unprocessed_functions = resolve_names(ast, &mut global_table);

    if !global_table.function_signatures().contains_key(&0) {
        // NO MAIN!
        todo!()
    }

    (global_table, unprocessed_functions)
}