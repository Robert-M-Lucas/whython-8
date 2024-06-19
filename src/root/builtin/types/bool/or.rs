use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::bool::printb::PrintB;
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolOr;

impl BuiltinInlineFunction for BoolOr {
    fn id(&self) -> FunctionID {
        f_id(BoolOr::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "or"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", BoolType::id().immediate()), ("rhs", BoolType::id().immediate())],
            Some(BoolType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into, gt, sz| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
"    mov al, byte {lhs}
    or al, byte {rhs}
    mov byte {return_into}, al
")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolAsOr;

impl BuiltinInlineFunction for BoolAsOr {
    fn id(&self) -> FunctionID {
        f_id(BoolAsOr::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_or"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", BoolType::id().with_indirection(1)), ("rhs", BoolType::id().immediate())],
            None
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {
            let lhs = args[0];
            let rhs = args[1];
            format!(
"    mov rdx, qword {lhs}
    mov al, byte [rdx]
    or al, byte {rhs}
    mov byte [rdx], al
")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}