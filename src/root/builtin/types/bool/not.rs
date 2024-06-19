use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::bool::printb::PrintB;
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

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
            true,
            &[("lhs", BoolType::id().immediate())],
            Some(BoolType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into, gt, sz| -> String {

            let jmp_false = gt.get_unique_tag(PrintB::id());
            let jmp_end = gt.get_unique_tag(PrintB::id());

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
")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}