use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

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
            SelfType::None,
            &[("lhs", IntType::id().immediate_single())],
            None,
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {
            let id = format!("{}_fstr", Self::id().string_id());

            let data = format!("{id} db `Integer: %ld\\n`,0");

            gt.add_readonly_data(&id, &data);

            let lhs = args[0];
            format!(
                "    mov rdi, {id}
    mov rsi, {lhs}
    mov al, 0
    sub rsp, {sz}
    extern printf
    call printf
    add rsp, {sz}
"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        None
    }
}
