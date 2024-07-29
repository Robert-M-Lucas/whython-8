use derive_getters::{Dissolve, Getters};
use derive_more::{Add, AddAssign, Display, Sub, SubAssign};
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone)]
#[display(fmt = "TypeID: {}", .0)]
/// A unique type ID. Negative is builtin, positive is user-defined
pub struct TypeID(pub isize);

impl TypeID {
    pub fn with_indirection(self, indirection: usize) -> TypeRef {
        TypeRef::new(self, Indirection(indirection))
    }

    pub fn immediate(self) -> TypeRef {
        TypeRef::new(self, Indirection(0))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone)]
#[display(fmt = "FunctionID: {}", .0)]
/// A unique function ID. Negative is builtin, 0 is main, and positive is user-defined
pub struct FunctionID(pub isize);

impl FunctionID {
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

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone, Add, AddAssign, Sub, SubAssign)]
#[display(fmt = "Indirection: {}", .0)]
/// The indirection to an address i.e. how many pointers you have to go through
pub struct Indirection(pub usize);

impl Indirection {
    pub fn has_indirection(&self) -> bool {
        self.0 != 0
    }
}

#[derive(
    Debug, PartialEq, Eq, Hash, Display, Copy, Clone, Default, Add, AddAssign, Sub, SubAssign,
)]
#[display(fmt = "ByteSize: {}", .0)]
/// The size of something, in bytes
pub struct ByteSize(pub usize);

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
    indirection: Indirection,
}

impl TypeRef {
    pub fn new(type_id: TypeID, indirection: Indirection) -> TypeRef {
        TypeRef {
            type_id: type_id,
            indirection,
        }
    }

    pub fn plus_one_indirect(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            indirection: Indirection(self.indirection.0 + 1),
        }
    }

    pub fn minus_one_indirect(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            indirection: Indirection(self.indirection.0 - 1),
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
