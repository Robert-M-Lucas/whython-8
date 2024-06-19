use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator, f_id};
use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntEq;

impl IntEq {
    pub const fn id() -> FunctionID {
        f_id(IntEq::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntEq {
    fn id(&self) -> FunctionID {
        IntEq::id()
    }

    fn name(&self) -> &'static str {
        "eq"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().immediate()), ("rhs", IntType::id().immediate())],
            Some(BoolType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let j = gt.get_unique_tag(IntEq::id());
            let j2 = gt.get_unique_tag(IntEq::id());

            format!(
"    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jz {j}
    mov byte {return_into}, 0
    jmp {j2}
    {j}:
    mov byte {return_into}, 1
    {j2}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
