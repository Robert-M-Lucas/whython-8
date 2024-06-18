use std::collections::HashSet;
use std::fmt::format;
use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::compiler::assembly::utils::{align_16_bytes, align_16_bytes_plus_8};
use crate::root::compiler::compile_evaluable::{compile_evaluable, compile_evaluable_into, compile_evaluable_reference};
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_line::LineTokens;
use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

pub fn compile_function(fid: FunctionID, function: FunctionToken, global_table: &mut GlobalDefinitionTable, global_tracker: &mut GlobalTracker) -> Result<String, WErr> {
    let mut local_variables = LocalVariableTable::new();

    let (_location, _name, return_type, _, parameters, lines) = function.dissolve();

    let return_type = if fid.is_main() { None } else { return_type };

    let return_type = if let Some(t) = return_type {
        Some(global_table.resolve_to_type_ref(&t)?)
    }
    else {
        None
    };

    let mut param_address = LocalAddress(16);

    for (param_name, param_type) in parameters {
        let t = global_table.resolve_to_type_ref(&param_type)?;

        local_variables.add_existing(param_name.name().clone(), AddressedTypeRef::new(param_address, t.clone()));

        param_address += LocalAddress(global_table.get_size(&t).0 as isize);
    }

    let return_variable = return_type.and_then(
        |t| Some(AddressedTypeRef::new(
            param_address,//  - LocalAddress(global_table.type_definitions().get(t.type_id()).unwrap().size().0 as isize),
            t
        ))
    );

    let full_contents = recursively_compile_lines(fid, &lines, &return_variable, &mut local_variables, global_table, global_tracker)?;

    // let stack_size = local_variables.stack_size();


    let mut final_contents = format!(
"{}:
    push rbp
    mov rbp, rsp
{}",
        fid.string_id(),
        full_contents
    );

    // if fid.is_main() {
    //     final_contents += "\n\tleave\n\tret";
    // }

    Ok(final_contents)
}

fn recursively_compile_lines(fid: FunctionID, lines: &[LineTokens], return_variable: &Option<AddressedTypeRef>, local_variables: &mut LocalVariableTable, global_table: &mut GlobalDefinitionTable, global_tracker: &mut GlobalTracker) -> Result<String, WErr> {
    local_variables.enter_block();
    let mut contents = AssemblyBuilder::new();

    let mut last_is_return = false;

    for line in lines {
        last_is_return = false;
        match line {
            LineTokens::Initialisation(it) => {
                let (name, type_name, value) = (it.name(), it.type_name(), it.value());
                let address = global_table.add_local_variable_named(name.name().clone(), type_name, local_variables)?;
                contents.other(&compile_evaluable_into(fid, value, address, local_variables, global_table, global_tracker)?);
            },
            LineTokens::Assignment(_) => todo!(),
            LineTokens::If(if_token) => {
                let condition_addr = global_table.add_local_variable_unnamed_base(BoolType::id().immediate(), local_variables);
                contents.other(&compile_evaluable_into(fid, if_token.if_condition(), condition_addr.clone(), local_variables, global_table, global_tracker)?);
                contents.line(&format!("cmp byte {}, 0", condition_addr.local_address()));

                let end_if_tag = global_tracker.get_unique_tag(fid);
                contents.line(&format!("jz {end_if_tag}"));
                contents.other(&recursively_compile_lines(fid, if_token.if_contents(), &None, local_variables, global_table, global_tracker)?);

                if let Some(else_contents) = if_token.else_contents() {
                    let end_else_tag = global_tracker.get_unique_tag(fid);
                    contents.line(&format!("jmp {end_else_tag}"));
                    contents.line(&format!("{end_if_tag}:"));
                    contents.other(&recursively_compile_lines(fid, else_contents, &None, local_variables, global_table, global_tracker)?);
                    contents.line(&format!("{end_else_tag}:"));
                }
                else {
                    contents.line(&format!("{end_if_tag}:"));
                }

            },
            LineTokens::While(_) => todo!(),
            LineTokens::Return(rt) => {
                last_is_return = true;
                if fid.is_main() {
                    if rt.return_value().is_none() {
                        todo!()
                    }

                    let address = global_table.add_local_variable_unnamed_base(TypeRef::new(IntType::id(), Indirection(0)), local_variables);
                    contents.other(&compile_evaluable_into(fid, rt.return_value().as_ref().unwrap(), address.clone(), local_variables, global_table, global_tracker)?);
                    contents.line(&format!("mov rax, qword {}", address.local_address()));
                }
                else {
                    if let Some(return_value) = rt.return_value() {
                        if return_variable.is_none() {
                            todo!()
                        }

                        contents.other(&compile_evaluable_into(fid, return_value, return_variable.clone().unwrap(), local_variables, global_table, global_tracker)?);
                    }
                    else {
                        if return_variable.is_some() {
                            todo!()
                        }
                    }
                }

                contents.line("leave");
                contents.line("ret");
            }
            LineTokens::Break(_) => todo!(),
            LineTokens::NoOp(et) => {
                contents.other(&compile_evaluable_reference(fid, et, local_variables, global_table, global_tracker)?.0);
            }
        }
    }

    local_variables.leave_block();

    Ok(contents.finish())
}