use std::collections::HashMap;

use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::compiler::assembly::heap::free_function;
use crate::root::compiler::assembly::null::{is_null_function, null_function};
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::location::Location;
use crate::root::parser::parse_function::parse_evaluable::{
    FullNameToken, FullNameTokens, UnresolvedTypeRefToken,
};
use crate::root::parser::parse_function::parse_operator::{OperatorToken, PrefixOrInfixEx};
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_struct::StructToken;
use crate::root::parser::path_storage::FileID;
use crate::root::shared::common::{AddressedTypeRef, ByteSize, FunctionID, TypeID, TypeRef};
use crate::root::shared::types::Type;
use crate::root::unrandom::new_hashmap;
use crate::root::POINTER_SIZE;

#[derive(Debug)]
enum NameTreeEntry {
    Type(TypeID),
    Function(FunctionID),
}

#[derive(Default, Debug)]
/// Table of names and what they correspond to
struct NameTree {
    table: HashMap<String, NameTreeEntry>,
}

impl NameTree {
    pub fn add_entry(&mut self, name: String, entry: NameTreeEntry) {
        self.table.insert(name, entry);
    }

    pub fn get_entry(&self, name: &str) -> Option<&NameTreeEntry> {
        self.table.get(name)
    }
}

/// Top level of tree containing all named objects/functions/types
#[derive(Default)]
struct TopLevelNameTree {
    table: HashMap<FileID, NameTree>,
}

impl TopLevelNameTree {
    pub fn get_tree_mut(&mut self, file_id: FileID) -> &mut NameTree {
        self.table.entry(file_id).or_default();

        self.table.get_mut(&file_id).unwrap()
    }
}

/// Tables containing all global, unchanging definitions from name resolution step
pub struct GlobalDefinitionTable {
    id_counter: isize,
    type_definitions: HashMap<TypeID, Box<dyn Type>>,
    impl_definitions: HashMap<TypeID, HashMap<String, FunctionID>>,
    function_signatures: HashMap<FunctionID, FunctionSignature>,
    inline_functions: HashMap<FunctionID, InlineFunctionGenerator>,
    name_table: TopLevelNameTree,
    builtin_type_name_table: HashMap<String, TypeID>,
    builtin_function_name_table: HashMap<String, FunctionID>,
}

pub enum NameResult {
    Function(FunctionID),
    Type(TypeID),
    Variable(AddressedTypeRef),
}

impl Default for GlobalDefinitionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalDefinitionTable {
    pub fn new() -> GlobalDefinitionTable {
        GlobalDefinitionTable {
            id_counter: 2,
            type_definitions: new_hashmap(),
            impl_definitions: new_hashmap(),
            function_signatures: new_hashmap(),
            inline_functions: Default::default(),
            name_table: Default::default(),
            builtin_type_name_table: Default::default(),
            builtin_function_name_table: Default::default(),
        }
    }

    /// Registers a builtin type
    pub fn register_builtin_type(&mut self, t: Box<dyn Type>) {
        let id = t.id();
        self.builtin_type_name_table
            .insert(t.name().to_string(), id);
        self.type_definitions.insert(id, t);

        let fid = self.id_counter;
        self.id_counter += 1;
        let null_function = null_function(id, FunctionID(fid));
        self.register_inline_function(&null_function);

        let fid = self.id_counter;
        self.id_counter += 1;
        let is_null_function = is_null_function(id, FunctionID(fid));
        self.register_inline_function(&is_null_function);

        let fid = self.id_counter;
        self.id_counter += 1;
        let free_function = free_function(id, FunctionID(fid));
        self.register_inline_function(&free_function);
    }

    // pub fn register_builtin_function(&mut self, name: String, t: FunctionSignature, id: FunctionID) {
    //     self.function_signatures.insert(id, t);
    //     self.builtin_function_name_table.insert(name, id);
    // }

    /// Gets a mutable reference to an impl table for a type
    fn get_impl_mut(&mut self, t: TypeID) -> &mut HashMap<String, FunctionID> {
        self.impl_definitions.entry(t).or_default();

        self.impl_definitions.get_mut(&t).unwrap()
    }

    /// Registers an inline assembly function
    pub fn register_inline_function(&mut self, inline: &dyn BuiltinInlineFunction) {
        self.function_signatures
            .insert(inline.id(), inline.signature());
        self.inline_functions.insert(inline.id(), inline.inline());

        if let Some(parent) = inline.parent_type() {
            self.get_impl_mut(parent)
                .insert(inline.name().to_string(), inline.id());
        } else {
            self.builtin_function_name_table
                .insert(inline.name().to_string(), inline.id());
        }
    }

    /// Gets a function impld for a type by name
    pub fn get_impl_function_by_name(&self, base: TypeID, name: &str) -> Option<FunctionID> {
        self.impl_definitions
            .get(&base)
            .and_then(|i| i.get(name))
            .copied()
    }

