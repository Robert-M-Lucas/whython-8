use b_box::b;
use unique_type_id::UniqueTypeId;
use subtract::IntAssignSubtract;
use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::t_id;
use crate::root::builtin::types::int::addition::{IntAddition, IntAssignAddition};
use crate::root::builtin::types::int::comparators::{IntEqual, IntGreaterThan, IntGreaterThanEqual, IntLessThan, IntLessThanEqual, IntNotEqual};
use crate::root::builtin::types::int::division::{IntAssignDivide, IntDivide};
use crate::root::builtin::types::int::modulo::{IntAssignModulo, IntModulo};
use crate::root::builtin::types::int::multiply::{IntAssignMultiply, IntMultiply};
use crate::root::builtin::types::int::prefix_addition::IntPrefixAddition;
use crate::root::builtin::types::int::prefix_subtract::IntPrefixSubtract;
use crate::root::builtin::types::int::print_int::PrintInt;
use crate::root::builtin::types::int::subtract::IntSubtract;
use crate::root::compiler::assembly::utils::write_64bit_int;
use crate::root::errors::compiler_errors::CompErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};
use crate::root::shared::types::Type;

mod addition;
mod comparators;
mod division;
mod modulo;
mod multiply;
mod prefix_addition;
mod prefix_subtract;
mod print_int;
mod subtract;

// fn int_op_sig() -> FunctionSignature {
//     FunctionSignature::new_inline_builtin(
//         true,
//         &[("lhs", BoolType::id().immediate()), ("rhs", BoolType::id().immediate())],
//         Some(BoolType::id().immediate())
//     )
// }

/// Registers all integer types and functions in the `GlobalTable`
pub fn register_int(global_table: &mut GlobalTable) {
    global_table.register_builtin_type(b!(IntType));
    global_table.register_inline_function(&IntAddition);
    global_table.register_inline_function(&IntAssignAddition);
    global_table.register_inline_function(&IntPrefixAddition);
    global_table.register_inline_function(&IntSubtract);
    global_table.register_inline_function(&IntAssignSubtract);
    global_table.register_inline_function(&IntPrefixSubtract);
    global_table.register_inline_function(&IntMultiply);
    global_table.register_inline_function(&IntAssignMultiply);
    global_table.register_inline_function(&IntDivide);
    global_table.register_inline_function(&IntAssignDivide);
    global_table.register_inline_function(&IntModulo);
    global_table.register_inline_function(&IntAssignModulo);
    global_table.register_inline_function(&IntEqual);
    global_table.register_inline_function(&IntNotEqual);
    global_table.register_inline_function(&IntGreaterThan);
    global_table.register_inline_function(&IntLessThan);
    global_table.register_inline_function(&IntGreaterThanEqual);
    global_table.register_inline_function(&IntLessThanEqual);
    global_table.register_inline_function(&PrintInt);
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
    ) -> Result<Assembly, WErr> {
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
