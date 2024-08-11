use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntDiv;

impl BuiltinInlineFunction for IntDiv {
    fn id(&self) -> FunctionID {
        f_id(IntDiv::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "div"
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
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
                "    mov rax, qword {lhs}
    mov rdx, 0
    mov rbx, qword {rhs}
    idiv rbx
    mov qword {return_into}, rax\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAsDiv;

impl BuiltinInlineFunction for IntAsDiv {
    fn id(&self) -> FunctionID {
        f_id(IntAsDiv::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "as_div"
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
        |args: &[LocalAddress], _, _, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            format!(
                "    mov rcx, qword {lhs}
    mov rax, qword [rcx]
    mov rdx, 0
    mov rbx, qword {rhs}
    idiv rbx
    mov qword [rcx], rax\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