    /// Adds a type from a `StructToken`
    ///
    /// `TypeID` returned MUST BE USED to set a type definition
    pub fn add_from_struct_token(&mut self, st: &StructToken) -> TypeID {
        // TODO
        let file_level_tree = self
            .name_table
            .get_tree_mut(st.location().file_id().unwrap());
        self.id_counter += 1;
        let id = TypeID(self.id_counter - 1);

        file_level_tree.add_entry(st.name().name().clone(), NameTreeEntry::Type(id));

        id
    }

    /// Adds a function from a `FunctionToken`
    ///
    /// `FunctionID` returned MUST BE USED to set a function signature
    pub fn add_from_function_token(
        &mut self,
        ft: &FunctionToken,
        containing_class: Option<TypeID>,
    ) -> FunctionID {
        let id = if ft.name().name() == "main" {
            FunctionID(0)
        } else {
            self.id_counter += 1;
            FunctionID(self.id_counter - 1)
        };

        if let Some(containing_class) = containing_class {
            self.impl_definitions.entry(containing_class).or_default();

            self.impl_definitions
                .get_mut(&containing_class)
                .unwrap()
                .insert(ft.name().name().clone(), id);
        } else {
            // TODO
            let file_level_tree = self
                .name_table
                .get_tree_mut(ft.location().file_id().unwrap());
            file_level_tree.add_entry(ft.name().name().clone(), NameTreeEntry::Function(id));
        }

        id
    }

    /// Adds a function signature for a previously given `FunctionID`
    pub fn add_function_signature(
        &mut self,
        given_id: FunctionID,
        function_signature: FunctionSignature,
    ) {
        self.function_signatures
            .insert(given_id, function_signature);
    }

    /// Adds a type definition for a previously given `TypeID`
    pub fn add_user_type(&mut self, given_id: TypeID, definition: Box<dyn Type>) {
        self.type_definitions.insert(given_id, definition);

        let fid = self.id_counter;
        self.id_counter += 1;
        let null_function = null_function(given_id, FunctionID(fid));
        self.register_inline_function(&null_function);

        let fid = self.id_counter;
        self.id_counter += 1;
        let is_null_function = is_null_function(given_id, FunctionID(fid));
        self.register_inline_function(&is_null_function);

        let fid = self.id_counter;
        self.id_counter += 1;
        let free_function = free_function(given_id, FunctionID(fid));
        self.register_inline_function(&free_function);
    }

    /// Takes a name and resolves it to a type (or error)
    pub fn resolve_to_type_ref(&mut self, name: &UnresolvedTypeRefToken) -> Result<TypeRef, WErr> {
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
            _ => WErr::ne(
                NRErrs::ExpectedTypeNotMethodOrAttribute,
                find_error_point(full_name, full_name.location()),
            )?,
        };

        let name = if name.name() == "Self" && containing.is_some() {
            containing.as_ref().unwrap()
        } else {
            name
        };

        let process_tree = |tree: &NameTree| -> Option<_> {
            tree.get_entry(name.name()).map(|val| match val {
                NameTreeEntry::Type(t) => Ok(TypeRef::new(*t, 1, *indirection)),
                NameTreeEntry::Function(_) => WErr::ne(
                    NRErrs::FoundFunctionNotType(name.name().clone()),
                    full_name.location().clone(),
                ),
            })
        };

        // TODO
        if let Some(r) = process_tree(
            self.name_table
                .get_tree_mut(full_name.location().file_id().unwrap()),
        ) {
            return r;
        }

        for (_, tree) in self
            .name_table
            .table
            .iter()
            .filter(|(p, _)| **p != full_name.location().file_id().unwrap())
        {
            if let Some(r) = process_tree(tree) {
                return r;
            }
        }

        if let Some(r) = self.builtin_type_name_table.get(name.name()) {
            return Ok(TypeRef::new(*r, 1, *indirection));
        }

        if let Some(_fid) = self.builtin_function_name_table.get(name.name()) {
            return WErr::ne(
                NRErrs::FoundFunctionNotType(name.name().clone()),
                full_name.location().clone(),
            );
        }

