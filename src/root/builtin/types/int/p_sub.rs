use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, f_id, InlineFunctionGenerator};
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntPSub;

impl BuiltinInlineFunction for IntPSub {
    fn id(&self) -> FunctionID {
        f_id(IntPSub::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "p_sub"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().immediate())],
            Some(IntType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> String {
            let lhs = args[0];
            let return_into = return_into.unwrap();
            format!(
"    mov rax, qword {lhs}
    neg rax
    mov qword {return_into}, rax\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAsSub;

impl BuiltinInlineFunction for IntAsSub {
    fn id(&self) -> FunctionID {
        f_id(IntAsSub::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_sub"
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
    sub qword [rax], rdx\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}