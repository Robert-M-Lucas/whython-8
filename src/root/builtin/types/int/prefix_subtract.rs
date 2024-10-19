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
pub struct IntPrefixSubtract;

impl BuiltinInlineFunction for IntPrefixSubtract {
    fn id(&self) -> FunctionID {
        f_id(IntPrefixSubtract::unique_type_id().0)
    }

    fn name(&self) -> &'static str {
        "p_sub"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[("lhs", IntType::id().immediate_single())],
            Some(IntType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, _, _| -> Assembly {
            let lhs = args[0];
            let return_into = return_into.unwrap();
            format!(
                "    mov rax, qword {lhs}
    neg rax
    mov qword {return_into}, rax\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

