use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::types::bool::{boolean_signature, BoolType};
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

/// Implements the boolean or operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolOr;

impl BuiltinInlineFunction for BoolOr {
    fn id(&self) -> FunctionID {
        f_id(BoolOr::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "or"
    }

    fn signature(&self) -> FunctionSignature {
        boolean_signature()
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into, _, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
                "    mov al, byte {lhs}
    or al, byte {rhs}
    mov byte {return_into}, al
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
pub struct BoolAssignOr;

impl BuiltinInlineFunction for BoolAssignOr {
    fn id(&self) -> FunctionID {
        f_id(BoolAssignOr::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_or"
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
        |args: &[LocalAddress], _, _, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            format!(
                "    mov rdx, qword {lhs}
    mov al, byte [rdx]
    or al, byte {rhs}
    mov byte [rdx], al
"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(BoolType::id())
    }
}
