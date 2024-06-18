mod add;
mod sub;
mod p_sub;
mod printi;
mod eq;

use b_box::b;
use unique_type_id::UniqueTypeId;
use crate::root::builtin::t_id;
use crate::root::builtin::types::int::add::IntAdd;
use crate::root::builtin::types::int::eq::IntEq;
use crate::root::builtin::types::int::p_sub::IntPSub;
use crate::root::builtin::types::int::printi::PrintI;
use crate::root::builtin::types::int::sub::IntSub;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};
use crate::root::shared::types::Type;

pub fn register_int(global_table: &mut GlobalDefinitionTable) {
    global_table.register_builtin_type(b!(IntType));
    global_table.register_inline_function(&IntAdd);
    global_table.register_inline_function(&IntSub);
    global_table.register_inline_function(&IntPSub);
    global_table.register_inline_function(&IntEq);
    global_table.register_inline_function(&PrintI);
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntType;

impl IntType {
    pub const fn id() -> TypeID {
        t_id(IntType::unique_type_id().0)
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
                    format!("    mov qword {location}, 0\n")
                }
                else {
                    format!("    mov qword {location}, 1\n")
                }
            }
            LiteralTokens::Int(value) => {
                format!("    mov qword {location}, {value}\n")
            }
        })
    }
}
