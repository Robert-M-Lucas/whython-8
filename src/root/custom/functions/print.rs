use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::{Function, Line, UserFunction};
use crate::root::compiler::generate_asm::{compile_user_function, get_function_sublabel};
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::name_resolver::type_builder::TypedFunction;
use crate::root::parser::line_info::LineInfo;

pub fn add_function_signatures(existing: &mut Vec<(Option<isize>, Box<dyn TypedFunction>)>) {
    let signatures: [(Option<isize>, Box<dyn TypedFunction>); 3] = [
        (None, Box::new(PrintI {})),
        (None, Box::new(PrintB {})),
        (None, Box::new(PrintF {})),
    ];

    for s in signatures {
        existing.push(s);
    }
}

pub fn add_function_implementations(existing: &mut Vec<Box<dyn Function>>) {
    let functions: [Box<dyn Function>; 3] = [
        Box::new(PrintI {}),
        Box::new(PrintB {}),
        Box::new(PrintF {}),
    ];

    for s in functions {
        existing.push(s);
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintI {}
lazy_static! {
    static ref PRINT_I_ARGS: [(String, TypeInfo); 1] =
        [(String::from("integer"), TypeInfo::new(Int::get_id(), 0))];
}
impl TypedFunction for PrintI {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printi"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        PRINT_I_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, _args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

impl Function for PrintI {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 8,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov dword [rbp-4], 0x000a".to_string(),
                "mov dword [rbp-8], 0x646C6C25".to_string(),
                "mov rcx, rbp".to_string(),
                "sub rcx, 8".to_string(),
                "mov rdx, qword [rbp+16]".to_string(),
                "sub rsp, 40".to_string(),
                "call printf".to_string(),
                "add rsp, 40".to_string(),
            ])],
            name: "printi".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintB {}
lazy_static! {
    static ref PRINT_B_ARGS: [(String, TypeInfo); 1] =
        [(String::from("bool"), TypeInfo::new(Bool::get_id(), 0))];
}
impl TypedFunction for PrintB {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printb"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        PRINT_B_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }
}

impl Function for PrintB {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 32,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov dword [rbp-8], 0x65757274".to_string(),
                "mov dword [rbp-4], 0x0D0A".to_string(),
                "mov rax, qword [rbp+16]".to_string(),
                "cmp rax, 0".to_string(),
                format!(
                    "jz {}",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "mov dword [rbp-8], 0x736C6166".to_string(),
                "mov dword [rbp-4], 0x0D0A65".to_string(),
                format!(
                    "{}:",
                    get_function_sublabel(TypedFunction::get_id(self), "true")
                ),
                "mov rcx, rbp".to_string(),
                "sub rcx, 8".to_string(),
                "mov rdx, qword [rbp+16]".to_string(),
                "sub rsp, 40".to_string(),
                "call printf".to_string(),
                "add rsp, 40".to_string(),
            ])],
            name: "printb".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct PrintF {}
lazy_static! {
    static ref PRINT_F_ARGS: [(String, TypeInfo); 1] =
        [(String::from("float"), TypeInfo::new(Float::get_id(), 0))];
}
impl TypedFunction for PrintF {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "printf"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        PRINT_F_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        None
    }

    fn is_inline(&self) -> bool {
        false
    }
}

impl Function for PrintF {
    fn get_asm(&self) -> String {
        compile_user_function(&UserFunction {
            id: TypedFunction::get_id(self),
            local_variable_size: 8,
            arg_count: 1,
            lines: vec![Line::InlineAsm(vec![
                "mov dword [rbp-4], 0x00".to_string(),
                "mov dword [rbp-8], 0x0a664C25".to_string(),
                "mov rcx, rbp".to_string(),
                "sub rcx, 8".to_string(),
                "movsd xmm1, qword [rbp+16]".to_string(),
                "mov rdx, qword [rbp+16]".to_string(),
                "sub rsp, 40".to_string(),
                "call printf".to_string(),
                "add rsp, 40".to_string(),
            ])],
            name: "printf".to_string(),
        })
    }

    fn get_id(&self) -> isize {
        TypedFunction::get_id(self)
    }
}
