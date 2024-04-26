use std::collections::HashMap;
use derive_getters::{Dissolve, Getters};
use crate::root::name_resolver::resolve_function_contents::resolve_function_contents;
use crate::root::name_resolver::resolve_names::{resolve_names, UserType};
use crate::root::parser::parse_name::NameToken;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;

#[derive(Getters)]
pub struct TypeRef {
    type_id: isize,
    indirection: usize
}

impl TypeRef {
    pub fn new(type_id: isize, indirection: usize) -> TypeRef { 
        TypeRef { type_id, indirection }
    }
}


struct Function {
    id: isize,
    args: Vec<isize>
}

pub fn resolve(ast: Vec<TopLevelTokens>) {
    let (sized_types, type_names, unprocessed_functions) = resolve_names(ast);

    resolve_function_contents(sized_types, type_names, unprocessed_functions);
}