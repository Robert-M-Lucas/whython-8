use std::collections::HashMap;
use derive_getters::Getters;
use itertools::Itertools;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResultId};
use crate::root::name_resolver::resolve::TypeRef;
use crate::root::name_resolver::resolve_function_signatures::resolve_function_signature;
use crate::root::name_resolver::resolve_type_sizes::{resolve_type_sizes, UnsizedUserType};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::UnresolvedNameToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
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

impl Type for UserType {
    fn id(&self) -> isize {
        self.id
    }

    fn size(&self) -> usize {
        self.size
    }
}

// ! Unoptimised
pub fn resolve_names(ast: Vec<TopLevelTokens>, global_table: &mut GlobalDefinitionTable) -> Vec<(isize, FunctionToken)> {
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

    let mut unsized_final_types: HashMap<isize, UnsizedUserType> = HashMap::new();
    let mut unprocessed_functions: Vec<(isize, FunctionToken)> = Vec::new();

    for symbol in ast {
        match symbol {
            TopLevelTokens::Struct(st) => {
                let (location, _, attributes, id) = st.dissolve();
                let id = id.unwrap();
                // TODO: Process indirection
                let attributes = attributes.into_iter()
                    .map(|(name, type_name)| {
                        // TODO
                        let type_ref = match global_table.resolve_global_name_to_id(&type_name).unwrap() {
                            NameResultId::Function(_) => todo!(),
                            NameResultId::Type(type_ref) => type_ref,
                            NameResultId::NotFound => todo!(),
                        };

                        (name, type_ref)
                    }
                ).collect_vec();
                unsized_final_types.insert(id, UnsizedUserType::new(id, attributes, location));
            }
            TopLevelTokens::Impl(it) => {
                // TODO
                let type_ref = match global_table.resolve_global_name_to_id(&UnresolvedNameToken::new_unresolved_top(it.name().clone(), it.location().clone())).unwrap() {
                    NameResultId::Function(_) => todo!(),
                    NameResultId::Type(type_ref) => type_ref,
                    NameResultId::NotFound => todo!(),
                };

                // TODO
                if *type_ref.indirection() != 0 {
                    panic!()
                }

                for ft in it.dissolve().2 {
                    let function_id = global_table.add_from_function_token(&ft, Some(*type_ref.type_id()));
                    global_table.add_function_signature(function_id, resolve_function_signature(&ft, &global_table));
                    unprocessed_functions.push((function_id, ft));
                }
            }
            TopLevelTokens::Function(ft) => {
                let function_id = global_table.add_from_function_token(&ft, None);
                global_table.add_function_signature(function_id, resolve_function_signature(&ft, &global_table));
                unprocessed_functions.push((function_id, ft));
            }
        };
    }

    let mut final_types: HashMap<isize, UserType> = HashMap::new();

    while !unsized_final_types.is_empty() {
        let next_type_id = *unsized_final_types.keys().next().unwrap();
        let unsized_type = unsized_final_types.remove(&next_type_id).unwrap();
        resolve_type_sizes(unsized_type, &mut final_types, &mut unsized_final_types, global_table, &mut Vec::new());
    }

    for (id, user_type) in final_types {
        global_table.add_type(id, Box::new(user_type));
    }

    // (final_types, type_names, unprocessed_functions)
    unprocessed_functions
}
