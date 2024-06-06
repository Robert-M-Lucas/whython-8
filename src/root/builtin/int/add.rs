use unique_type_id::UniqueTypeId;
use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::builtin::int::IntType;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::NameResult::Function;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;

use crate::root::shared::common::{FunctionID, Indirection, LocalAddress, TypeID, TypeRef};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntAdd {}

impl BuiltinInlineFunction for IntAdd {
    fn id(&self) -> FunctionID {
        FunctionID(-(IntAdd::unique_type_id().0 as isize) - 1)
    }

    fn name(&self) -> &'static str {
        "add"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            &[("lhs", IntType::id().immediate()), ("rhs", IntType::id().immediate())],
            Some(IntType::id().immediate())
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args: &[LocalAddress], return_addr: Option<LocalAddress>| -> Result<String, WErr> {
            todo!()
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
