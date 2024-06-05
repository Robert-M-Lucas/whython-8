use std::collections::HashSet;
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvaluableErrors;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_evaluable(
    fid: FunctionID,
    et: &EvaluableToken,
    target: Option<AddressedTypeRef>,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<(String, Option<AddressedTypeRef>), WError> {

    let et = et.token();

    Ok(match et {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name.name(), local_variables)? {
                NameResult::Function(_) => todo!(),
                NameResult::Type(_) => todo!(),
                NameResult::Variable(address) => {
                    if let Some(target) = target {
                        if target.type_ref() != address.type_ref() {
                            todo!()
                        }

                        (copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref())), Some(target))
                    }
                    else {
                        let target = global_table.add_local_variable_unnamed_base(address.type_ref().clone(), local_variables);
                        (copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref())), Some(target))
                    }
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let (address, t) = if let Some(target) = target {
                if target.type_ref().indirection().has_indirection() {
                    return Err(WError::n(EvaluableErrors::BadIndirection(target.type_ref().indirection().0, 0), literal.location().clone()));
                }
                let t = global_table.get_type(*target.type_ref().type_id());
                (target, t)
            }
            else {
                let tid = literal.literal().default_type();
                let address = global_table.add_local_variable_unnamed_base(TypeRef::new(tid.clone(), Indirection(0)), local_variables);
                let t = global_table.get_type(tid);
                (address, t)
            };

            (t.instantiate_from_literal(address.local_address(), literal), Some(address))
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(),
        EvaluableTokens::StaticAccess(_, _) => todo!(),
        EvaluableTokens::FunctionCall(_, _, _) => todo!()
    })
}