use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::types::bool::{boolean_signature, BoolType};
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

/// Implements the boolean equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolEqual;

impl BoolEqual {
    pub const fn id() -> FunctionID {
        f_id(BoolEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for BoolEqual {
    fn id(&self) -> FunctionID {
        BoolEqual::id()
    }

    fn name(&self) -> &'static str {
        "eq"
    }

    fn signature(&self) -> FunctionSignature {
        boolean_signature()
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_false = gt.get_unique_tag(BoolEqual::id());
            let jmp_true = gt.get_unique_tag(BoolEqual::id());
            let jmp_end = gt.get_unique_tag(BoolEqual::id());

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
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}


/// Implements the boolean not equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNotEqual;

impl BoolNotEqual {
    pub const fn id() -> FunctionID {
        f_id(BoolNotEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for BoolNotEqual {
    fn id(&self) -> FunctionID {
        BoolNotEqual::id()
    }

    fn name(&self) -> &'static str {
        "ne"
    }

    fn signature(&self) -> FunctionSignature {
        boolean_signature()
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_false = gt.get_unique_tag(BoolNotEqual::id());
            let jmp_true = gt.get_unique_tag(BoolNotEqual::id());
            let jmp_end = gt.get_unique_tag(BoolNotEqual::id());

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
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}
