use crate::root::builtin::register_builtin;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::name_resolver::resolve_names::resolve_names;
use crate::root::parser::location::Location;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::parser::path_storage::{FileID, PathStorage};
use crate::root::shared::common::FunctionID;
use crate::root::unrandom::new_hashmap;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Converts parsed tokens into a collection of functions to be compiled and a `GlobalDefinitionTable`
/// with function signatures and type definitions
pub fn resolve(
    ast: HashMap<FileID, Vec<TopLevelTokens>>,
    path_storage: &PathStorage,
) -> Result<(GlobalTable, HashMap<FunctionID, FunctionToken>), WErr> {
    let mut global_table = GlobalTable::new();
    register_builtin(&mut global_table);

    let mut ast = ast;
    let mut unprocessed_functions = new_hashmap();
    let mut processed_files = HashSet::new();
    let mut process_order = Vec::new();
    let (file, first) = ast.remove_entry(&FileID::MAIN_FILE).unwrap();

    resolve_file(
        file,
        first,
        &mut ast,
        &mut global_table,
        &mut unprocessed_functions,
        &mut processed_files,
        path_storage,
        &mut process_order,
    )?;

    if !global_table.has_main() {
        return WErr::locationless_e(NRErrs::NoMain);
    }

    Ok((global_table, unprocessed_functions))
}

/// Processes a file handling its imports
fn resolve_file(
    file_id: FileID,
    tokens: Vec<TopLevelTokens>,
    ast: &mut HashMap<FileID, Vec<TopLevelTokens>>,
    global_table: &mut GlobalTable,
    unprocessed_functions: &mut HashMap<FunctionID, FunctionToken>,
    processed_files: &mut HashSet<FileID>,
    path_storage: &PathStorage,
    process_order: &mut Vec<FileID>,
) -> Result<(), WErr> {
    process_order.push(file_id);

    let scope = path_storage.get_file(file_id).scope().clone();

    for (f, l) in scope.files_imported().iter().chain(scope.files_used()) {
        process_if_needed(
            *f,
            l,
            ast,
            global_table,
            unprocessed_functions,
            processed_files,
            path_storage,
            process_order,
        )?;
    }
    for (fld, l) in scope.folders_imported() {
        for f in path_storage.get_folder(*fld).child_files().values() {
            process_if_needed(
                *f,
                l,
                ast,
                global_table,
                unprocessed_functions,
                processed_files,
                path_storage,
                process_order,
            )?;
        }
    }

    global_table.scope_namespace(file_id, scope);
    resolve_names(tokens, global_table, unprocessed_functions)?;

    processed_files.insert(file_id);
    process_order.pop();

    Ok(())
}

/// Processes a file if it hasn't already been processed
fn process_if_needed(
    file_id: FileID,
    location: &Location,
    ast: &mut HashMap<FileID, Vec<TopLevelTokens>>,
    global_table: &mut GlobalTable,
    unprocessed_functions: &mut HashMap<FunctionID, FunctionToken>,
    processed_files: &mut HashSet<FileID>,
    path_storage: &PathStorage,
    process_order: &mut Vec<FileID>,
) -> Result<(), WErr> {
    if processed_files.contains(&file_id) {
        return Ok(());
    };
    if let Some((file_id, tokens)) = ast.remove_entry(&file_id) {
        resolve_file(
            file_id,
            tokens,
            ast,
            global_table,
            unprocessed_functions,
            processed_files,
            path_storage,
            process_order,
        )?;
        Ok(())
    } else {
        // Reconstruct error due to circular import
        process_order.push(file_id);
        let s = process_order
            .iter()
            .map(|f| path_storage.reconstruct_file(*f))
            .join(" -> ");
        WErr::ne(NRErrs::CircularImport(s), location.clone())
    }
}
