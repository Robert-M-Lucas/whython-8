use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, f_id, InlineFunctionGenerator};
use crate::root::builtin::types::int::IntType;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::NameResult::Function;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeID, TypeRef};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintI;

impl PrintI {
    pub const fn id() -> FunctionID {
        f_id(PrintI::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for PrintI {
    fn id(&self) -> FunctionID {
        Self::id()
    }

    fn name(&self) -> &'static str {
        "printi"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().immediate())],
            None
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {
            let lhs = &args[0];

            let id = format!("{}_fstr", Self::id().string_id());

            let data = format!("{id} db `Integer: %d\\n`,0");

            gt.add_readonly_data(&id, &data);

            let lhs = args[0];
            format!(
                "
    mov rdi, {id}
    mov rsi, {lhs}
    mov al, 0
    sub rsp, {sz}
    extern printf
    call printf
    add rsp, {sz}
    ")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        None
    }
}
