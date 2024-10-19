use std::collections::HashMap;

use derive_getters::Dissolve;

use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::name_resolver::resolve_names::UserType;
use crate::root::parser::location::Location;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::shared::common::TypeRef;
use crate::root::shared::common::{ByteSize, TypeID};
use crate::root::POINTER_SIZE;

#[derive(Dissolve)]
/// A user type with TBD size
pub struct UnsizedUserType {
    id: TypeID,
    name: String,
    attributes: Vec<(SimpleNameToken, TypeRef)>,
    location: Location,
}

impl UnsizedUserType {
    pub fn new(
        id: TypeID,
        name: String,
        attributes: Vec<(SimpleNameToken, TypeRef)>,
        location: Location,
    ) -> UnsizedUserType {
        UnsizedUserType {
            id,
            name,
            attributes,
            location,
        }
    }
}

/// Resolves the size of a user type recursively
/// Returns the size of the type, an error or a `Vec<Name, TypeID, Location>` in reverse order
/// representing how the circular definition happened
pub fn resolve_type_sizes(
    unsized_type: UnsizedUserType,
    final_types: &mut HashMap<TypeID, UserType>,
    unsized_types: &mut HashMap<TypeID, UnsizedUserType>,
    global_table: &GlobalTable,
) -> Result<Result<ByteSize, Vec<(String, TypeID, Location)>>, WErr> {
    let (id, name, attributes, location) = unsized_type.dissolve();

    let mut size: ByteSize = ByteSize(0);
    let mut processed_attributes: Vec<(ByteSize, SimpleNameToken, TypeRef)> = Vec::new();

    for (attribute_name, attribute_type) in attributes {
        let offset = size;

        if attribute_type.indirection().has_indirection() {
            // Indirection mean fixed size
            size += POINTER_SIZE;
        } else if let Some(sized_type) = final_types.get(attribute_type.type_id()) {
            // Type already processed
            size += *sized_type.size();
        } else if let Some(sized_type) = global_table.try_get_type(*attribute_type.type_id()) {
            // Built-in type already processed
            size += sized_type.size();
        } else if let Some(unsized_type) = unsized_types.remove(attribute_type.type_id()) {
            // Recurse
            size +=
                match resolve_type_sizes(unsized_type, final_types, unsized_types, global_table)? {
                    Ok(s) => s,
                    Err(mut e) => {
                        e.push((name, id, location));
                        return Ok(Err(e));
                    }
                };
        } else {
            // Type not in unsized_types or type table due to circular definition
            return Ok(Err(vec![
                (
                    String::new(),
                    *attribute_type.type_id(),
                    Location::builtin(),
                ),
                (name, id, location),
            ]));
        }

        processed_attributes.push((offset, attribute_name, attribute_type));
    }

    final_types.insert(
        id,
        UserType::new(id, name, size, processed_attributes, location),
    );

    Ok(Ok(size))
}
