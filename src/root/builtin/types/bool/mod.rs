mod and;
mod comparators;
mod not;
mod or;
mod printb;

use crate::root::builtin::types::bool::and::{BoolAnd, BoolAsAnd};
use crate::root::builtin::types::bool::comparators::{BoolEq, BoolNE};
use crate::root::builtin::types::bool::not::BoolNot;
use crate::root::builtin::types::bool::or::{BoolAsOr, BoolOr};
use crate::root::builtin::types::bool::printb::PrintB;
use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, t_id, BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{ByteSize, FunctionID, LocalAddress, TypeID};
use crate::root::shared::types::Type;
use b_box::b;
use unique_type_id::UniqueTypeId;

fn bool_op_sig() -> FunctionSignature {
    FunctionSignature::new_inline_builtin(
        SelfType::CopySelf,
        &[
            ("lhs", BoolType::id().immediate()),
            ("rhs", BoolType::id().immediate()),
        ],
        Some(BoolType::id().immediate()),
    )
}

pub fn register_bool(global_table: &mut GlobalDefinitionTable) {
    global_table.register_builtin_type(b!(BoolType));
    global_table.register_inline_function(&PrintB);
    global_table.register_inline_function(&BoolEq);
    global_table.register_inline_function(&BoolNE);
    global_table.register_inline_function(&BoolAnd);
    global_table.register_inline_function(&BoolAsAnd);
    global_table.register_inline_function(&BoolOr);
    global_table.register_inline_function(&BoolAsOr);
    global_table.register_inline_function(&BoolNot);
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
    fn id(&self) -> TypeID {
        Self::id()
    }

    fn size(&self) -> ByteSize {
        ByteSize(1)
    }

    fn name(&self) -> &str {
        "bool"
    }

    fn instantiate_from_literal(
        &self,
        location: &LocalAddress,
        literal: &LiteralToken,
    ) -> Result<String, WErr> {
        Ok(match literal.literal() {
            LiteralTokens::Bool(value) => {
                if *value {
                    format!("    mov byte {location}, 1\n")
                } else {
                    format!("    mov byte {location}, 0\n")
                }
            }
            LiteralTokens::Int(value) => {
                if *value == 0 {
                    format!("    mov byte {location}, 0\n")
                } else {
                    format!("    mov byte {location}, 1\n")
                }
            }
        })
    }
}
