use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::int::IntType;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::NameResult::Function;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeID, TypeRef};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntMod;

impl BuiltinInlineFunction for IntMod {
    fn id(&self) -> FunctionID {
        f_id(IntMod::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "mod"
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
    mov rdx, 0
    mov rbx, qword {rhs}
    idiv rbx
    mov qword {return_into}, rdx\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAsMod;

impl BuiltinInlineFunction for IntAsMod {
    fn id(&self) -> FunctionID {
        f_id(IntAsMod::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_mod"
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
"    mov rcx, qword {lhs}
    mov rax, qword [rcx]
    mov rdx, 0
    mov rbx, qword {rhs}
    idiv rbx
    mov qword [rcx], rdx\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}