use std::collections::HashMap;

use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_function_signatures::resolve_function_signature;
use crate::root::name_resolver::resolve_type_sizes::{resolve_type_sizes, UnsizedUserType};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_evaluable::{FullNameToken, FullNameTokens};
use crate::root::parser::parse_function::parse_literal::LiteralToken;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::shared::common::{ByteSize, FunctionID, TypeID};
use crate::root::shared::common::{LocalAddress, TypeRef};
use crate::root::shared::types::Type;
use crate::root::unrandom::new_hashmap;
use derive_getters::Getters;
use itertools::Itertools;

/// A Whython-code defined type
#[derive(Getters)]
pub struct UserType {
    id: TypeID,
    name: String,
    size: ByteSize,
    attributes: Vec<(ByteSize, SimpleNameToken, TypeRef)>,
    location: Location,
}

impl UserType {
    pub fn new(
        id: TypeID,
        name: String,
        size: ByteSize,
        attributes: Vec<(ByteSize, SimpleNameToken, TypeRef)>,
        location: Location,
    ) -> UserType {
        UserType {
            id,
            name,
            size,
            attributes,
            location,
        }
    }
}

impl Type for UserType {
    fn id(&self) -> TypeID {
        self.id
    }

    fn size(&self) -> ByteSize {
        self.size
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_attributes(
        &self,
        _: &Location,
    ) -> Result<&[(ByteSize, SimpleNameToken, TypeRef)], WErr> {
        Ok(&self.attributes)
    }

    fn instantiate_from_literal(
        &self,
        _location: &LocalAddress,
        literal: &LiteralToken,
    ) -> Result<String, WErr> {
        WErr::ne(
            EvalErrs::TypeCannotBeInitialisedByLiteral(self.name().clone()),
            literal.location().clone(),
        )
    }
}

// ! Unoptimised
/// Converts parsed tokens into a collection of functions to be compiled and a `GlobalDefinitionTable`
/// with function signatures and type definitions
pub fn resolve_names(
    ast: Vec<TopLevelTokens>,
    global_table: &mut GlobalDefinitionTable,
) -> Result<HashMap<FunctionID, FunctionToken>, WErr> {
    let mut ast = ast;

    // ? User types > 1; Builtin Types < -1
    // let mut type_names: HashMap<TypeName, isize> = HashMap::new();
    // ? (Name, (type, id)) - Type 0 means global
    // let mut function_names: HashMap<String, (isize, isize)> = HashMap::new();

    // * Type names

    for symbol in &mut ast {
        match symbol {
            TopLevelTokens::Struct(st) => {
                let id = global_table.add_from_struct_token(st);
                st.set_id(id);
            }
            TopLevelTokens::Impl(_) => {}
            TopLevelTokens::Function(_) => {}
        };
    }

    let mut unsized_final_types: HashMap<TypeID, UnsizedUserType> = new_hashmap();
    let mut unprocessed_functions: HashMap<FunctionID, FunctionToken> = new_hashmap();

    for symbol in ast {
        match symbol {
            TopLevelTokens::Struct(st) => {
                let (location, name, attributes, id) = st.dissolve();
                let id = id.unwrap();

                let mut p_attributes: Vec<(SimpleNameToken, TypeRef)> = Vec::new();
                for (name, type_name) in attributes {
                    let type_ref = global_table.resolve_to_type_ref(&type_name)?;

                    for (e_name, _) in &p_attributes {
                        if e_name.name() == name.name() {
                            return WErr::ne(
                                NRErrs::SameAttributeName(name.name().clone()),
                                name.location().clone(),
                            );
                        }
                    }
                    p_attributes.push((name, type_ref))
                }
                unsized_final_types.insert(
                    id,
                    UnsizedUserType::new(id, name.take_name(), p_attributes, location),
                );
            }
            TopLevelTokens::Impl(it) => {
                let (location, name, functions) = it.dissolve();
                let type_id = *global_table
                    .resolve_to_type_ref(
                        &FullNameToken::new(location.clone(), FullNameTokens::Name(name, None))
                            .with_no_indirection(),
                    )?
                    .type_id();

                for ft in functions {
                    let function_id = global_table.add_from_function_token(&ft, Some(type_id));
                    let signature = resolve_function_signature(&ft, global_table)?;
                    global_table.add_function_signature(function_id, signature);
                    unprocessed_functions.insert(function_id, ft);
                }
            }
            TopLevelTokens::Function(ft) => {
                let function_id = global_table.add_from_function_token(&ft, None);
                let signature = resolve_function_signature(&ft, global_table)?;
                global_table.add_function_signature(function_id, signature);
                unprocessed_functions.insert(function_id, ft);
            }
        };
    }
    let mut final_types: HashMap<TypeID, UserType> = new_hashmap();

    while !unsized_final_types.is_empty() {
        let next_type_id = *unsized_final_types.keys().next().unwrap();
        let unsized_type = unsized_final_types.remove(&next_type_id).unwrap();
        if let Err(mut e) = resolve_type_sizes(
            unsized_type,
            &mut final_types,
            &mut unsized_final_types,
            global_table,
        )? {
            let n = e
                .iter()
                .rev()
                .find(|(_, id, _)| *id == e.first().unwrap().1)
                .unwrap();
            e.first_mut().unwrap().0 = n.0.clone();

            return WErr::ne(
                NRErrs::CircularType(
                    e.last().unwrap().0.clone(),
                    e.iter().rev().map(|(s, _, _)| s).join(" -> ").to_string(),
                ),
                e.last().unwrap().2.clone(),
            );
        };
    }

    for (id, user_type) in final_types {
        global_table.add_user_type(id, Box::new(user_type));
    }

    // (final_types, type_names, unprocessed_functions)
    Ok(unprocessed_functions)
}
