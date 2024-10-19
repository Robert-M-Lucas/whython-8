use crate::root::assembler::assembly_builder::Assembly;
use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, TypeID};
use unique_type_id::UniqueTypeId;

/// Implements the integer prefix add operation
/// This operation does nothing (e.g. `+6` is `6`)
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntPrefixAddition;

impl BuiltinInlineFunction for IntPrefixAddition {
    fn id(&self) -> FunctionID {
        f_id(IntPrefixAddition::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "p_add"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[("lhs", IntType::id().immediate_single())],
            Some(IntType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |_, _, _, _| -> Assembly { String::new() }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
