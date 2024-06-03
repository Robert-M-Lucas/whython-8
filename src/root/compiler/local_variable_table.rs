use std::collections::HashMap;
use crate::root::shared::types::{AddressedTypeRef, ByteSize, Type, TypeID};

/// Function-local table of defined variables. Only used within function processing
#[derive(Default)]
pub struct LocalVariableTable {
    outer: Option<Box<LocalVariableTable>>,
    table: HashMap<String, AddressedTypeRef>,
    stack_size: ByteSize
}

impl LocalVariableTable {
    pub fn stack_size(&self) -> ByteSize {
        self.stack_size
    }

    pub fn enter_block(self) -> Box<LocalVariableTable> {
        let stack_size = *(&self.stack_size);
        Box::new(LocalVariableTable {
            outer: Some(Box::new(self)),
            table: Default::default(),
            stack_size
        })
    }

    pub fn leave_block(mut self) -> Box<LocalVariableTable> {
        let stack_size = *(&self.stack_size);
        let mut outer = self.outer.take().unwrap();
        outer.stack_size = stack_size;
        outer
    }

    pub fn add_existing(&mut self, name: String, addressed_type_ref: AddressedTypeRef) {
        self.table.insert(name, addressed_type_ref);
    }

    pub fn get_ref(&self, name: &str) -> Option<AddressedTypeRef> {
        if let Some(r) = self.table.get(name) {
            Some(r.clone())
        }
        else {
            None
        }
    }

    pub fn get_ref_and_type<'a>(&self, name: &str, type_defs: &'a HashMap<TypeID, Box<dyn Type>>) -> Option<(AddressedTypeRef, &'a dyn Type)> {
        if let Some(r) = self.table.get(name) {
            if let Some(t) = type_defs.get(r.type_ref().type_id()) {
                return Some((r.clone(), t.as_ref()));
            }
            panic!("Type in VariableTable but no corresponding definition found!");
        }
        else {
            None
        }
    }
}