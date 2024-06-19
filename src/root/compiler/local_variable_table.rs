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

    pub fn enter_block(&mut self){
        self.stack_size.push(*self.stack_size.last().unwrap());
        self.table.push(Default::default());
    }

    pub fn leave_block(&mut self) {
        self.table.pop();
        self.stack_size.pop();
    }

    pub fn add_existing(&mut self, name: String, addressed_type_ref: AddressedTypeRef) {
        self.table.last_mut().unwrap().insert(name, addressed_type_ref);
    }

    pub fn add_new_unnamed(&mut self, size: ByteSize) -> LocalAddress {
        *self.stack_size.last_mut().unwrap() += size;
        LocalAddress(-(self.stack_size.last().unwrap().0 as isize))
    }

    pub fn get_ref(&self, name: &str) -> Option<AddressedTypeRef> {
        self.table.last().unwrap().get(name).cloned()
    }

    pub fn get_name(&self, name: &str) -> Option<AddressedTypeRef> {
        for table in self.table.iter().rev() {
            if let Some(v) = table.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}