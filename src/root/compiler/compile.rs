use std::collections::{HashMap, HashSet};
use crate::root::compiler::compile_function::compile_function;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::shared::common::FunctionID;

pub fn compile(mut global_table: GlobalDefinitionTable, unprocessed_functions: HashMap<FunctionID, FunctionToken>) -> Result<String, WErr> {
    let mut unprocessed_functions = unprocessed_functions;
    let mut compiled_functions = HashMap::new();
    let mut compiled_len = 0usize;
    let mut open_set = HashSet::new();
    open_set.insert(FunctionID(0)); // Start with main

    while !open_set.is_empty() {
        let current_function = *open_set.iter().next().unwrap();
        open_set.remove(&current_function);

        let current_function_token = unprocessed_functions.remove(&current_function).unwrap();

        let (compiled, called_functions) = compile_function(current_function, current_function_token, &mut global_table)?;
        compiled_len += compiled.len() + 10;
        compiled_functions.insert(current_function, compiled);

        for called in called_functions {
            if !compiled_functions.contains_key(&called) {
                open_set.insert(called);
            }
        }
    }

    let mut s = String::with_capacity(compiled_len);

    s +=
"    global main

    section .text

";

    for (_id, f) in compiled_functions {
        s += &f;
    }

    Ok(s)
}