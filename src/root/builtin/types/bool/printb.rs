use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintB;

impl PrintB {
    pub const fn id() -> FunctionID {
        f_id(PrintB::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for PrintB {
    fn id(&self) -> FunctionID {
        Self::id()
    }

    fn name(&self) -> &'static str {
        "printb"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::None,
            &[("lhs", BoolType::id().immediate_single())],
            None,
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {
            let id_false = format!("{}_f_fstr", Self::id().string_id());
            let id_true = format!("{}_t_fstr", Self::id().string_id());

            let data_false = format!("{id_false} db `Boolean: False\\n`,0");
            let data_true = format!("{id_true} db `Boolean: True\\n`,0");

            gt.add_readonly_data(&id_false, &data_false);
            gt.add_readonly_data(&id_true, &data_true);

            let jmp_false = gt.get_unique_tag(PrintB::id());
            let jmp_end = gt.get_unique_tag(PrintB::id());

            let lhs = args[0];
            format!(
                "    mov al, byte {lhs}
    cmp al, 0
    jz {jmp_false}
    mov rdi, {id_true}
    jmp {jmp_end}
    {jmp_false}:
    mov rdi, {id_false}
    {jmp_end}:
    mov rsi, 0
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
