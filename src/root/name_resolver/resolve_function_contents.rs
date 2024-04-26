use std::collections::HashMap;
use crate::root::name_resolver::resolve::TypeRef;
use crate::root::name_resolver::resolve_names::{TypeName, UserType};
use crate::root::parser::parse_function::FunctionToken;

struct Function {

}

pub fn resolve_function_contents(types: HashMap<isize, UserType>, type_names: HashMap<TypeName, isize>, functions: Vec<(isize, FunctionToken)>) -> Vec<Function> {
    // 1st pass - Convert let a =

    todo!()
}