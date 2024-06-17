use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, f_id, InlineFunctionGenerator};
use crate::root::builtin::types::int::IntType;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::NameResult::Function;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeID, TypeRef};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct ExitFunction;

impl ExitFunction {
    pub const fn id() -> FunctionID {
        f_id(ExitFunction::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for ExitFunction {
    fn id(&self) -> FunctionID {
        Self::id()
    }

    fn name(&self) -> &'static str {
        "exit"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            false,
            &[("lhs", IntType::id().immediate())],
            None
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {
            let lhs = &args[0];

            // 0 us exit syscall
            format!("    mov rax, 60
    mov rdi, {lhs}
    syscall")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        None
    }
}
