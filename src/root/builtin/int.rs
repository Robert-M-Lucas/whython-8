use unique_type_id::UniqueTypeId;
use crate::root::shared::types::{ByteSize, Type, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntType {}

impl Type for IntType {
    fn id(&self) -> TypeID {
        TypeID(-(IntType::unique_type_id().0 as isize) - 1)
    }

    fn size(&self) -> ByteSize {
        ByteSize(8)
    }
}