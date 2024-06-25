use std::collections::HashMap;
use crate::root::shared::common::{AddressedTypeRef, ByteSize, LocalAddress};

/// Function-local table of defined variables. Only used within function processing
pub struct LocalVariableTable {
    table: Vec<HashMap<String, AddressedTypeRef>>,
    stack_size: Vec<ByteSize>
}

impl Default for LocalVariableTable {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalVariableTable {
    pub fn new() -> LocalVariableTable {
        LocalVariableTable {
            table: vec![Default::default()],
            stack_size: vec![ByteSize(0)]
        }
    }

    pub fn stack_size(&self) -> ByteSize {
        *self.stack_size.last().unwrap()
    }

    /// Creates a new variable scope
    pub fn enter_scope(&mut self){
        self.stack_size.push(*self.stack_size.last().unwrap());
        self.table.push(Default::default());
    }

    /// Removes the topmost variable scope
    pub fn leave_scope(&mut self) {
        self.table.pop();
        self.stack_size.pop();
    }

    /// Adds an allocated, named variable to the variable table and stack size
    pub fn add_existing(&mut self, name: String, addressed_type_ref: AddressedTypeRef) {
        self.table.last_mut().unwrap().insert(name, addressed_type_ref);
    }

    /// Adds a variable that can't be referenced to the stack size
    pub fn add_new_unnamed(&mut self, size: ByteSize) -> LocalAddress {
        *self.stack_size.last_mut().unwrap() += size;
        LocalAddress(-(self.stack_size.last().unwrap().0 as isize))
    }

    // pub fn get_ref(&self, name: &str) -> Option<AddressedTypeRef> {
    //     self.table.last().unwrap().get(name).cloned()
    // }

    /// Returns a local variable
    pub fn get(&self, name: &str) -> Option<AddressedTypeRef> {
        for table in self.table.iter().rev() {
            if let Some(v) = table.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}