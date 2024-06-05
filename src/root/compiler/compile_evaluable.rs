use std::any::Any;
use std::collections::HashSet;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{FunctionID, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_evaluable(fid: FunctionID, et: &EvaluableToken, target: Option<AddressedTypeRef>, local_variables: &mut LocalVariableTable, global_table: &mut GlobalDefinitionTable, function_calls: &mut HashSet<FunctionID>) -> (String, Option<AddressedTypeRef>) {
    let et = et.token();

    match et {
        EvaluableTokens::Name(_, _) => todo!(),
        EvaluableTokens::Literal(literal) => {
            let (address, t) = if let Some(target) = target {
                if target.type_ref().indirection().has_indirection() {
                    todo!()
                }
                let t = global_table.get_type(*target.type_ref().type_id());
                (target, t)
            }
            else {
                let tid = literal.literal().default_type();
                let address = global_table.add_local_variable_unnamed_base(tid.clone(), local_variables);
                let t = global_table.get_type(*tid.type_id());
                (address, t)
            };

            (t.instantiate_from_literal(address.local_address(), literal), Some(address))
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(),
        EvaluableTokens::StaticAccess(_, _) => todo!(),
        EvaluableTokens::FunctionCall(_, _, _) => todo!()
    }
}