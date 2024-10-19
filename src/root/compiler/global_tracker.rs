use crate::root::parser::path_storage::PathStorage;
use crate::root::shared::common::FunctionID;
use derive_getters::{Dissolve, Getters};
use std::collections::HashSet;
use crate::root::assembler::assembly_builder::Assembly;

/// Tracks data between function compilations, including data about files and folders
#[derive(Dissolve, Getters)]
pub struct GlobalTracker<'a> {
    path_storage: &'a PathStorage,
    function_calls: HashSet<FunctionID>,
    readonly_contents: HashSet<String>,
    readonly_data_section: Assembly,
    unique_tag_counter: usize,
}

impl<'a> GlobalTracker<'a> {
    pub fn new(path_storage: &'a PathStorage) -> GlobalTracker {
        GlobalTracker {
            path_storage,
            function_calls: Default::default(),
            readonly_contents: Default::default(),
            readonly_data_section: "".to_string(),
            unique_tag_counter: 0,
        }
    }

    pub fn functions_mut(&mut self) -> &mut HashSet<FunctionID> {
        &mut self.function_calls
    }

    /// Stores that a function has been called to ensure it gets compiled
    pub fn store_function_call(&mut self, fid: FunctionID) {
        self.function_calls.insert(fid);
    }

    /// Clears the function calls - needed between function compilations
    pub fn reset_functions(&mut self) {
        self.function_calls = Default::default();
    }

    /// Adds readonly data to be appended to the assembly, ensuring data is not stored twice
    pub fn add_readonly_data(&mut self, name: &str, data: &str) {
        if !self.readonly_contents.contains(name) {
            self.readonly_contents.insert(name.to_string());
            self.readonly_data_section += "\n    ";
            self.readonly_data_section += data;
        }
    }

    /// Returns a program-wide unique tag e.g. for jump instructions
    pub fn get_unique_tag(&mut self, function: FunctionID) -> String {
        let r = format!("{}_{}", function.string_id(), self.unique_tag_counter);
        self.unique_tag_counter += 1;
        r
    }
}
