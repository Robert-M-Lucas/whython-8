use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

use crate::root::ast::literals::Literal;
use crate::root::compiler::generate_asm::get_local_address;
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::parser::line_info::LineInfo;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::{Type, TypedFunction, TypeTable};

pub fn add_function_signatures(existing: &mut Vec<(Option<isize>, Box<dyn TypedFunction>)>) {
    let signatures: [(Option<isize>, Box<dyn TypedFunction>); 3] = [
        (Some(Bool::get_id()), Box::new(BoolNot {})),
        (Some(Bool::get_id()), Box::new(BoolEQ {})),
        (Some(Bool::get_id()), Box::new(BoolNE {}))
    ];
    for s in signatures {
        existing.push(s);
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct Bool {}

impl Bool {
    pub fn new() -> Bool {
        Bool {}
    }
    pub fn get_id() -> isize {
        -(Self::id().0 as isize) - 1
    }
}

impl Type for Bool {
    fn get_id(&self) -> isize {
        Self::get_id()
    }

    fn get_name(&self) -> &str {
        "bool"
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
        let Literal::Bool(val) = literal.unwrap() else {
            panic!()
        };

        if *val {
            Ok(vec![format!(
                "mov qword [{}], 0",
                get_local_address(local_address)
            )])
        } else {
            Ok(vec![format!(
                "mov qword [{}], 1",
                get_local_address(local_address)
            )])
        }
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNot {}
lazy_static! {
    static ref BOOL_NOT_ARGS: [(String, TypeInfo); 1] =
        [(String::from("lhs"), TypeInfo::new(Bool::get_id(), 0))];
}
impl TypedFunction for BoolNot {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "not"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        BOOL_NOT_ARGS.as_ref()
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
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            "cmp rax, 0".to_string(),
            "setz al".to_string(),
            format!("mov qword [{}], rax", get_local_address(args[1])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolEQ {}
lazy_static! {
    static ref BOOL_EQ_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Bool::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Bool::get_id(), 0))
    ];
}
impl TypedFunction for BoolEQ {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "eq"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        BOOL_EQ_ARGS.as_ref()
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
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setnz [{}]", get_local_address(args[2])),
        ]
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct BoolNE {}
lazy_static! {
    static ref BOOL_NE_ARGS: [(String, TypeInfo); 2] = [
        (String::from("lhs"), TypeInfo::new(Bool::get_id(), 0)),
        (String::from("rhs"), TypeInfo::new(Bool::get_id(), 0))
    ];
}
impl TypedFunction for BoolNE {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "ne"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        BOOL_NE_ARGS.as_ref()
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
            format!("mov rax, qword [{}]", get_local_address(args[0])),
            format!("mov rcx, [{}]", get_local_address(args[1])),
            "cmp rcx, rax".to_string(),
            format!("mov qword [{}], 0", get_local_address(args[2])),
            format!("setz [{}]", get_local_address(args[2])),
        ]
    }
}
