use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::builtin::{f_id, BuiltinInlineFunction, InlineFnGenerator};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use unique_type_id::UniqueTypeId;
use crate::root::assembler::assembly_builder::Assembly;

/// Implements the integer equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntEqual;

impl IntEqual {
    pub const fn id() -> FunctionID {
        f_id(IntEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntEqual {
    fn id(&self) -> FunctionID {
        IntEqual::id()
    }

    fn name(&self) -> &'static str {
        "eq"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntEqual::id());
            let jmp_end = gt.get_unique_tag(IntEqual::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jz {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer not equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntNotEqual;

impl IntNotEqual {
    pub const fn id() -> FunctionID {
        f_id(IntNotEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntNotEqual {
    fn id(&self) -> FunctionID {
        IntNotEqual::id()
    }

    fn name(&self) -> &'static str {
        "ne"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntNotEqual::id());
            let jmp_end = gt.get_unique_tag(IntNotEqual::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jnz {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer greater than operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGreaterThan;

impl IntGreaterThan {
    pub const fn id() -> FunctionID {
        f_id(IntGreaterThan::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntGreaterThan {
    fn id(&self) -> FunctionID {
        IntGreaterThan::id()
    }

    fn name(&self) -> &'static str {
        "gt"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntGreaterThan::id());
            let jmp_end = gt.get_unique_tag(IntGreaterThan::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jg {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer less than operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLessThan;

impl IntLessThan {
    pub const fn id() -> FunctionID {
        f_id(IntLessThan::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntLessThan {
    fn id(&self) -> FunctionID {
        IntLessThan::id()
    }

    fn name(&self) -> &'static str {
        "lt"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntLessThan::id());
            let jmp_end = gt.get_unique_tag(IntLessThan::id());

            format!(
                "    mov rax, qword {rhs}
    cmp rax, qword {lhs}
    jg {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer greater than or equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGreaterThanEqual;

impl IntGreaterThanEqual {
    pub const fn id() -> FunctionID {
        f_id(IntGreaterThanEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntGreaterThanEqual {
    fn id(&self) -> FunctionID {
        IntGreaterThanEqual::id()
    }

    fn name(&self) -> &'static str {
        "ge"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntGreaterThanEqual::id());
            let jmp_end = gt.get_unique_tag(IntGreaterThanEqual::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jge {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

/// Implements the integer less than or equal operation
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLessThanEqual;

impl IntLessThanEqual {
    pub const fn id() -> FunctionID {
        f_id(IntLessThanEqual::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntLessThanEqual {
    fn id(&self) -> FunctionID {
        IntLessThanEqual::id()
    }

    fn name(&self) -> &'static str {
        "le"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new_inline_builtin(
            SelfType::CopySelf,
            &[
                ("lhs", IntType::id().immediate_single()),
                ("rhs", IntType::id().immediate_single()),
            ],
            Some(BoolType::id().immediate_single()),
        )
    }

    fn inline(&self) -> InlineFnGenerator {
        |args: &[LocalAddress], return_into: Option<LocalAddress>, gt, _| -> Assembly {
            let lhs = args[0];
            let rhs = args[1];
            let return_into = return_into.unwrap();
            let jmp_true = gt.get_unique_tag(IntLessThanEqual::id());
            let jmp_end = gt.get_unique_tag(IntLessThanEqual::id());

            format!(
                "    mov rax, qword {rhs}
    cmp rax, qword {lhs}
    jge {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
