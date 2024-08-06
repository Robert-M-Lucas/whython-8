use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintNL;

impl PrintNL {
    pub const fn id() -> FunctionID {
        f_id(PrintNL::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for PrintNL {
    fn id(&self) -> FunctionID {
        Self::id()
    }

    fn name(&self) -> &'static str {
        "printnl"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::None,
            &[],
            None,
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |_, _, gt, sz| -> String {
            let id = format!("{}_fstr", Self::id().string_id());
            let data = format!("{id} db `\\n`,0");
            gt.add_readonly_data(&id, &data);
            
            format!(
                "    mov rdi, {id}
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
