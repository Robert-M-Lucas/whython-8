use b_box::b;
use unique_type_id::UniqueTypeId;

use crate::root::builtin::{BuiltinInlineFunction, f_id, InlineFunctionGenerator, t_id};
use crate::root::builtin::types::int::add::{IntAdd, IntAsAdd};
use crate::root::builtin::types::int::comparators::{IntEq, IntGE, IntGT, IntLE, IntLT, IntNE};
use crate::root::builtin::types::int::div::{IntAsDiv, IntDiv};
use crate::root::builtin::types::int::modulo::{IntAsMod, IntMod};
use crate::root::builtin::types::int::mul::{IntAsMul, IntMul};
use crate::root::builtin::types::int::p_add::IntPAdd;
use crate::root::builtin::types::int::p_sub::{IntAsSub, IntPSub};
use crate::root::builtin::types::int::printi::PrintI;
use crate::root::builtin::types::int::sub::IntSub;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{ByteSize, FunctionID, LocalAddress, TypeID};
use crate::root::shared::types::Type;

mod add;
mod sub;
mod p_sub;
mod printi;
mod p_add;
mod mul;
mod div;
mod modulo;
mod comparators;

pub fn register_int(global_table: &mut GlobalDefinitionTable) {
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
    global_table.register_inline_function(&IntAssign);
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
                if *value < 2147483648 {
                    format!("    mov qword {location}, {value}\n")
                }
                else {
                    let full_hex = format!("{:016x}", value);
                    format!("    mov dword {location}, 0x{}
    mov dword {}, 0x{}\n", &full_hex[8..], *location + LocalAddress(4), &full_hex[..8])
                }
            }
        })
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAssign;

impl BuiltinInlineFunction for IntAssign {
    fn id(&self) -> FunctionID {
        f_id(IntAssign::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "assign"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().with_indirection(1)), ("rhs", IntType::id().immediate())],
            None
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            format!(
                "    mov rax, qword {lhs}
    mov rdx, qword {rhs}
    mov qword [rax], rdx\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}