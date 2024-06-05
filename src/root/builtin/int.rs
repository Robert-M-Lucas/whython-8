use unique_type_id::UniqueTypeId;
use crate::root::compiler::assembly::utils::get_qword_stack_pointer;
use crate::root::errors::WError;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{AddressedTypeRef, ByteSize, FunctionID, LocalAddress, TypeID};
use crate::root::shared::types::Type;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntType {}

impl IntType {
    pub const fn id() -> TypeID {
        TypeID(-(IntType::unique_type_id().0 as isize) - 1)
    }
}

impl Type for IntType {
    fn id(&self) -> TypeID { Self::id() }

    fn size(&self) -> ByteSize {
        ByteSize(8)
    }

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> Result<String, WError> {
        let location = get_qword_stack_pointer(location);
        Ok(match literal.literal() {
            LiteralTokens::Bool(value) => {
                if *value {
                    format!("\tmov {location}, 0")
                }
                else {
                    format!("\tmov {location}, 1")
                }
            }
            LiteralTokens::Int(value) => {
                format!("\tmov {location}, {value}")
            }
        })
    }
}