use std::collections::HashMap;

use derive_getters::Getters;

use crate::root::parser::{parse::Location, parse_function::FunctionToken, parse_toplevel::TopLevelTokens};

#[derive(Getters)]
struct TypeRef {
    type_id: isize,
    indirection: usize
}

impl TypeRef {
    pub fn new(type_id: isize, indirection: usize) -> TypeRef { 
        TypeRef { type_id, indirection }
    }
}

struct UnsizedUserType {
    id: isize,
    attributes: Vec<(String, TypeRef)>,
    location: Location
}

impl UnsizedUserType {
    pub fn new(id: isize, attributes: Vec<TypeRef>, location: Location) -> UnsizedUserType {
        UnsizedUserType { id, attributes, location }
    }
}

struct UserType {
    id: isize,
    size: usize,
    attributes: Vec<(usize, String, TypeRef)>,
    location: Location
}

struct Function {
    id: isize,
    args: Vec<isize>
}

// ! Intentionally unoptimised
pub fn resolve_names(ast: Vec<TopLevelTokens>) {
    // ! User types > 1; Bultin Types < -1
    let mut type_names: HashMap<String, isize> = HashMap::new();
    let mut type_id: isize = 1;
    // ! (Name, (type, id)) - Type 0 means global
    let mut function_names: HashMap<String, (isize, isize)> = HashMap::new();
    let mut function_id: isize = 1;

    // * Type names

    for symbol in &ast {
        match symbol {
            TopLevelTokens::Struct(s) => {
                // TODO: Name collision error
                if type_names.contains_key(s.name()) {
                    panic!();
                }
                type_names.insert(s.name().clone(), type_id);
                type_id += 1;
            },
            TopLevelTokens::Impl(_) => {},
            TopLevelTokens::Function(_) => {}
        };
    }

    let mut unsized_final_types: HashMap<isize, UnsizedUserType> = HashMap::new();
    let mut unprocessed_functions: Vec<(isize, FunctionToken)> = Vec::new();

    for symbol in ast {
        match symbol {
            TopLevelTokens::Struct(s) => {
                let (location, name, attributes) = s.dissolve();
                let id = *type_names.get(&name).unwrap();
                unsized_final_types.insert(id, UnsizedUserType::new(id, attributes, location));
            },
            TopLevelTokens::Impl(i) => {
                // TODO: Errors
                let type_id = *type_names.get(i.name()).unwrap();
                
                for function in i.dissolve().2 {
                    function_names.insert(function.name().clone(), (type_id, function_id));
                    function_id += 1;
                    unprocessed_functions.push((type_id, function));
                }
            },
            TopLevelTokens::Function(f) => {
                function_names.insert(f.name().clone(), (0, function_id));
                function_id += 1;
                unprocessed_functions.push((0, f));
            }
        };
    }

    let mut final_types: HashMap<isize, UserType> = HashMap::new();
}

fn resolve_types(id: isize, unsized_type: UnsizedUserType, final_types: &mut HashMap<isize, UserType>, path: &mut Vec<isize>) {
    if path.contains(&id) {
        // TODO: Circular type def
        panic!()
    }

}