use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::types::bool::print_bool::PrintBool;
use crate::root::builtin::types::bool::{boolean_signature, BoolType};
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

/// Implements the boolean and operation
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
        boolean_signature()
    }

    fn inline(&self) -> InlineFnGenerator {
        |args, return_into, gt, _| -> Assembly {
            let jmp_false = gt.get_unique_tag(PrintBool::id());
            let jmp_end = gt.get_unique_tag(PrintBool::id());

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
"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolAssignAnd;

impl BuiltinInlineFunction for BoolAssignAnd {
    fn id(&self) -> FunctionID {
        f_id(BoolAssignAnd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_and"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::RefSelf,
            &[
                ("lhs", BoolType::id().with_indirection_single(1)),
                ("rhs", BoolType::id().immediate_single()),
            ],
            None,
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], _, gt, _| -> Assembly {
            let jmp_true = gt.get_unique_tag(PrintBool::id());

            let lhs = args[0];
            let rhs = args[1];
            format!(
                "    mov al, byte {rhs}
    cmp al, 0
    jnz {jmp_true}
    mov rax, qword {lhs}
    mov byte [rax], 0
    {jmp_true}:
"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}
