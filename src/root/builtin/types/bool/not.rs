use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::types::bool::print_bool::PrintBool;
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};


/// Implements the boolean not operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNot;

impl BuiltinInlineFunction for BoolNot {
    fn id(&self) -> FunctionID {
        f_id(BoolNot::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "p_not"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[("lhs", BoolType::id().immediate_single())],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into, gt, _| -> Assembly {
            let jmp_false = gt.get_unique_tag(PrintBool::id());
            let jmp_end = gt.get_unique_tag(PrintBool::id());

            let lhs = args[0];
            let return_into = return_into.unwrap();
            format!(
                "    mov al, byte {lhs}
    cmp al, 0
    jz {jmp_false}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_false}:
    mov byte {return_into}, 1
    {jmp_end}:
"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}
