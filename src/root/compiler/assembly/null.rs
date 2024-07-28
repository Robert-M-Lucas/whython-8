use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};

pub struct NullFunction {
    id: FunctionID,
    parent_type: TypeID
}

impl BuiltinInlineFunction for NullFunction {
    fn id(&self) -> FunctionID {
        self.id
    }

    fn name(&self) -> &'static str {
        "null"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new(SelfType::None, vec![], Some(self.parent_type.with_indirection(1)))
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |_, return_into: Option<LocalAddress>, _, _| -> String {
            let return_into = return_into.unwrap();
            format!("    mov qword {return_into}, 0\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(self.parent_type)
    }
}

pub fn null_function(t: TypeID, f: FunctionID) -> NullFunction {
    NullFunction {
        id: f,
        parent_type: t,
    }
}