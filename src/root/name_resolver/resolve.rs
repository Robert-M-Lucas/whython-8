use derive_getters::Getters;
use crate::root::name_resolver::resolve_names::resolve_names;
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

pub struct FunctionSignature {
    id: isize,
    args: Vec<isize>
}

pub fn resolve(ast: Vec<TopLevelTokens>) {
    let (sized_types, type_names, unprocessed_functions) = resolve_names(ast);
}