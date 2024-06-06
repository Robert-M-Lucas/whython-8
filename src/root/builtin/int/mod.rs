mod add;

use b_box::b;
use unique_type_id::UniqueTypeId;
use crate::root::builtin::int::add::IntAdd;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{AddressedTypeRef, ByteSize, FunctionID, LocalAddress, TypeID};
use crate::root::shared::types::Type;

pub fn register_int(global_table: &mut GlobalDefinitionTable) {
    global_table.register_builtin_type(b!(IntType{}));
    global_table.register_inline_function(&IntAdd{});
}

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

    fn name(&self) -> &str {
        "int"
    }

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> Result<String, WErr> {
        Ok(match literal.literal() {
            LiteralTokens::Bool(value) => {
                if *value {
                    format!("    mov qword {location}, 0")
                }
                else {
                    format!("    mov qword {location}, 1")
                }
            }
            LiteralTokens::Int(value) => {
                format!("    mov qword {location}, {value}")
            }
        })
    }
}