mod printb;
mod and;

use b_box::b;
use unique_type_id::UniqueTypeId;
use crate::root::builtin::t_id;
use crate::root::builtin::types::bool::printb::PrintB;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};
use crate::root::shared::types::Type;

pub fn register_bool(global_table: &mut GlobalDefinitionTable) {
    global_table.register_builtin_type(b!(BoolType));
    global_table.register_inline_function(&PrintB);
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolType;

impl BoolType {
    pub const fn id() -> TypeID {
        t_id(BoolType::unique_type_id().0)
    }
}

impl Type for BoolType {
    fn id(&self) -> TypeID { Self::id() }

    fn size(&self) -> ByteSize {
        ByteSize(1)
    }

    fn name(&self) -> &str {
        "bool"
    }

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> Result<String, WErr> {
        Ok(match literal.literal() {
            LiteralTokens::Bool(value) => {
                if *value {
                    format!("    mov byte {location}, 1\n")
                }
                else {
                    format!("    mov byte {location}, 0\n")
                }
            }
            LiteralTokens::Int(value) => {
                if *value == 0 {
                    format!("    mov byte {location}, 0\n")
                }
                else {
                    format!("    mov byte {location}, 1\n")
                }
            }
        })
    }
}
