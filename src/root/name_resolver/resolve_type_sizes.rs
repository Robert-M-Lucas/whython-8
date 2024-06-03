use std::collections::HashMap;
use derive_getters::{Dissolve, Getters};
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve::TypeRef;
use crate::root::name_resolver::resolve_names::UserType;
use crate::root::parser::parse::Location;
use crate::root::POINTER_SIZE;

#[derive(Dissolve)]
pub struct UnsizedUserType {
    id: isize,
    attributes: Vec<(String, TypeRef)>,
    location: Location
}

impl UnsizedUserType {
    pub fn new(id: isize, attributes: Vec<(String, TypeRef)>, location: Location) -> UnsizedUserType {
        UnsizedUserType { id, attributes, location }
    }
}

pub fn resolve_type_sizes(
    unsized_type: UnsizedUserType,
    final_types: &mut HashMap<isize, UserType>,
    unsized_types: &mut HashMap<isize, UnsizedUserType>,
    global_table: &GlobalDefinitionTable,
    path: &mut Vec<isize>
) -> usize {
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
            if let Some(unsized_type) = unsized_types.remove(&attribute_type.type_id()) {
                size += resolve_type_sizes(unsized_type, final_types, unsized_types, global_table, path);
            }
            else {
                if let Some(builtin_type) = global_table.type_definitions().get(&attribute_type.type_id()) {
                    size += builtin_type.size();
                }
                else {
                    todo!()
                }
            }
        }

        processed_attributes.push((offset, attribute_name, attribute_type));
    }

    final_types.insert(id, UserType::new(id, size, processed_attributes, location));

    size
}
