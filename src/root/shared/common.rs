use std::fmt::{Display, Formatter};
use derive_more::{Add, AddAssign, Display, Sub, SubAssign};
use derive_getters::{Dissolve, Getters};

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone)]
#[display(fmt = "TypeID: {}", .0)]
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
pub struct FunctionID(pub isize);

impl FunctionID {
    pub fn is_main(&self) -> bool {
        self.0 == 0
    }

    pub fn string_id(&self) -> String {
        if self.is_main() {
            return "main".to_string();
        }

        let id = self.0;
        if id > 0 {
            format!("_{id}")
        }
        else {
            format!("__{}", -id)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone)]
#[derive(Add, AddAssign, Sub, SubAssign)]
#[display(fmt = "Indirection: {}", .0)]
pub struct Indirection(pub usize);

impl Indirection {
    pub fn has_indirection(&self) -> bool {
        self.0 != 0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone, Default)]
#[derive(Add, AddAssign, Sub, SubAssign)]
#[display(fmt = "ByteSize: {}", .0)]
pub struct ByteSize(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[derive(Add, AddAssign, Sub, SubAssign)]
pub struct LocalAddress(pub isize);

impl Display for LocalAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 >= 0 {
            write!(f, "[rbp+{}]", self.0)
        }
        else {
            write!(f, "[rbp{}]", self.0)
        }
    }
}

#[derive(Getters, Clone, PartialEq, Debug)]
pub struct TypeRef {
    type_id: TypeID,
    indirection: Indirection
}

impl TypeRef {
    pub fn new(type_id: TypeID, indirection: Indirection) -> TypeRef {
        TypeRef { type_id, indirection }
    }

    pub fn plus_one_indirect(&self) -> TypeRef {
        TypeRef {
            type_id: self.type_id,
            indirection: Indirection(self.indirection.0 + 1)
        }
    }
}

#[derive(Getters, Clone, Dissolve)]
pub struct AddressedTypeRef {
    local_address: LocalAddress,
    type_ref: TypeRef
}

impl AddressedTypeRef {
    pub fn new(local_address: LocalAddress, type_ref: TypeRef) -> AddressedTypeRef {
        AddressedTypeRef { local_address, type_ref }
    }
}
