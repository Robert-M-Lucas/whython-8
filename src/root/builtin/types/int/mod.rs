use b_box::b;
use unique_type_id::UniqueTypeId;

use crate::root::builtin::t_id;
use crate::root::builtin::types::int::add::{IntAdd, IntAsAdd};
use crate::root::builtin::types::int::comparators::{IntEq, IntGE, IntGT, IntLE, IntLT, IntNE};
use crate::root::builtin::types::int::div::{IntAsDiv, IntDiv};
use crate::root::builtin::types::int::modulo::{IntAsMod, IntMod};
use crate::root::builtin::types::int::mul::{IntAsMul, IntMul};
use crate::root::builtin::types::int::p_add::IntPAdd;
use crate::root::builtin::types::int::p_sub::{IntAsSub, IntPSub};
use crate::root::builtin::types::int::printi::PrintI;
use crate::root::builtin::types::int::sub::IntSub;
use crate::root::compiler::assembly::utils::write_64bit_int;
use crate::root::compiler::compiler_errors::CompErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};
use crate::root::shared::types::Type;

mod add;
mod comparators;
mod div;
mod modulo;
mod mul;
mod p_add;
mod p_sub;
mod printi;
mod sub;

// fn int_op_sig() -> FunctionSignature {
//     FunctionSignature::new_inline_builtin(
//         true,
//         &[("lhs", BoolType::id().immediate()), ("rhs", BoolType::id().immediate())],
//         Some(BoolType::id().immediate())
//     )
// }

pub fn register_int(global_table: &mut GlobalTable) {
    global_table.register_builtin_type(b!(IntType));
    global_table.register_inline_function(&IntAdd);
    global_table.register_inline_function(&IntAsAdd);
    global_table.register_inline_function(&IntPAdd);
    global_table.register_inline_function(&IntSub);
    global_table.register_inline_function(&IntAsSub);
    global_table.register_inline_function(&IntPSub);
    global_table.register_inline_function(&IntMul);
    global_table.register_inline_function(&IntAsMul);
    global_table.register_inline_function(&IntDiv);
    global_table.register_inline_function(&IntAsDiv);
    global_table.register_inline_function(&IntMod);
    global_table.register_inline_function(&IntAsMod);
    global_table.register_inline_function(&IntEq);
    global_table.register_inline_function(&IntNE);
    global_table.register_inline_function(&IntGT);
    global_table.register_inline_function(&IntLT);
    global_table.register_inline_function(&IntGE);
    global_table.register_inline_function(&IntLE);
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
    fn id(&self) -> TypeID {
        Self::id()
    }

    fn size(&self) -> ByteSize {
        ByteSize(8)
    }

    fn name(&self) -> &str {
        "int"
    }

    fn instantiate_from_literal(
        &self,
        location: &LocalAddress,
        literal: &LiteralToken,
    ) -> Result<String, WErr> {
        Ok(match literal.literal() {
            LiteralTokens::Bool(value) => {
                if *value {
                    format!("    mov qword {location}, 0\n")
                } else {
                    format!("    mov qword {location}, 1\n")
                }
            }
            LiteralTokens::Int(value) => {
                if *value > i64::MAX as i128 {
                    return WErr::ne(
                        CompErrs::IntLiteralExceedsMax(*value, i64::MAX as i128),
                        literal.location().clone(),
                    );
                }
                if *value < i64::MIN as i128 {
                    return WErr::ne(
                        CompErrs::IntLiteralBelowMin(*value, i64::MAX as i128),
                        literal.location().clone(),
                    );
                }

                let value = *value as i64;

                write_64bit_int(value, location)
            }
        })
    }
}
