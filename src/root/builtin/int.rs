use unique_type_id::UniqueTypeId;
use crate::root::shared::types::Type;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntType {}

impl Type for IntType {
    fn id(&self) -> isize {
        -(IntType::unique_type_id().0 as isize) - 1
    }

    fn size(&self) -> usize {
        8
    }
}