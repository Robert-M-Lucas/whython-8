use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::builtin::int::IntType;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::NameResult::Function;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeID, TypeRef};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntSub;

impl BuiltinInlineFunction for IntSub {
    fn id(&self) -> FunctionID {
        FunctionID(-(IntSub::unique_type_id().0 as isize) - 1)
    }

    fn name(&self) -> &'static str {
        "sub"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            true,
            &[("lhs", IntType::id().immediate()), ("rhs", IntType::id().immediate())],
            Some(IntType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>| -> String {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            format!(
"    mov rax, qword {lhs}
    sub rax, qword {rhs}
    mov qword {return_into}, rax")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
