use std::collections::HashMap;

use derive_getters::{Dissolve, Getters};
use itertools::Itertools;

use crate::root::POINTER_SIZE;
use crate::root::parser::parse_name::NameToken;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_toplevel::TopLevelTokens;

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

#[derive(Hash)]
struct TypeName {
    name: String
}

impl TypeName {
    pub fn from_string(name: String) -> TypeName {
        TypeName { name }
    }

    pub fn from_name_token(name: NameToken) -> TypeName {
        TypeName { name: name.dissolve().1 }
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for TypeName {}


#[derive(Dissolve)]
struct UnsizedUserType {
    id: isize,
    attributes: Vec<(String, TypeRef)>,
    location: Location
}

impl UnsizedUserType {
    pub fn new(id: isize, attributes: Vec<(String, TypeRef)>, location: Location) -> UnsizedUserType {
        UnsizedUserType { id, attributes, location }
    }
}

#[derive(Getters)]
struct UserType {
    id: isize,
    size: usize,
    attributes: Vec<(usize, String, TypeRef)>,
    location: Location
}

impl UserType {
    pub fn new(id: isize, size: usize, attributes: Vec<(usize, String, TypeRef)>, location: Location) -> UserType {
        UserType { id, size, attributes, location }
    }
}

struct Function {
    id: isize,
    args: Vec<isize>
}

// ! Intentionally unoptimised
pub fn resolve_names(ast: Vec<TopLevelTokens>) {
    // ! User types > 1; Bultin Types < -1
    let mut type_names: HashMap<TypeName, isize> = HashMap::new();
    let mut type_id: isize = 1;
    // ! (Name, (type, id)) - Type 0 means global
    let mut function_names: HashMap<String, (isize, isize)> = HashMap::new();
    let mut function_id: isize = 1;

    // * Type names

    for symbol in &ast {
        match symbol {
            TopLevelTokens::Struct(s) => {
                let name = TypeName::from_string(s.name().clone());
                // TODO: Name collision error
                if type_names.get(&name).is_none() {
                    todo!();
                }
                type_names.insert(name, type_id);
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
                let id = *type_names.get(&TypeName::from_string(name.clone())).unwrap();
                // TODO: Process indirection
                let attributes = attributes.into_iter()
                    .map(|(name, type_name)| {
                        // TODO: Name error
                        (name, TypeRef::new(*type_names.get(&TypeName::from_name_token(type_name)).unwrap(), 0))
                    }
                ).collect_vec();
                unsized_final_types.insert(id, UnsizedUserType::new(id, attributes, location));
            },
            TopLevelTokens::Impl(i) => {
                // TODO: Errors
                let type_id = *type_names.get(&TypeName::from_string(i.name().clone())).unwrap();
                
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

    while !unsized_final_types.is_empty() {
        let next_type_id = *unsized_final_types.keys().next().unwrap();
        let unsized_type = unsized_final_types.remove(&next_type_id).unwrap();
        resolve_types(unsized_type, &mut final_types, &mut unsized_final_types, &mut Vec::new());
    }
}

fn resolve_types(unsized_type: UnsizedUserType, final_types: &mut HashMap<isize, UserType>, unsized_types: &mut HashMap<isize, UnsizedUserType>, path: &mut Vec<isize>) -> usize {
    let (id, attributes, location) = unsized_type.dissolve();

    if path.contains(&id) {
        // TODO: Circular type def error
        todo!();
    }
    path.push(id);

    let mut size: usize = 0;
    let mut processed_attributes: Vec<(usize, String, TypeRef)> = Vec::new();

    for (attribute_name, attribute_type) in attributes {
        let offset = size;

        if *attribute_type.indirection() != 0 {
            size += POINTER_SIZE;
        }
        else if let Some(sized_type) = final_types.get(&attribute_type.type_id()) {
            size += sized_type.size();
        }
        else {
            let unsized_type = unsized_types.remove(&attribute_type.type_id()).unwrap();
            size += resolve_types(unsized_type, final_types, unsized_types, path);
        }

        processed_attributes.push((offset, attribute_name, attribute_type));
    }

    final_types.insert(id, UserType::new(id, size, processed_attributes, location));

    size
}