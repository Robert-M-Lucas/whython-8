use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::bool::{bool_op_sig, BoolType};
use crate::root::builtin::types::bool::printb::PrintB;
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolAnd;

impl BuiltinInlineFunction for BoolAnd {
    fn id(&self) -> FunctionID {
        f_id(BoolAnd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "and"
    }

    fn signature(&self) -> FunctionSignature {
        bool_op_sig()
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into, gt, sz| -> String {

            let jmp_false = gt.get_unique_tag(PrintB::id());
            let jmp_end = gt.get_unique_tag(PrintB::id());

            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
"    mov al, byte {lhs}
    cmp al, 0
    jz {jmp_false}
    mov al, byte {rhs}
    mov byte {return_into}, al
    jmp {jmp_end}
    {jmp_false}:
    mov byte {return_into}, 0
    {jmp_end}:
")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolAsAnd;

impl BuiltinInlineFunction for BoolAsAnd {
    fn id(&self) -> FunctionID {
        f_id(BoolAsAnd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_and"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", BoolType::id().with_indirection(1)), ("rhs", BoolType::id().immediate())],
            None
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], _, gt, sz| -> String {

            let jmp_true = gt.get_unique_tag(PrintB::id());

            let lhs = args[0];
            let rhs = args[1];
            format!(
"    mov al, byte {rhs}
    cmp al, 0
    jnz {jmp_true}
    mov rax, qword {lhs}
    mov byte [rax], 0
    {jmp_true}:
")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}