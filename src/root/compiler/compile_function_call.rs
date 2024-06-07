use std::collections::HashSet;
use either::Either;
use itertools::Itertools;
use crate::root::compiler::assembly::utils::{align_16_bytes, align_16_bytes_plus_8, copy, get_function_tag};
use crate::root::compiler::compile_evaluable::{compile_evaluable, compile_evaluable_into};
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_evaluable::EvaluableToken;
use crate::root::shared::common::{AddressedTypeRef, ByteSize, FunctionID, LocalAddress};
use crate::root::utils::warn;


// TODO: Cleanup code
pub fn call_function(
    parent_fid: FunctionID,
    fid: FunctionID,
    arguments: &[Either<&EvaluableToken, &AddressedTypeRef>],
    return_address: Option<AddressedTypeRef>,
    global_table: &mut GlobalDefinitionTable,
    local_variables: &mut LocalVariableTable,
    function_calls: &mut HashSet<FunctionID>) -> Result<(String, Option<AddressedTypeRef>), WErr> {
    function_calls.insert(fid);

    warn("Unchecked Function Arguments");

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

        // TODO: Check arg lengths

        let mut args = Vec::new();
        let signature_args = global_table.get_function(fid).0.args().iter().map(|(_, t)| t.clone()).collect_vec();

        for (i, a) in arguments.iter().enumerate() {
            match a {
                Either::Left(eval) => {
                    let into = global_table.add_local_variable_unnamed_base(signature_args[i].clone(), local_variables);
                    let c = compile_evaluable_into(parent_fid, eval, into.clone(), local_variables, global_table, function_calls)?;

                    if i != 0 {
                        code += "\n";
                    }

                    code += &c;
                    args.push(*into.local_address());
                }
                Either::Right(addr) => {
                    // TODO: Check argument
                    args.push(*addr.local_address());
                }
            }
        }

        code += "\n";
        code += &inline_o(&args, return_into.as_ref().map(|x| *x.local_address()));
        Ok((code, return_into))
    }
    else {
        let mut code = String::new();

        // TODO: Check args length
        let mut args = Vec::new();
        let mut size = ByteSize(0);
        let signature_args = global_table.get_function(fid).0.args().iter().map(|(_, t)| t.clone()).collect_vec();

        for (i, a) in arguments.iter().enumerate() {
            match a {
                Either::Left(eval) => {
                    let into = global_table.add_local_variable_unnamed_base(signature_args[i].clone(), local_variables);
                    size += global_table.get_size(into.type_ref());
                    let c = compile_evaluable_into(parent_fid, eval, into.clone(), local_variables, global_table, function_calls)?;
                    if i != 0 {
                        code += "\n";
                    }
                    code += &c;
                    args.push(into);
                }
                Either::Right(addr) => {
                    // TODO: Check argument
                    args.push((*addr).clone());
                }
            }
        }

        // ? Let return value remain after stack up
        let return_addr = if let Some(return_type) = global_table.get_function(fid).0.return_type().clone() {
            let s = global_table.get_size(&return_type);
            size += s;

            // ? Padding
            let diff = align_16_bytes(size) - size;
            local_variables.add_new_unnamed(diff);

            let into = global_table.add_local_variable_unnamed_base(return_type.clone(), local_variables);

            Some(into)
        }
        else {
            // ? Padding
            let diff = align_16_bytes_plus_8(size) - size;
            local_variables.add_new_unnamed(diff);
            None
        };

        // ? Enter block
        local_variables.enter_block();

        // ? Arguments
        for arg in args {
            let into = global_table.add_local_variable_unnamed_base(arg.type_ref().clone(), local_variables);
            code += "\n";
            code += &copy(*arg.local_address(), *into.local_address(), global_table.get_size(into.type_ref()));
        }

        code += &format!("\n    sub rsp, {}", local_variables.stack_size().0);
        code += &format!("\n    call {}", get_function_tag(fid));
        code += &format!("\n    add rsp, {}", local_variables.stack_size().0);

        // ? Leave block (invalidate parameters)
        local_variables.leave_block();

        let return_addr = if let Some(return_address) = return_address {
            if return_addr.is_none() {
                todo!()
            }

            code += "\n";
            code += &copy(*return_addr.as_ref().unwrap().local_address(), *return_address.local_address(), global_table.get_size(return_addr.as_ref().unwrap().type_ref()));
            Some(return_address)
        }
        else {
            return_addr
        };

        Ok((code, return_addr))
    }
}