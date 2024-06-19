use std::collections::HashMap;
use derive_getters::Dissolve;
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::shared::common::TypeRef;
use crate::root::name_resolver::resolve_names::UserType;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::POINTER_SIZE;
use crate::root::shared::common::{ByteSize, TypeID};

#[derive(Dissolve)]
pub struct UnsizedUserType {
    id: TypeID,
    name: String,
    attributes: Vec<(SimpleNameToken, TypeRef)>,
    location: Location
}

impl UnsizedUserType {
    pub fn new(id: TypeID, name: String, attributes: Vec<(SimpleNameToken, TypeRef)>, location: Location) -> UnsizedUserType {
        UnsizedUserType { id, name, attributes, location }
    }
}

pub fn resolve_type_sizes(
    unsized_type: UnsizedUserType,
    final_types: &mut HashMap<TypeID, UserType>,
    unsized_types: &mut HashMap<TypeID, UnsizedUserType>,
    global_table: &GlobalDefinitionTable,
) -> Result<ByteSize, WErr> {
    let (id, name, attributes, location) = unsized_type.dissolve();

    println!("{}: {}", id, &name);

    let mut size: ByteSize = ByteSize(0);
    let mut processed_attributes: Vec<(usize, SimpleNameToken, TypeRef)> = Vec::new();

    for (attribute_name, attribute_type) in attributes {
        let offset = size;

        if attribute_type.indirection().has_indirection() {
            size += POINTER_SIZE;
        }
        else if let Some(sized_type) = final_types.get(attribute_type.type_id()) {
            size += *sized_type.size();
        }
        else if let Some(sized_type) = global_table.try_get_type(*attribute_type.type_id()) {
            size += sized_type.size();
        }
        else if let Some(unsized_type) = unsized_types.remove(attribute_type.type_id()) {
            size += resolve_type_sizes(unsized_type, final_types, unsized_types, global_table)?;
        }
        else {
            // Type not in unsized_types or type table due to circular definition
            return Err(WErr::n(NRErrors::CircularType(name), attribute_name.location().clone()));
        }

        processed_attributes.push((offset.0, attribute_name, attribute_type));
    }

    final_types.insert(id, UserType::new(id, name, size, processed_attributes, location));

    Ok(size)
}
