use std::fmt::{Display, Formatter};

use derive_getters::{Dissolve, Getters};
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
/// A unique type ID. Negative is builtin, positive is user-defined
pub struct TypeID(pub isize);

#[allow(dead_code)]
impl TypeID {
    /// Creates a `TypeRef`
    pub fn with_indirection(self, elements: usize, indirection: usize) -> TypeRef {
        TypeRef::new(self, elements, Indirection(indirection))
    }

    /// Creates a `TypeRef` with 1 element
    pub fn with_indirection_single(self, indirection: usize) -> TypeRef {
        TypeRef::new(self, 1, Indirection(indirection))
    }

    /// Creates a `TypeRef` with no indirection
    pub fn immediate(self, elements: usize) -> TypeRef {
        TypeRef::new(self, elements, Indirection(0))
    }

    /// Creates a `TypeRef` with 1 element and no indirection
    pub fn immediate_single(self) -> TypeRef {
        TypeRef::new(self, 1, Indirection(0))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
/// A unique function ID. Negative is builtin, 0 is main, and positive is user-defined
pub struct FunctionID(pub isize);

impl FunctionID {
    pub const MAIN_FUNCTION: FunctionID = FunctionID(0);

    pub fn is_main(&self) -> bool {
        self.0 == 0
    }

    /// Gets an identifier for the function that can be used in assembly
    pub fn string_id(&self) -> String {
        if self.is_main() {
            return "main".to_string();
        }

        let id = self.0;
        if id > 0 {
            format!("_{id}")
        } else {
            format!("__{}", -id)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Add, AddAssign, Sub, SubAssign)]
/// The indirection to an address i.e. how many pointers you have to go through
pub struct Indirection(pub usize);

#[allow(dead_code)]
impl Indirection {
    pub fn has_indirection(&self) -> bool {
        self.0 != 0
    }

    pub fn plus(&self, amount: usize) -> Indirection {
        Indirection(self.0 + amount)
    }

    pub fn minus(&self, amount: usize) -> Indirection {
        Indirection(self.0 - amount)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default, Add, AddAssign, Sub, SubAssign)]
/// The size of something, in bytes
pub struct ByteSize(pub usize);

impl Display for ByteSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Add, AddAssign, Sub, SubAssign)]
/// A stack-frame-relative local address. Like in assembly, negative addresses are in the current
/// frame whereas positive addresses are in a previous one
pub struct LocalAddress(pub isize);

impl Display for LocalAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 >= 0 {
            write!(f, "[rbp+{}]", self.0)
        } else {
            write!(f, "[rbp{}]", self.0)
        }
    }
}

#[derive(Getters, Clone, PartialEq, Debug)]
/// A `TypeID` with `Indirection`
pub struct TypeRef {
    type_id: TypeID,
    elements: usize,
    indirection: Indirection,
}

impl TypeRef {
    pub fn new(type_id: TypeID, elements: usize, indirection: Indirection) -> TypeRef {
        TypeRef {
            type_id,
            elements,
            indirection,
        }
    }

    // /// Returns whether a type is an array
    // #[allow(dead_code)]
    // pub fn is_array(&self) -> bool {
    //     self.elements == 1
    // }

    /// Returns a `TypeRef` with a different indirection
    pub fn with_indirection(&self, indirection: Indirection) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            elements: self.elements,
            indirection,
        }
    }

    /// Returns a `TypeRef` with one more indirection
    pub fn plus_one_indirect(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            elements: self.elements,
            indirection: Indirection(self.indirection.0 + 1),
        }
    }

    /// Returns a `TypeRef` with one less indirection
    pub fn minus_one_indirect(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            elements: self.elements,
            indirection: Indirection(self.indirection.0 - 1),
        }
    }

    /// Returns a `TypeRef` with no indirection
    pub fn immediate(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            elements: self.elements,
            indirection: Indirection(0),
        }
    }
}

#[derive(Getters, Clone, Dissolve, Debug)]
/// A `TypeRef` with a `LocalAddress`
pub struct AddressedTypeRef {
    local_address: LocalAddress,
    type_ref: TypeRef,
}

impl AddressedTypeRef {
    pub fn new(local_address: LocalAddress, type_ref: TypeRef) -> AddressedTypeRef {
        AddressedTypeRef {
            local_address,
            type_ref,
        }
    }
}
