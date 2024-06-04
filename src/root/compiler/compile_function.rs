use std::collections::HashSet;
use crate::root::compiler::assembly::utils::{align_16_bytes, align_16_bytes_plus_8, get_function_tag, get_qword_stack_pointer};
use crate::root::compiler::compile_evaluable::compile_evaluable;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResultId};
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_line::LineTokens;
use crate::root::shared::common::{FunctionID, LocalAddress};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_function(fid: FunctionID, function: FunctionToken, global_table: &GlobalDefinitionTable) -> Result<(String, HashSet<FunctionID>), WError> {
    let mut local_variables = Box::new(LocalVariableTable::default());

    let (_location, _name, return_type, parameters, lines) = function.dissolve();

    let return_type = if fid.is_main() { None } else { return_type };

    let return_type = if let Some((t, loc)) = return_type {
        Some(match global_table.resolve_global_name_to_id(&t, &loc)? {
            NameResultId::Function(_) => todo!(),
            NameResultId::Type(type_ref) => {
                if type_ref.indirection().has_indirection() {
                    todo!()
                }
                type_ref
            }
            NameResultId::NotFound => todo!()
        })
    }
    else {
        None
    };

    let mut param_address = LocalAddress(8);

    for ((param_name, param_name_loc), (param_type, param_type_loc)) in parameters {
        let type_ref = match global_table.resolve_global_name_to_id(&param_type, &param_type_loc)? {
            NameResultId::Function(_) => todo!(),
            NameResultId::Type(type_ref) => {
                if type_ref.indirection().has_indirection() {
                    todo!()
                }
                type_ref
            }
            NameResultId::NotFound => todo!()
        };

        let size = global_table.type_definitions().get(type_ref.type_id()).unwrap().size();
        local_variables.add_existing(param_name, AddressedTypeRef::new(param_address, type_ref));

        param_address += LocalAddress(size.0 as isize);
    }

    let return_variable = return_type.and_then(
        |t| Some(AddressedTypeRef::new(
            param_address,//  - LocalAddress(global_table.type_definitions().get(t.type_id()).unwrap().size().0 as isize),
            t
        ))
    );

    let mut function_calls = HashSet::new();
    let (full_contents, local_variables) = recursively_compile_lines(fid, &lines, &return_variable, local_variables, global_table, &mut function_calls);

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

fn recursively_compile_lines(fid: FunctionID, lines: &[LineTokens], return_variable: &Option<AddressedTypeRef>, local_variables: Box<LocalVariableTable>, global_table: &GlobalDefinitionTable, function_calls: &mut HashSet<FunctionID>) -> (String, Box<LocalVariableTable>) {
    let mut local_variables = local_variables.enter_block();
    let mut contents = String::new();

    for line in lines {
        match line {
            LineTokens::Initialisation(_) => todo!(),
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

    (contents, local_variables.leave_block())
}