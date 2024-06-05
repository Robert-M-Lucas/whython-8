use std::any::Any;
use std::collections::HashMap;

use derive_getters::Getters;
use itertools::Itertools;
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WError;

use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::name_resolver::resolve_function_signatures::resolve_function_signature;
use crate::root::name_resolver::resolve_type_sizes::{resolve_type_sizes, UnsizedUserType};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_evaluable::{FullNameToken, FullNameTokens};
use crate::root::parser::parse_function::parse_literal::LiteralToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::shared::common::{LocalAddress, TypeRef};
use crate::root::shared::common::{ByteSize, FunctionID, TypeID};
use crate::root::shared::types::Type;

// #[derive(Hash)]
// pub struct TypeName {
//     name: String,
//     location: Location,
// }
//
// impl TypeName {
//     pub fn from_struct_token(st: &StructToken) -> TypeName {
//         TypeName {
//             name: st.name().clone(),
//             location: st.location().clone(),
//         }
//     }
//
//     pub fn from_impl_token(it: &ImplToken) -> TypeName {
//         TypeName {
//             name: it.name().clone(),
//             location: it.location().clone(),
//         }
//     }

    // pub fn from_name_token(name: UnresolvedNameToken) -> TypeName {
    //     let name: (Location, Rc<PathBuf>, Option<String>, usize, String, Vec<(NameConnectors, String)>, Option<Vec<EvaluableToken>>) = name.dissolve();
    //     TypeName {
    //         name: name.4,
    //         location: name.0,
    //     }
    // }
// }

// impl PartialEq for TypeName {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }
//
// impl Eq for TypeName {}


/// A whython-code-defined type
#[derive(Getters)]
pub struct UserType {
    id: TypeID,
    size: ByteSize,
    attributes: Vec<(usize, String, TypeRef)>,
    location: Location
}

impl UserType {
    pub fn new(id: TypeID, size: ByteSize, attributes: Vec<(usize, String, TypeRef)>, location: Location) -> UserType {
        UserType { id, size, attributes, location }
    }
}

impl Type for UserType {
    fn id(&self) -> TypeID {
        self.id
    }

    fn size(&self) -> ByteSize {
        self.size
    }

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> String {
        todo!()
    }
}

// ! Unoptimised
pub fn resolve_names(ast: Vec<TopLevelTokens>, global_table: &mut GlobalDefinitionTable) -> Result<HashMap<FunctionID, FunctionToken>, WError> {
    let mut ast = ast;

    // ? User types > 1; Builtin Types < -1
    // let mut type_names: HashMap<TypeName, isize> = HashMap::new();
    // ? (Name, (type, id)) - Type 0 means global
    // let mut function_names: HashMap<String, (isize, isize)> = HashMap::new();

    // * Type names

    for symbol in &mut ast {
        match symbol {
            TopLevelTokens::Struct(st) => {
                let id = global_table.add_from_struct_token(&st);
                st.set_id(id);
            },
            TopLevelTokens::Impl(_) => {},
            TopLevelTokens::Function(_) => {}
        };
    }

    let mut unsized_final_types: HashMap<TypeID, UnsizedUserType> = HashMap::new();
    let mut unprocessed_functions: HashMap<FunctionID, FunctionToken> = HashMap::new();

    for symbol in ast {
        match symbol {
            TopLevelTokens::Struct(st) => {
                let (location, _, attributes, id) = st.dissolve();
                let id = id.unwrap();

                let mut p_attributes = Vec::new();
                for ((name, name_loc), (type_name, type_name_loc)) in attributes {
                    let type_ref = global_table.resolve_global_name_to_id(&type_name)?;

                    for (e_name, _) in &p_attributes {
                        if e_name == &name {
                            return Err(WError::n(NRErrors::SameAttributeName(name), name_loc));
                        }
                    }
                    p_attributes.push((name, type_ref))
                }
                unsized_final_types.insert(id, UnsizedUserType::new(id, p_attributes, location));
            }
            TopLevelTokens::Impl(it) => {
                let (location, name, functions) = it.dissolve();
                let type_id = *global_table.resolve_global_name_to_id(FullNameToken::new(
                    location.clone(),
                    FullNameTokens::Name(name, None)
                ).with_no_indirection())?.type_id();

                for ft in it.dissolve().2 {
                    let function_id = global_table.add_from_function_token(&ft, Some(type_id));
                    global_table.add_function_signature(function_id, resolve_function_signature(&ft, &global_table)?);
                    unprocessed_functions.insert(function_id, ft);
                }
            }
            TopLevelTokens::Function(ft) => {
                let function_id = global_table.add_from_function_token(&ft, None);
                global_table.add_function_signature(function_id, resolve_function_signature(&ft, &global_table)?);
                unprocessed_functions.insert(function_id, ft);
            }
        };
    }

    let mut final_types: HashMap<TypeID, UserType> = HashMap::new();

    while !unsized_final_types.is_empty() {
        let next_type_id = *unsized_final_types.keys().next().unwrap();
        let unsized_type = unsized_final_types.remove(&next_type_id).unwrap();
        resolve_type_sizes(unsized_type, &mut final_types, &mut unsized_final_types, global_table, &mut Vec::new());
    }

    for (id, user_type) in final_types {
        global_table.add_type(id, Box::new(user_type));
    }

    // (final_types, type_names, unprocessed_functions)
    Ok(unprocessed_functions)
}