        WErr::ne(
            NRErrs::TypeNotFound(name.name().clone()),
            full_name.location().clone(),
        )
    }

    /// Gets the size of a given type
    pub fn get_size(&mut self, t: &TypeRef) -> ByteSize {
        if t.indirection().has_indirection() {
            POINTER_SIZE
        } else {
            self.type_definitions.get(t.type_id()).unwrap().size()
        }
    }

    /// Adds a local, unnamed variable to the `LocalVariableTable` and returns the address
    pub fn add_local_variable_unnamed_base(
        &mut self,
        t: TypeRef,
        local_variable_table: &mut LocalVariableTable,
    ) -> AddressedTypeRef {
        let size = self.get_size(&t);
        let address = local_variable_table.add_new_unnamed(size);
        AddressedTypeRef::new(address, t)
    }

    /// Adds a local, unnamed variable to the `LocalVariableTable` and returns the address
    pub fn add_local_variable_unnamed(
        &mut self,
        t: &UnresolvedTypeRefToken,
        local_variable_table: &mut LocalVariableTable,
    ) -> Result<AddressedTypeRef, WErr> {
        let t = self.resolve_to_type_ref(t)?;
        Ok(self.add_local_variable_unnamed_base(t, local_variable_table))
    }

    /// Adds a local, named variable to the `LocalVariableTable` and returns the address
    pub fn add_local_variable_named(
        &mut self,
        name: String,
        t: &UnresolvedTypeRefToken,
        local_variable_table: &mut LocalVariableTable,
    ) -> Result<AddressedTypeRef, WErr> {
        let t = self.resolve_to_type_ref(t)?;
        let size = self.get_size(&t);
        let address = local_variable_table.add_new_unnamed(size);
        let address = AddressedTypeRef::new(address, t);
        local_variable_table.add_existing(name, address.clone());
        Ok(address)
    }

    /// Returns whether a main function has been defined
    pub fn has_main(&self) -> bool {
        self.function_signatures.contains_key(&FunctionID(0))
    }

    /// Returns a `Type` specified by the `TypeID`. Panics if it does not exist
    pub fn get_type(&self, type_id: TypeID) -> &dyn Type {
        (*self.type_definitions.get(&type_id).as_ref().unwrap()).as_ref()
    }

    /// Returns a `Type` specified by the `TypeID`, if it exists
    pub fn try_get_type(&self, type_id: TypeID) -> Option<&dyn Type> {
        self.type_definitions
            .get(&type_id)
            .as_ref()
            .map(|x| (*x).as_ref())
    }

    /// Converts a `TypeRef` to a user-readable format
    pub fn get_type_name(&self, type_ref: &TypeRef) -> String {
        format!(
            "{}{}",
            unsafe { String::from_utf8_unchecked(vec![b'&'; type_ref.indirection().0]) },
            self.get_type(*type_ref.type_id()).name()
        )
    }

    /// Returns what a name resolves to
    pub fn resolve_name(
        &mut self,
        name: &SimpleNameToken,
        _containing_class: Option<&SimpleNameToken>,
        local_variable_table: &LocalVariableTable,
    ) -> Result<NameResult, WErr> {
        if let Some(variable) = local_variable_table.get(name.name()) {
            return Ok(NameResult::Variable(variable));
        }

        let process_tree = |tree: &NameTree| -> Option<_> {
            tree.get_entry(name.name()).map(|val| match val {
                NameTreeEntry::Type(t) => Ok(NameResult::Type(*t)),
                NameTreeEntry::Function(f) => Ok(NameResult::Function(*f)),
            })
        };

        // TODO
        if let Some(r) = process_tree(
            self.name_table
                .get_tree_mut(name.location().file_id().unwrap()),
        ) {
            return r;
        }

        // TODO
        for (_, tree) in self
            .name_table
            .table
            .iter()
            .filter(|(p, _)| **p != name.location().file_id().unwrap())
        {
            if let Some(r) = process_tree(tree) {
                return r;
            }
        }

        if let Some(r) = self.builtin_type_name_table.get(name.name()) {
            return Ok(NameResult::Type(*r));
        }

        if let Some(r) = self.builtin_function_name_table.get(name.name()) {
            return Ok(NameResult::Function(*r));
        }

        WErr::ne(
            NRErrs::CannotFindName(name.name().clone()),
            name.location().clone(),
        )
    }

    /// Gets the corresponding function for an operator
    pub fn get_operator_function(
        &self,
        lhs: TypeID,
        operator: &OperatorToken,
        kind: PrefixOrInfixEx,
    ) -> Result<FunctionID, WErr> {
        let op_name = operator.operator().get_method_name(kind);

        if let Some(op_name) = op_name {
            self.impl_definitions
                .get(&lhs)
                .and_then(|f| f.get(&op_name))
                .ok_or(WErr::n(
                    NRErrs::OpMethodNotImplemented(
                        op_name.to_string(),
                        self.get_type(lhs).name().to_string(),
                        operator.operator().to_str().to_string(),
                    ),
                    operator.location().clone(),
                ))
                .copied()
        } else {
            WErr::ne(
                match kind {
                    PrefixOrInfixEx::Prefix => {
                        NRErrs::OpCantBePrefix(operator.operator().to_str().to_string())
                    }
                    PrefixOrInfixEx::Infix => {
                        NRErrs::OpCantBeInfix(operator.operator().to_str().to_string())
                    }
                },
                operator.location().clone(),
            )
        }
    }

    /// Returns the `FunctionSignature` of a function
    pub fn get_function_signature(&self, function_id: FunctionID) -> &FunctionSignature {
        self.function_signatures.get(&function_id).as_ref().unwrap()
    }

    /// Returns a function specified by the `FunctionID`
    pub fn get_function(
        &self,
        function: FunctionID,
    ) -> (&FunctionSignature, Option<&InlineFunctionGenerator>) {
        let signature = self.get_function_signature(function);
        let inline = self.inline_functions.get(&function);
        (signature, inline)
    }
}
