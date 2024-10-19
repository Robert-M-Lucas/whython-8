use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;

/// `null` function that returns a null pointer with a specified type
pub struct NullFunction {
    id: FunctionID,
    parent_type: TypeID,
}

impl BuiltinInlineFunction for NullFunction {
    fn id(&self) -> FunctionID {
        self.id
    }

    fn name(&self) -> &'static str {
        "null"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new(
            SelfType::None,
            vec![],
            Some(self.parent_type.with_indirection_single(1)),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |_, return_into: Option<LocalAddress>, _, _| -> Assembly {
            let return_into = return_into.unwrap();
            format!("    mov qword {return_into}, 0\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(self.parent_type)
    }
}

/// Creates a `NullFunction` for a given type and function id
pub fn null_function(t: TypeID, f: FunctionID) -> NullFunction {
    NullFunction {
        id: f,
        parent_type: t,
    }
}

/// `is_null` function for checking if a reference is null
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IsNullFunction {
    id: FunctionID,
    parent_type: TypeID,
}

impl IsNullFunction {
    fn const_id() -> FunctionID {
        f_id(IsNullFunction::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IsNullFunction {
    fn id(&self) -> FunctionID {
        self.id
    }

    fn name(&self) -> &'static str {
        "is_null"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new(
            SelfType::None,
            vec![(
                SimpleNameToken::new_builtin("pointer".to_string()),
                self.parent_type.with_indirection_single(1),
            )],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into, gt, _| -> Assembly {
            let jmp_false = gt.get_unique_tag(IsNullFunction::const_id());
            let jmp_end = gt.get_unique_tag(IsNullFunction::const_id());

            let lhs = args[0];
            let return_into = return_into.unwrap();
            format!(
                "    mov rax, qword {lhs}
    cmp rax, 0
    jz {jmp_false}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_false}:
    mov byte {return_into}, 1
    {jmp_end}:
"
            )
        }
    }
    fn parent_type(&self) -> Option<TypeID> {
        Some(self.parent_type)
    }
}

/// Generates an `IsNullFunction` for a given type and function id
pub fn is_null_function(t: TypeID, f: FunctionID) -> IsNullFunction {
    IsNullFunction {
        id: f,
        parent_type: t,
    }
}
