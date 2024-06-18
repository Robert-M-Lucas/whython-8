use std::collections::HashSet;
use derive_getters::{Dissolve, Getters};
use crate::root::shared::common::FunctionID;

#[derive(Default, Dissolve, Getters)]
pub struct GlobalTracker {
    function_calls: HashSet<FunctionID>,
    readonly_contents: HashSet<String>,
    readonly_data_section: String,
    unique_tag_counter: usize
}

impl GlobalTracker {
    pub fn functions_mut(&mut self) -> &mut HashSet<FunctionID> {
        &mut self.function_calls
    }

    pub fn f_call(&mut self, fid: FunctionID) {
        self.function_calls.insert(fid);
    }

    pub fn reset_functions(&mut self) {
        self.function_calls = Default::default();
    }

    pub fn add_readonly_data(&mut self, name: &str, data: &str) {
        if !self.readonly_contents.contains(name) {
            self.readonly_contents.insert(name.to_string());
            self.readonly_data_section += "\n    ";
            self.readonly_data_section += data;
        }
    }

    pub fn get_unique_tag(&mut self, function: FunctionID) -> String {
        let r = format!("{}_{}", function.string_id(), self.unique_tag_counter);
        self.unique_tag_counter += 1;
        r
    }
}