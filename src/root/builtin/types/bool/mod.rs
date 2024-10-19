use b_box::b;
use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::t_id;
use crate::root::builtin::types::bool::and::{BoolAnd, BoolAssignAnd};
use crate::root::builtin::types::bool::comparators::{BoolEqual, BoolNotEqual};
use crate::root::builtin::types::bool::not::BoolNot;
use crate::root::builtin::types::bool::or::{BoolAssignOr, BoolOr};
use crate::root::builtin::types::bool::print_bool::PrintBool;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};
use crate::root::shared::types::Type;

mod and;
mod comparators;
mod not;
mod or;
mod print_bool;

/// Helper function for getting the function signature of common boolean operations
/// `lhs`: `bool`
/// `rhs`: `bool`
/// `return_type`: `Some(bool)`
fn boolean_signature() -> FunctionSignature {
    FunctionSignature::new_inline_builtin(
        SelfType::CopySelf,
        &[
            ("lhs", BoolType::id().immediate_single()),
            ("rhs", BoolType::id().immediate_single()),
        ],
        Some(BoolType::id().immediate_single()),
    )
}

/// Registers all boolean types and functions in the `GlobalTable`
pub fn register_bool(global_table: &mut GlobalTable) {
    global_table.register_builtin_type(b!(BoolType));
    global_table.register_inline_function(&PrintBool);
    global_table.register_inline_function(&BoolEqual);
    global_table.register_inline_function(&BoolNotEqual);
    global_table.register_inline_function(&BoolAnd);
    global_table.register_inline_function(&BoolAssignAnd);
    global_table.register_inline_function(&BoolOr);
    global_table.register_inline_function(&BoolAssignOr);
    global_table.register_inline_function(&BoolNot);
}

/// The boolean type `bool`
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
    ) -> Result<Assembly, WErr> {
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
