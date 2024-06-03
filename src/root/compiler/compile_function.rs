use std::collections::HashSet;
use crate::root::compiler::assembly::utils::get_function_tag;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResultId};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_line::LineTokens;
use crate::root::parser::parse_function::parse_operator::get_priority;
use crate::root::parser::parse_name::UnresolvedNameToken;
use crate::root::parser::parse_parameters::Parameters;
use crate::root::shared::types::{AddressedTypeRef, FunctionID, LocalAddress, TypeID};

pub fn compile_function(fid: FunctionID, function: FunctionToken, global_table: &GlobalDefinitionTable) -> (String, HashSet<FunctionID>) {
    let mut local_variables = Box::new(LocalVariableTable::default());

    let (_location, _name, return_type, parameters, lines) = function.dissolve();

    let return_type = return_type.and_then(
        |t| Some(match global_table.resolve_global_name_to_id(&t).unwrap().unwrap() {
            NameResultId::Function(_) => todo!(),
            NameResultId::Type(type_ref) => {
                if type_ref.indirection().has_indirection() {
                    todo!()
                }
                *type_ref.type_id()
            }
            NameResultId::NotFound => todo!()
        }));

    let mut param_address = LocalAddress(-8);

    for (param_name, param_type) in parameters {
        let type_ref = match global_table.resolve_global_name_to_id(&param_type).unwrap().unwrap() {
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
        param_address -= LocalAddress(size.0 as isize);

        local_variables.add_existing(param_name, AddressedTypeRef::new(param_address, type_ref));
    }

    let mut function_calls = HashSet::new();
    let (full_contents, local_variables) = recursively_compile_lines(fid, &lines, return_type, local_variables, global_table, &mut function_calls);

    let stack_size = local_variables.stack_size();

    let final_contents = format!(
"{}:
    push rbp
    mov  rbp, rsp
    {}",
        get_function_tag(fid),
        full_contents
    );

    (final_contents, function_calls)
}

fn recursively_compile_lines(fid: FunctionID, lines: &[LineTokens], return_type: Option<TypeID>, local_variables: Box<LocalVariableTable>, global_table: &GlobalDefinitionTable, function_calls: &mut HashSet<FunctionID>) -> (String, Box<LocalVariableTable>) {
    let local_variables = local_variables.enter_block();
    let mut contents = String::new();


    (contents, local_variables.leave_block())
}