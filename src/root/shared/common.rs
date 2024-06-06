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

#[derive(Debug, PartialEq, Eq, Hash, Display, Copy, Clone)]
#[derive(Add, AddAssign, Sub, SubAssign)]
#[display(fmt = "LocalAddress: {}", .0)]
pub struct LocalAddress(pub isize);

#[derive(Getters, Clone, PartialEq)]
pub struct TypeRef {
    type_id: TypeID,
    indirection: Indirection
}

impl TypeRef {
    pub fn new(type_id: TypeID, indirection: Indirection) -> TypeRef {
        TypeRef { type_id, indirection }
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
