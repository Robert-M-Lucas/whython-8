use std::collections::HashMap;
use crate::root::name_resolver::resolve::{AddressedTypeRef, FunctionSignature};
use crate::root::name_resolver::resolve_names::Type;
use crate::root::parser::parse_name::UnresolvedNameToken;

/// Contents of a `DefinitionTable`
enum DefinitionTableEntry {
    Table(Box<DefinitionTable>),
    Function(isize),
    Type(isize),
}

/// Recursive tree containing all named objects/functions/types
struct DefinitionTable {
    table: HashMap<String, DefinitionTableEntry>
}

/// Function-local table of defined variables. Only used within function processing
struct LocalVariableTable {
    table: HashMap<String, AddressedTypeRef>
}

impl LocalVariableTable {
    pub fn new() { todo!(); }

    pub fn get_ref(&self, name: &str) -> Option<AddressedTypeRef> {
        if let Some(r) = self.table.get(name) {
            Some(r.clone())
        }
        else {
            None
        }
    }

    pub fn get_ref_and_type<'a>(&self, name: &str, type_defs: &[&'a HashMap<isize, Box<dyn Type>>]) -> Option<(AddressedTypeRef, &'a dyn Type)> {
        if let Some(r) = self.table.get(name) {
            for map in type_defs {
                if let Some(t) = map.get(r.type_ref().type_id()) {
                    return Some((r.clone(), t.as_ref()));
                }
            }
            panic!("Type in VariableTable but no corresponding definition found!");
        }
        else {
            None
        }
    }
}

struct GlobalDefinitionTable {
    builtin_type_definitions: HashMap<isize, Box<dyn Type>>,
    global_type_definitions: HashMap<isize, Box<dyn Type>>,
    global_function_signatures: HashMap<isize, FunctionSignature>,
    definition_table: DefinitionTable
}


enum NameResult<'a> {
    Function(&'a FunctionSignature),
    Type(&'a dyn Type),
    Variable(AddressedTypeRef, &'a dyn Type)
}

impl GlobalDefinitionTable {
    pub fn new() { todo!(); }

    pub fn resolve_local_name(&self, name: &UnresolvedNameToken, local_variable_table: &LocalVariableTable) -> NameResult {
        let temp_name = &name.names()[0].1;

        if let Some((a, t)) =
            local_variable_table.get_ref_and_type(
                temp_name, &[&self.global_type_definitions, &self.builtin_type_definitions]
            ) {
            return NameResult::Variable(a, t);
        }
        todo!()
    }

    pub fn resolve_global_name(&self, name: &UnresolvedNameToken) -> NameResult {
        todo!()
    }
}
