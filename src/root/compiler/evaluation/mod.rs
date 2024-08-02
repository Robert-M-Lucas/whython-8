pub mod coerce_self;
pub mod function_only;
pub mod into;
pub mod new;
pub mod reference;
pub mod type_only;

// Error on an empty address
// pub fn expect_addr(
//     r: (String, Option<AddressedTypeRef>),
// ) -> Result<(String, AddressedTypeRef), WErr> {
//     Ok((r.0, r.1.unwrap())) // TODO
// }

// Will ignore everything other than type with a target type
// pub fn compile_evaluable_type_only_into(
//     fid: FunctionID,
//     et: &EvaluableToken,
//     target: TypeRef,
//     local_variables: &mut LocalVariableTable,
//     global_table: &mut GlobalDefinitionTable,
//     global_tracker: &mut GlobalTracker
// ) -> Result<bool, WErr> {
//
//     let et = et.token();
//
//
// }
