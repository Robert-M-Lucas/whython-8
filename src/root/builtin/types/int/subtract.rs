use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;

/// Implements the integer subtract operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntSubtract;

impl BuiltinInlineFunction for IntSubtract {
    fn id(&self) -> FunctionID {
        f_id(IntSubtract::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "sub"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(IntType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
                "    mov rax, qword {lhs}
    sub rax, qword {rhs}
    mov qword {return_into}, rax\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer subtract assign operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAssignSubtract;

impl BuiltinInlineFunction for IntAssignSubtract {
    fn id(&self) -> FunctionID {
        f_id(IntAssignSubtract::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_sub"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::RefSelf,
            &[
                ("lhs", IntType::id().with_indirection_single(1)),
                ("rhs", IntType::id().immediate_single()),
            ],
            None,
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], _, _, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            format!(
                "    mov rax, qword {lhs}
    mov rdx, qword {rhs}
    sub qword [rax], rdx\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}