use std::collections::HashSet;
use crate::root::compiler::assembly::utils::{align_16_bytes, align_16_bytes_plus_8, get_function_tag, get_qword_stack_pointer};
use crate::root::compiler::compile_evaluable::compile_evaluable;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_line::LineTokens;
use crate::root::shared::common::{FunctionID, LocalAddress};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_function(fid: FunctionID, function: FunctionToken, global_table: &mut GlobalDefinitionTable) -> Result<(String, HashSet<FunctionID>), WError> {
    let mut local_variables = Box::new(LocalVariableTable::default());

    let (_location, _name, return_type, parameters, lines) = function.dissolve();

    let return_type = if fid.is_main() { None } else { return_type };

    let return_type = if let Some(t) = return_type {
        Some(global_table.resolve_to_type_ref(&t)?)
    }
    else {
        None
    };

    let mut param_address = LocalAddress(8);

    for (param_name, param_type) in parameters {
        let t = global_table.resolve_to_type_ref(&param_type)?;
        global_table.add_local_variable_named(param_name.name().clone(), &param_type, &mut local_variables)?;

        param_address += LocalAddress(global_table.get_size(&t).0 as isize);
    }

    let return_variable = return_type.and_then(
        |t| Some(AddressedTypeRef::new(
            param_address,//  - LocalAddress(global_table.type_definitions().get(t.type_id()).unwrap().size().0 as isize),
            t
        ))
    );

    let mut function_calls = HashSet::new();
    let (full_contents, local_variables) = recursively_compile_lines(fid, &lines, &return_variable, local_variables, global_table, &mut function_calls)?;

    let stack_size = local_variables.stack_size();



    let mut final_contents = format!(
"{}:
    push rbp
    mov  rbp, rsp
    sub  rsp, {}
    {}",
        get_function_tag(fid),
        if fid.is_main() { align_16_bytes(stack_size) } else { align_16_bytes_plus_8(stack_size) },
        full_contents
    );

    if fid.is_main() {
        final_contents += "\n\tleave\n\tret"
    }

    Ok((final_contents, function_calls))
}

fn recursively_compile_lines(fid: FunctionID, lines: &[LineTokens], return_variable: &Option<AddressedTypeRef>, local_variables: Box<LocalVariableTable>, global_table: &mut GlobalDefinitionTable, function_calls: &mut HashSet<FunctionID>) -> Result<(String, Box<LocalVariableTable>), WError> {
    let mut local_variables = local_variables.enter_block();
    let mut contents = String::new();

    for line in lines {
        match line {
            LineTokens::Initialisation(it) => {
                let (name, type_name, value) = (it.name(), it.type_name(), it.value());
                let address = global_table.add_local_variable_named(name.name().clone(), type_name, &mut local_variables)?;
                compile_evaluable(fid, value, Some(address), &mut local_variables, global_table, function_calls);
            },
            LineTokens::Assignment(_) => todo!(),
            LineTokens::If(_) => todo!(),
            LineTokens::While(_) => todo!(),
            LineTokens::Return(rt) => {
                if fid.is_main() {
                    let (code, location) = compile_evaluable(fid, rt.return_value(), None, &mut local_variables, global_table, function_calls);
                    let location = location.unwrap();
                    contents += "\n";
                    contents += &code;
                    contents += &format!("\n\tmov rax, {}", get_qword_stack_pointer(location.local_address()));
                }
                else {
                    todo!()
                }
            }
            LineTokens::Break(_) => todo!(),
            LineTokens::NoOp(et) => {
                contents += "\n";
                contents += &compile_evaluable(fid, et, None, &mut local_variables, global_table, function_calls).0;
            }
        }
    }

    Ok((contents, local_variables.leave_block()))
}