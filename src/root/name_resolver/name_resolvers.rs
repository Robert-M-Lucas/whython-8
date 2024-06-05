use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use derive_getters::Getters;
use either::{Either, Left, Right};
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::name_resolver_errors::NRErrors::IdentifierNotFound;
use crate::root::errors::WError;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse::Location;
use crate::root::shared::types::Type;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_function::parse_evaluable::{FullNameToken, FullNameTokens, FullNameWithIndirectionToken};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_struct::StructToken;
use crate::root::shared::common::{AddressedTypeRef, FunctionID, TypeID, TypeRef};

#[derive(Debug)]
enum NameTreeEntry {
    Type(TypeID),
    Function(FunctionID)
}

#[derive(Default, Debug)]
struct NameTree {
    table: HashMap<String, NameTreeEntry>
}

impl NameTree {
    pub fn add_entry(&mut self, name: String, entry: NameTreeEntry) {
        self.table.insert(name, entry);
    }

    pub fn get_entry(&self, name: &str) -> Option<&NameTreeEntry> {
        self.table.get(name)
    }
}

/// Top level of recursive tree containing all named objects/functions/types
#[derive(Default)]
struct TopLevelNameTree {
    table: HashMap<Rc<PathBuf>, NameTree>
}

impl TopLevelNameTree {
    pub fn get_tree_mut(&mut self, path: Rc<PathBuf>) -> &mut NameTree {
        if !self.table.contains_key(&path) {
            self.table.insert(path.clone(), Default::default());
        }

        self.table.get_mut(&path).unwrap()
    }
}

#[derive(Getters)]
pub struct GlobalDefinitionTable {
    id_counter: isize,
    type_definitions: HashMap<TypeID, Box<dyn Type>>,
    impl_definitions: HashMap<TypeID, HashMap<String, FunctionID>>,
    function_signatures: HashMap<FunctionID, FunctionSignature>,
    name_table: TopLevelNameTree,
    builtin_type_name_table: HashMap<String, TypeID>,
    builtin_function_name_table: HashMap<String, FunctionID>
}


pub enum NameResult<'a> {
    Function(&'a FunctionSignature),
    Type(&'a dyn Type),
    Variable(AddressedTypeRef, &'a dyn Type),
    NotFound
}

impl GlobalDefinitionTable {
    pub fn new() -> GlobalDefinitionTable {
        GlobalDefinitionTable {
            id_counter: 1,
            type_definitions: Default::default(),
            impl_definitions: Default::default(),
            function_signatures: Default::default(),
            name_table: Default::default(),
            builtin_type_name_table: Default::default(),
            builtin_function_name_table: Default::default(),
        }
    }
    pub fn register_builtin_type(&mut self, name: String, t: Box<dyn Type>) {
        let id = t.id();
        self.type_definitions.insert(id, t);
        self.builtin_type_name_table.insert(name, id);
    }

    pub fn register_builtin_function(&mut self, name: String, t: FunctionSignature, id: FunctionID) {
        self.function_signatures.insert(id, t);
        self.builtin_function_name_table.insert(name, id);
    }

    pub fn add_from_struct_token(&mut self, st: &StructToken) -> TypeID {
        let file_level_tree = self.name_table.get_tree_mut(st.location().path().clone());
        self.id_counter += 1;
        let id = TypeID(self.id_counter - 1);

        file_level_tree.add_entry(st.name().name().clone(), NameTreeEntry::Type(id));

        id
    }

    pub fn add_from_function_token(&mut self, ft: &FunctionToken, containing_class: Option<TypeID>) -> FunctionID {
        let id = if ft.name().name() == "main" {
            FunctionID(0)
        } else {
            self.id_counter += 1;
            FunctionID(self.id_counter - 1)
        };


        if let Some(containing_class) = containing_class {
            if !self.impl_definitions.contains_key(&containing_class) {
                self.impl_definitions.insert(containing_class, Default::default());
            }

            self.impl_definitions.get_mut(&containing_class).unwrap().insert(ft.name().name().clone(), id);
        }
        else {
            let file_level_tree = self.name_table.get_tree_mut(ft.location().path().clone());
            file_level_tree.add_entry(ft.name().name().clone(), NameTreeEntry::Function(id));
        }

        id
    }

    pub fn add_function_signature(&mut self, given_id: FunctionID, function_signature: FunctionSignature) {
        self.function_signatures.insert(given_id, function_signature);
    }

    pub fn add_type(&mut self, given_id: TypeID, definition: Box<dyn Type>) {
        self.type_definitions.insert(given_id, definition);
    }


    pub fn resolve_to_type_ref(&mut self, name: &FullNameWithIndirectionToken) -> Result<TypeRef, WError> {
        let (indirection, full_name) = (name.indirection(), name.inner());

        fn find_error_point(name: &FullNameToken, prev_location: &Location) -> Location {
            match name.token() {
                FullNameTokens::Name(_, _) => prev_location.clone(),
                FullNameTokens::StaticAccess(n, _) => find_error_point(n, name.location()),
                FullNameTokens::DynamicAccess(n, _) => find_error_point(n, name.location()),
            }
        }

        let (name, containing) = match full_name.token() {
            FullNameTokens::Name(n, c) => (n, c),
            _ => Err(WError::n(NRErrors::ExpectedTypeNotMethodOrAttribute, find_error_point(full_name, full_name.location())))?
        };

        let name = if name.name() == "Self" && containing.is_some() {
            containing.as_ref().unwrap()
        } else { name };

        let process_tree = |tree: &NameTree| -> Option<_> {
            if let Some(val) = tree.get_entry(name.name()) {
                Some(match val {
                    NameTreeEntry::Type(t) => Ok(TypeRef::new(*t, *indirection)),
                    NameTreeEntry::Function(_) => {
                        Err(WError::n(NRErrors::FoundFunctionNotType(name.name().clone()), full_name.location().clone()))
                    }
                })
            }
            else {
                None
            }
        };

        if let Some(r) = process_tree(self.name_table.get_tree_mut(full_name.location().path().clone())) {
            return r;
        }

        for (_, tree) in self.name_table.table.iter().filter(|(p, _)| *p != full_name.location().path()) {
            if let Some(r) = process_tree(tree) {
                return r;
            }
        }

        if let Some(r) = self.builtin_type_name_table.get(name.name()) {
            return Ok(TypeRef::new(*r, *indirection))
        }


        Err(WError::n(NRErrors::TypeNotFound(name.name().clone()), full_name.location().clone()))
    }
}
