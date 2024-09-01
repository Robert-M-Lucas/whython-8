// use std::collections::HashMap;
// use either::Either;
// use itertools::Itertools;
// use crate::root::parser::location::Location;
// 
// pub struct GlobalTable {
//     type_templates: HashMap<String, UserTypeTemplate>,
//     cached_types: HashMap<TypeID, >
// }
// 
// #[derive(Eq, PartialEq, Hash)]
// pub enum TypeID {
//     User { id: usize, parameters: Vec<TypeID>, array_count: usize },
//     Builtin { id: usize }
// }
// 
// 
// #[derive(Eq, PartialEq)]
// pub struct TraitID(usize);
// 
// pub struct UserTypeTemplate {
//     location: Location,
//     parameters: Vec<Vec<TraitID>>,
//     attributes: Vec<(String, Either<TypeID, usize>)> // (Name, Either<Type, index of parameter>) 
// }
// 
// impl UserTypeTemplate {
//     pub fn fill<'a>(&'a self, parameters: &'a [TypeID]) -> UserType {
//         UserType {
//             location: &self.location,
//             size: 0,
//             attributes: self.attributes.iter().map(|(name, t)| {
//                 match t {
//                     Either::Left(t) => (0, name.as_str(), t),
//                     Either::Right(i) => (0, name.as_str(), &parameters[*i])
//                 }
//             }).collect_vec()
//         }
//     }
// }
// 
// pub struct UserType<'a> {
//     location: &'a Location,
//     size: usize,
//     attributes: Vec<(usize, &'a str, &'a TypeID)>
// }