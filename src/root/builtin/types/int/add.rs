use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAdd;

impl BuiltinInlineFunction for IntAdd {
    fn id(&self) -> FunctionID {
        f_id(IntAdd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "add"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().immediate()), ("rhs", IntType::id().immediate())],
            Some(IntType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
"    mov rax, qword {lhs}
    add rax, qword {rhs}
    mov qword {return_into}, rax\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAsAdd;

impl BuiltinInlineFunction for IntAsAdd {
    fn id(&self) -> FunctionID {
        f_id(IntAsAdd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_add"
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
    add qword [rax], rdx\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}