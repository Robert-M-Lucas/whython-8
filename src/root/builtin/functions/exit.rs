use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

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
            SelfType::None,
            &[("lhs", IntType::id().immediate_single())],
            None,
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], _, _, _| -> String {
            let lhs = &args[0];

            // 0 us exit syscall
            format!(
                "    mov rax, 60
    mov rdi, {lhs}
    syscall\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        None
    }
}
