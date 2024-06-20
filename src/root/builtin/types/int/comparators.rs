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
            let jmp_true = gt.get_unique_tag(IntEq::id());
            let jmp_end = gt.get_unique_tag(IntEq::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jz {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGT;

impl IntGT {
    pub const fn id() -> FunctionID {
        f_id(IntGT::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntGT {
    fn id(&self) -> FunctionID {
        IntGT::id()
    }

    fn name(&self) -> &'static str {
        "gt"
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
            let jmp_true = gt.get_unique_tag(IntEq::id());
            let jmp_end = gt.get_unique_tag(IntEq::id());

            format!(
"    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jg {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLT;

impl IntLT {
    pub const fn id() -> FunctionID {
        f_id(IntLT::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntLT {
    fn id(&self) -> FunctionID {
        IntLT::id()
    }

    fn name(&self) -> &'static str {
        "lt"
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
            let jmp_true = gt.get_unique_tag(IntEq::id());
            let jmp_end = gt.get_unique_tag(IntEq::id());

            format!(
                "    mov rax, qword {rhs}
    cmp rax, qword {lhs}
    jg {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntGE;

impl IntGE {
    pub const fn id() -> FunctionID {
        f_id(IntGE::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntGE {
    fn id(&self) -> FunctionID {
        IntGE::id()
    }

    fn name(&self) -> &'static str {
        "ge"
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
            let jmp_true = gt.get_unique_tag(IntEq::id());
            let jmp_end = gt.get_unique_tag(IntEq::id());

            format!(
                "    mov rax, qword {lhs}
    cmp rax, qword {rhs}
    jge {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct IntLE;

impl IntLE {
    pub const fn id() -> FunctionID {
        f_id(IntLE::unique_type_id().0)
    }
}

impl BuiltinInlineFunction for IntLE {
    fn id(&self) -> FunctionID {
        IntLE::id()
    }

    fn name(&self) -> &'static str {
        "le"
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
            let jmp_true = gt.get_unique_tag(IntEq::id());
            let jmp_end = gt.get_unique_tag(IntEq::id());

            format!(
                "    mov rax, qword {rhs}
    cmp rax, qword {lhs}
    jge {jmp_true}
    mov byte {return_into}, 0
    jmp {jmp_end}
    {jmp_true}:
    mov byte {return_into}, 1
    {jmp_end}:\n")
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(IntType::id())
    }
}
