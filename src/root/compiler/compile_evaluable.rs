use std::collections::HashSet;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{FunctionID, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_evaluable(fid: FunctionID, et: &EvaluableToken, target: Option<AddressedTypeRef>, local_variables: &mut LocalVariableTable, global_table: &GlobalDefinitionTable, function_calls: &mut HashSet<FunctionID>) -> (String, Option<AddressedTypeRef>) {
    let et = et.token();

    match et {
        EvaluableTokens::Name(_) => todo!(),
        EvaluableTokens::Literal(literal) => {
            let (address, t, tid) = if let Some(target) = target {
                let (address, tid) = target.dissolve();
                if tid.indirection().has_indirection() {
                    todo!()
                }
                let t = global_table.type_definitions().get(tid.type_id()).unwrap();
                (address, t, tid)
            }
            else {
                let tid = literal.literal().default_type();
                if tid.indirection().has_indirection() {
                    todo!()
                }
                let t = global_table.type_definitions().get(tid.type_id()).unwrap();
                let address = local_variables.add_new_unnamed(t.size());
                (address, t, tid)
            };

            (t.instantiate_from_literal(&address, literal), Some(AddressedTypeRef::new(address, tid)))
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!()
    }
}