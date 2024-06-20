use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, f_id, InlineFunctionGenerator};
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolEq;

impl BoolEq {
    pub const fn id() -> FunctionID {
        f_id(BoolEq::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for BoolEq {
    fn id(&self) -> FunctionID {
        BoolEq::id()
    }

    fn name(&self) -> &'static str {
        "eq"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", BoolType::id().immediate()), ("rhs", BoolType::id().immediate())],
            Some(BoolType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_false = gt.get_unique_tag(BoolEq::id());
            let jmp_true = gt.get_unique_tag(BoolEq::id());
            let jmp_end = gt.get_unique_tag(BoolEq::id());

            format!(
"    cmp byte {lhs}, 0
    jz {jmp_false}
    mov al, byte {rhs}
    mov byte {return_into}, al
    jmp {jmp_end}
    {jmp_false}:
    cmp byte {rhs}, 0
    jnz {jmp_true}
    mov byte {return_into}, 1
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 0
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNE;

impl BoolNE {
    pub const fn id() -> FunctionID {
        f_id(BoolNE::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for BoolNE {
    fn id(&self) -> FunctionID {
        BoolNE::id()
    }

    fn name(&self) -> &'static str {
        "ne"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", BoolType::id().immediate()), ("rhs", BoolType::id().immediate())],
            Some(BoolType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_false = gt.get_unique_tag(BoolNE::id());
            let jmp_true = gt.get_unique_tag(BoolNE::id());
            let jmp_end = gt.get_unique_tag(BoolNE::id());

            format!(
                "    cmp byte {lhs}, 0
    jnz {jmp_true}
    mov al, byte {rhs}
    mov byte {return_into}, al
    jmp {jmp_end}
    {jmp_true}:
    cmp byte {rhs}, 0
    jnz {jmp_false}
    mov byte {return_into}, 1
    jmp {jmp_end}
    {jmp_false}:
    mov byte {return_into}, 0
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}
