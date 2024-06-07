use std::collections::HashSet;
use either::Either;
use itertools::Itertools;
use crate::root::compiler::assembly::utils::get_function_tag;
use crate::root::compiler::compile_evaluable::{compile_evaluable, compile_evaluable_into};
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_evaluable::EvaluableToken;
use crate::root::shared::common::{AddressedTypeRef, FunctionID, LocalAddress};
use crate::root::utils::warn;

pub fn call_function(
    parent_fid: FunctionID,
    fid: FunctionID,
    arguments: &[Either<&EvaluableToken, &AddressedTypeRef>],
    return_address: Option<AddressedTypeRef>,
    global_table: &mut GlobalDefinitionTable,
    local_variables: &mut LocalVariableTable,
    function_calls: &mut HashSet<FunctionID>) -> Result<(String, Option<AddressedTypeRef>), WErr> {
    function_calls.insert(fid);

    {
        let (signature, inline) = global_table.get_function(fid);

        // TODO: check signature
        warn("Unchecked function signature");
    }

    if let Some(inline) = global_table.get_function(fid).1 {
        let inline_o = inline.clone();
        let mut code = String::new();

        let return_into = if let Some(expected_return) = global_table.get_function(fid).0.return_type().clone() {
            if let Some(return_address) = return_address {
                if return_address.type_ref() != &expected_return {
                    todo!()
                }
                Some(return_address)
            }
            else {
                Some(global_table.add_local_variable_unnamed_base(expected_return.clone(), local_variables))
            }
        }
        else {
            if return_address.is_some() {
                todo!()
            }
            None
        };

        let mut args = Vec::new();

        let signature_args = global_table.get_function(fid).0.args().iter().map(|(_, t)| t.clone()).collect_vec();
        for (i, a) in arguments.iter().enumerate() {
            match a {
                Either::Left(eval) => {
                    let into = global_table.add_local_variable_unnamed_base(signature_args[i].clone(), local_variables);
                    let c = compile_evaluable_into(parent_fid, eval, into.clone(), local_variables, global_table, function_calls)?;
                    code += &c;
                    args.push(*into.local_address());
                }
                Either::Right(addr) => {
                    args.push(*addr.local_address());
                }
            }
        }

        code += &inline_o(&args, return_into.as_ref().map(|x| *x.local_address()));
        Ok((code, return_into))
    }
    else {
        todo!()
    }
}