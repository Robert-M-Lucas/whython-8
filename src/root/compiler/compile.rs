use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(debug_assertions)]
use itertools::Itertools;
use crate::root::assembler::assembly_builder::Assembly;
use crate::root::compiler::compile_function::compile_function;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::path_storage::PathStorage;
use crate::root::shared::common::FunctionID;
use crate::root::unrandom::{new_hashmap, new_hashset};

/// Compiles the entire program. Returns assembly.
pub fn compile(
    mut global_table: GlobalTable,
    unprocessed_functions: HashMap<FunctionID, FunctionToken>,
    path_storage: &PathStorage,
) -> Result<Assembly, WErr> {
    let mut unprocessed_functions = unprocessed_functions;
    // TODO: Write assembly to disk asynchronously while compiling
    let mut compiled_functions = new_hashmap();
    let mut compiled_len = 0usize;
    
    // Functions to compile
    let mut open_set = new_hashset();
    let mut compiled_count: usize = 0;
    
    // Last time progress was shown
    let mut last_shown = Instant::now();

    open_set.insert(FunctionID::MAIN_FUNCTION); // Start with main
    let mut global_tracker = GlobalTracker::new(path_storage);

    while !open_set.is_empty() {
        // Reset state tracked during function compilation
        global_tracker.reset_functions();

        let current_function = *open_set.iter().next().unwrap();
        open_set.remove(&current_function);

        compiled_count += 1;
        
        let Some(current_function_token) = unprocessed_functions.remove(&current_function) else {
            continue; // Inline function
        };

        let file_id = current_function_token.location().file_id().unwrap();
        // Scope the global table to the current file to prevent namespace leaking
        global_table.scope_namespace(
            file_id,
            global_tracker
                .path_storage()
                .get_file(file_id)
                .scope()
                .clone(),
        );

        let compiled = compile_function(
            current_function,
            current_function_token,
            &mut global_table,
            &mut global_tracker,
        )?;

        compiled_len += compiled.len() + 10;
        compiled_functions.insert(current_function, compiled);

        // Add all function called in this functions compilation to the open set if not already compiled
        for called in global_tracker.function_calls() {
            if !compiled_functions.contains_key(called) {
                open_set.insert(*called);
            }
        }

        // Show info if last info shown was long enough ago
        if Instant::now() - last_shown > Duration::from_millis(1000) {
            print!(
                "\n  - {}/{} Functions Compiled",
                compiled_count,
                open_set.len() + compiled_count
            );
            last_shown = Instant::now();
        }
    }
    print!(
        "\n  - {}/{} Functions Compiled",
        compiled_count,
        open_set.len() + compiled_count
    );
    println!();

    let mut asm: Assembly = String::with_capacity(compiled_len);

    asm += "    global main

section .text

";
    // Add all functions
    
    #[cfg(not(debug_assertions))]
    for (_id, f) in compiled_functions {
        asm += &f;
        asm += "\n\n";
    }
    
    
    #[cfg(debug_assertions)]
    for (_id, f) in compiled_functions
        .iter()
        .sorted_by(|(x, _), (y, _)| x.0.cmp(&y.0))
    {
        asm += f;
        asm += "\n\n";
    }

    // Add static data 
    if !global_tracker.readonly_data_section().is_empty() {
        asm += "section .data_readonly";
        asm += global_tracker.readonly_data_section();
    }

    Ok(asm)
}
