use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::compiler::compiler_errors::CompErrs;
use crate::root::compiler::evaluation::into::compile_evaluable_into;
use crate::root::compiler::evaluation::reference::compile_evaluable_reference;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_line::LineTokens;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::shared::common::AddressedTypeRef;
use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeRef};
use crate::root::utils::warn;
use color_print::cprintln;

/// Compiles a given function into assembly
pub fn compile_function(
    fid: FunctionID,
    function: FunctionToken,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker,
) -> Result<String, WErr> {
    let mut local_variables = LocalVariableTable::new();

    let (_location, end_location, _name, return_type, _, parameters, lines) = function.dissolve();

    let return_type = if fid.is_main() { None } else { return_type };

    let return_type = if let Some(t) = return_type {
        Some(global_table.resolve_to_type_ref(&t)?)
    } else {
        None
    };

    let mut param_address = LocalAddress(16);

    for (param_name, param_type) in parameters {
        let t = global_table.resolve_to_type_ref(&param_type)?;

        local_variables.add_existing(
            param_name.name().clone(),
            AddressedTypeRef::new(param_address, t.clone()),
        );

        param_address += LocalAddress(global_table.get_size(&t).0 as isize);
    }

    let return_variable = return_type.map(|t| {
        AddressedTypeRef::new(
            param_address, //  - LocalAddress(global_table.type_definitions().get(t.type_id()).unwrap().size().0 as isize),
            t,
        )
    });

    let (mut full_contents, last_return) = recursively_compile_lines(
        fid,
        &lines,
        &return_variable,
        &None,
        &mut local_variables,
        global_table,
        global_tracker,
    )?;

    // let stack_size = local_variables.stack_size();

    if (return_variable.is_some() || fid.is_main()) && !last_return {
        let type_ref = return_variable
            .map(|x| x.type_ref().clone())
            .unwrap_or_else(|| IntType::id().immediate());
        return WErr::ne(
            CompErrs::ExpectedReturn(global_table.get_type_name(&type_ref)),
            end_location,
        );
    }
    if !last_return {
        full_contents += "\nleave\nret";
    }

    let final_contents = format!(
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

/// Recursively compiles lines provided to it e.g. function body, while body, etc. Returns assembly
fn recursively_compile_lines(
    fid: FunctionID,
    lines: &[LineTokens],
    return_variable: &Option<AddressedTypeRef>,
    break_tag: &Option<&str>,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(String, bool), WErr> {
    local_variables.enter_scope();
    let mut contents = AssemblyBuilder::new();

    let mut last_is_return = false;

    for (line_i, line) in lines.iter().enumerate() {
        last_is_return = false;
        match line {
            LineTokens::Initialisation(it) => {
                let (name, type_name, value) = (it.name(), it.type_name(), it.value());
                let address = global_table.add_local_variable_named(
                    name.name().clone(),
                    type_name,
                    local_variables,
                )?;
                contents.other(&compile_evaluable_into(
                    fid,
                    value,
                    address,
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
            }
            LineTokens::If(if_token) => {
                let condition_addr = global_table
                    .add_local_variable_unnamed_base(BoolType::id().immediate(), local_variables);
                contents.other(&compile_evaluable_into(
                    fid,
                    if_token.if_condition(),
                    condition_addr.clone(),
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
                contents.line(&format!("cmp byte {}, 0", condition_addr.local_address()));

                let end_tag = global_tracker.get_unique_tag(fid);
                let mut next_tag = global_tracker.get_unique_tag(fid);

                if !if_token.elif_condition_contents().is_empty()
                    || if_token.else_contents().is_some()
                {
                    contents.line(&format!("jz {next_tag}"));
                } else {
                    contents.line(&format!("jz {end_tag}"));
                }

                let (code, ret) = recursively_compile_lines(
                    fid,
                    if_token.if_contents(),
                    return_variable,
                    break_tag,
                    local_variables,
                    global_table,
                    global_tracker,
                )?;
                last_is_return = ret;
                contents.other(&code);

                for (elif_condition, elif_content) in if_token.elif_condition_contents() {
                    contents.line(&format!("jmp {end_tag}"));
                    contents.line(&format!("{next_tag}:"));
                    next_tag = global_tracker.get_unique_tag(fid);

                    let condition_addr = global_table.add_local_variable_unnamed_base(
                        BoolType::id().immediate(),
                        local_variables,
                    );
                    contents.other(&compile_evaluable_into(
                        fid,
                        elif_condition,
                        condition_addr.clone(),
                        local_variables,
                        global_table,
                        global_tracker,
                    )?);
                    contents.line(&format!("cmp byte {}, 0", condition_addr.local_address()));
                    contents.line(&format!("jz {next_tag}"));
                    let (code, ret) = recursively_compile_lines(
                        fid,
                        elif_content,
                        return_variable,
                        break_tag,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;
                    last_is_return &= ret;
                    contents.other(&code);
                }

                if let Some(else_contents) = if_token.else_contents() {
                    contents.line(&format!("jmp {end_tag}"));
                    contents.line(&format!("{next_tag}:"));
                    next_tag = global_tracker.get_unique_tag(fid);
                    let (code, ret) = recursively_compile_lines(
                        fid,
                        else_contents,
                        return_variable,
                        break_tag,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?;
                    last_is_return &= ret;
                    contents.other(&code);
                }

                contents.line(&format!("{next_tag}:"));
                contents.line(&format!("{end_tag}:"));
            }
            LineTokens::While(while_token) => {
                let start_tag = global_tracker.get_unique_tag(fid);
                let end_tag = global_tracker.get_unique_tag(fid);

                contents.line(&format!("{start_tag}:"));

                let condition_addr = global_table
                    .add_local_variable_unnamed_base(BoolType::id().immediate(), local_variables);
                contents.other(&compile_evaluable_into(
                    fid,
                    while_token.condition(),
                    condition_addr.clone(),
                    local_variables,
                    global_table,
                    global_tracker,
                )?);
                contents.line(&format!("cmp byte {}, 0", condition_addr.local_address()));
                contents.line(&format!("jz {end_tag}"));

                let (code, ret) = recursively_compile_lines(
                    fid,
                    while_token.contents(),
                    return_variable,
                    &Some(&end_tag),
                    local_variables,
                    global_table,
                    global_tracker,
                )?;
                last_is_return = ret;
                contents.other(&code);

                contents.line(&format!("jmp {start_tag}"));
                contents.line(&format!("{end_tag}:"))
            }
            LineTokens::Return(rt) => {
                last_is_return = true;
                if fid.is_main() {
                    if rt.return_value().is_none() {
                        return WErr::ne(
                            CompErrs::ExpectedSomeReturn(
                                global_table.get_type_name(&IntType::id().immediate()),
                            ),
                            rt.location().clone(),
                        );
                    }

                    let address = global_table.add_local_variable_unnamed_base(
                        TypeRef::new(IntType::id(), Indirection(0)),
                        local_variables,
                    );
                    contents.other(&compile_evaluable_into(
                        fid,
                        rt.return_value().as_ref().unwrap(),
                        address.clone(),
                        local_variables,
                        global_table,
                        global_tracker,
                    )?);
                    contents.line(&format!("mov rax, qword {}", address.local_address()));
                } else if let Some(return_value) = rt.return_value() {
                    if return_variable.is_none() {
                        return WErr::ne(
                            CompErrs::ExpectedNoReturn,
                            return_value.location().clone(),
                        );
                    }

                    contents.other(&compile_evaluable_into(
                        fid,
                        return_value,
                        return_variable.clone().unwrap(),
                        local_variables,
                        global_table,
                        global_tracker,
                    )?);
                } else if return_variable.is_some() {
                    return WErr::ne(
                        CompErrs::ExpectedSomeReturn(
                            global_table
                                .get_type_name(return_variable.as_ref().unwrap().type_ref()),
                        ),
                        rt.location().clone(),
                    );
                }

                contents.line("leave");
                contents.line("ret");

                if line_i != lines.len() - 1 {
                    warn(&format!("Return isn't the last instruction in the block. Following lines in block will not be compiled/run.\n{}", rt.location().clone().to_warning()))
                }
                break;
            }
            LineTokens::Break(bt) => {
                if let Some(break_tag) = break_tag {
                    contents.line(&format!("jmp {break_tag}"));
                } else {
                    return WErr::ne(CompErrs::CannotBreak, bt.location().clone());
                }
            }
            LineTokens::NoOp(et) => {
                contents.other(
                    &compile_evaluable_reference(
                        fid,
                        et,
                        local_variables,
                        global_table,
                        global_tracker,
                    )?
                    .0,
                );
            }
            #[cfg(debug_assertions)]
            LineTokens::Marker(value) => {
                cprintln!("\n<s><m!>At Compilation Marker:</> '{}'", value.value());
                contents.line(&format!(";{}", value.value()));
            }
        }
    }

    local_variables.leave_scope();

    Ok((contents.finish(), last_is_return))
}
