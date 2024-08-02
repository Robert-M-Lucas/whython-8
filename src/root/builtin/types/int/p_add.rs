use unique_type_id::UniqueTypeId;

use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, TypeID};

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntPAdd;

impl BuiltinInlineFunction for IntPAdd {
    fn id(&self) -> FunctionID {
        f_id(IntPAdd::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "p_add"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[("lhs", IntType::id().immediate())],
            Some(IntType::id().immediate()),
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |_, _, _, _| -> String { String::new() }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
