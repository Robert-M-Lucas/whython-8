use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

use crate::root::ast::literals::Literal;
use crate::root::compiler::generate_asm::get_local_address;
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::custom::types::bool::Bool;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::{Type, TypeTable, TypedFunction};
use crate::root::parser::line_info::LineInfo;

pub fn add_function_signatures(existing: &mut Vec<(Option<isize>, Box<dyn TypedFunction>)>) {
    let signatures: [(Option<isize>, Box<dyn TypedFunction>); 10] = [
        (Some(Float::get_id()), Box::new(FloatAdd {})),
        (Some(Float::get_id()), Box::new(FloatSub {})),
        (Some(Float::get_id()), Box::new(FloatMul {})),
        (Some(Float::get_id()), Box::new(FloatDiv {})),
        (Some(Float::get_id()), Box::new(FloatLT {})),
        (Some(Float::get_id()), Box::new(FloatGT {})),
        (Some(Float::get_id()), Box::new(FloatLE {})),
        (Some(Float::get_id()), Box::new(FloatGE {})),
        (Some(Float::get_id()), Box::new(FloatEQ {})),
        (Some(Float::get_id()), Box::new(FloatNE {})),
    ];
    for s in signatures {
        existing.push(s);
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct Float {}

impl Float {
    pub fn new() -> Float {
        Float {}
    }

    pub fn get_id() -> isize {
        -(Self::id().0 as isize) - 1
    }
}

impl Type for Float {
    fn get_id(&self) -> isize {
        Self::get_id()
    }

    fn get_name(&self) -> &str {
        "float"
    }

    fn get_size(
        &self,
        _type_table: &TypeTable,
        _path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError> {
        Ok(8)
    }

    fn instantiate(
        &self,
        literal: Option<&Literal>,
        local_address: isize,
    ) -> Result<Vec<String>, ProcessorError> {
        if literal.is_none() {
            return Ok(vec![]);
        }
        let Literal::Float(val) = literal.unwrap() else {
            panic!()
        };

        Ok(vec![
            format!("mov rax, __float64__({:?})", *val),
            format!("mov qword [{}], rax", get_local_address(local_address)),
        ])
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatAdd {}
lazy_static! {
    static ref FLOAT_ADD_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatAdd {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "add"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_ADD_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Float::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("addsd xmm0, qword [{}]", get_local_address(args[1])),
            format!("movsd qword [{}], xmm0", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatSub {}
lazy_static! {
    static ref FLOAT_SUB_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatSub {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "sub"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Float::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("subsd xmm0, qword [{}]", get_local_address(args[1])),
            format!("movsd qword [{}], xmm0", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatMul {}
lazy_static! {
    static ref FLOAT_MUL_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatMul {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "mul"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Float::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("mulsd xmm0, qword [{}]", get_local_address(args[1])),
            format!("movsd qword [{}], xmm0", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatDiv {}
lazy_static! {
    static ref FLOAT_DIV_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatDiv {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "div"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_SUB_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Float::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("divsd xmm0, qword [{}]", get_local_address(args[1])),
            format!("movsd qword [{}], xmm0", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatLT {}
lazy_static! {
    static ref FLOAT_LT_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatLT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "lt"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_LT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("seta [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatGT {}
lazy_static! {
    static ref FLOAT_GT_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatGT {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "gt"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_GT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setb [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatLE {}
lazy_static! {
    static ref FLOAT_LE_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatLE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "le"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_LE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setae [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatGE {}
lazy_static! {
    static ref FLOAT_GE_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatGE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "ge"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_GE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setbe [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatEQ {}
lazy_static! {
    static ref FLOAT_EQ_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatEQ {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_EQ_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setne [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct FloatNE {}
lazy_static! {
    static ref FLOAT_NE_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Float::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Float::get_id(), 0))
    ];
}
impl TypedFunction for FloatNE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "ne"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        FLOAT_NE_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        Some(TypeInfo::new(Bool::get_id(), 0))
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("movsd xmm0, qword [{}]", get_local_address(args[0])),
            format!("ucomisd xmm0, qword [{}]", get_local_address(args[1])),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("sete [{}]", get_local_address(args[2])),
        ]
    }
}
