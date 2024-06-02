use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use derive_getters::Getters;
use crate::root::name_resolver::resolve::AddressedTypeRef;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::types::Type;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::UnresolvedNameToken;
use crate::root::parser::parse_struct::StructToken;

#[derive(Default)]
struct ImplNode {
    functions: HashMap<String, isize>
}

/// Contents of a `DefinitionTable`
enum FileLevelTreeNode {
    Function(isize),
    Type(isize, ImplNode),
}

/// Recursive tree containing all named objects/functions/types
#[derive(Default)]
struct FileLevelTree {
    table: HashMap<String, FileLevelTreeNode>
}

impl FileLevelTree {
    pub fn add_type(&mut self, name: String, id: isize) {
        // TODO: Handle collision
        self.table.insert(name, FileLevelTreeNode::Type(id, ImplNode::default()));
    }

    pub fn add_function_impl(&mut self, name: String, id: isize, containing_class: isize) -> bool {
        for (_, n) in &mut self.table {
            match n {
                FileLevelTreeNode::Function(_) => {}
                FileLevelTreeNode::Type(type_id, i) => {
                    if containing_class != *type_id {
                        continue;
                    }

                    i.functions.insert(name, id);

                    return true;
                }
            }
        }

        return false;
    }

    pub fn add_function(&mut self, name: String, id: isize) {
        self.table.insert(name, FileLevelTreeNode::Function(id));
    }
}

/// Top level of recursive tree containing all named objects/functions/types
#[derive(Default)]
struct TopLevelNameTree {
    table: HashMap<Rc<PathBuf>, Box<FileLevelTree>>
}

impl TopLevelNameTree {
    pub fn get_path_tree(&mut self, path: &Rc<PathBuf>) -> &mut Box<FileLevelTree> {
        // ! Inefficient, done to make borrow checker happy
        if !self.table.contains_key(path) {
            self.table.insert(path.clone(), Box::new(FileLevelTree::default()));
        }

        self.table.get_mut(path).unwrap()
    }
}

/// Function-local table of defined variables. Only used within function processing
#[derive(Default)]
struct LocalVariableTable {
    table: HashMap<String, AddressedTypeRef>,
    stack_size: usize
}

impl LocalVariableTable {

    pub fn get_ref(&self, name: &str) -> Option<AddressedTypeRef> {
        if let Some(r) = self.table.get(name) {
            Some(r.clone())
        }
        else {
            None
        }
    }

    pub fn get_ref_and_type<'a>(&self, name: &str, type_defs: &'a HashMap<isize, Box<dyn Type>>) -> Option<(AddressedTypeRef, &'a dyn Type)> {
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

#[derive(Default, Getters)]
pub struct GlobalDefinitionTable {
    id_counter: isize,
    type_definitions: HashMap<isize, Box<dyn Type>>,
    function_signatures: HashMap<isize, FunctionSignature>,
    name_table: TopLevelNameTree
}


pub enum NameResult<'a> {
    Function(&'a FunctionSignature),
    Type(&'a dyn Type),
    Variable(AddressedTypeRef, &'a dyn Type),
    NotFound
}

pub enum NameResultId {
    Function(isize),
    Type(isize),
    NotFound
}

impl GlobalDefinitionTable {
    pub fn add_from_struct_token(&mut self, st: &StructToken) -> isize {
        let file_level_tree = self.name_table.get_path_tree(st.location().path());
        self.id_counter += 1;
        let id = self.id_counter - 1;

        file_level_tree.add_type(st.name().clone(), id);

        id
    }

    pub fn add_from_function_token(&mut self, ft: &FunctionToken, containing_class: Option<isize>) -> isize {

        self.id_counter += 1;
        let id = self.id_counter - 1;

        if let Some(containing_class) = containing_class {
            for (_, file_level_tree) in &mut self.name_table.table {
                if file_level_tree.add_function_impl(ft.name().clone(), id, containing_class) {
                    return id;
                }
            }
            panic!("Class for impl not found");
        }
        else {
            let file_level_tree = self.name_table.get_path_tree(ft.location().path());
            file_level_tree.add_function(ft.name().clone(), id);
        }

        id
    }

    pub fn add_function_signature(&mut self, given_id: isize, function_signature: FunctionSignature) {
        self.function_signatures.insert(given_id, function_signature);
    }

    pub fn add_type(&mut self, given_id: isize, definition: Box<dyn Type>) {
        // TODO: handle collisions
        self.type_definitions.insert(given_id, definition);
    }

    pub fn resolve_local_name(&self, name: &UnresolvedNameToken, local_variable_table: &LocalVariableTable) -> NameResult {
        let temp_name = &name.names()[0].1;

        if let Some((a, t)) =
            local_variable_table.get_ref_and_type(
                temp_name, &self.type_definitions
            ) {
            return NameResult::Variable(a, t);
        }
        todo!()
    }

    pub fn resolve_local_name_allow_function_calls(&self, name: &UnresolvedNameToken, local_variable_table: &mut LocalVariableTable) -> NameResult {
        let temp_name = &name.names()[0].1;

        if let Some((a, t)) =
            local_variable_table.get_ref_and_type(
                temp_name, &self.type_definitions
            ) {
            return NameResult::Variable(a, t);
        }
        todo!()
    }

    pub fn resolve_global_name(&self, name: &UnresolvedNameToken) -> NameResult {
        todo!()
    }

    pub fn resolve_global_name_to_id(&self, name: &UnresolvedNameToken) -> NameResultId {
        todo!()
    }
}
