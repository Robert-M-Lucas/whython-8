use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::generate_asm::get_local_address;
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::custom::types::int::Int;
use crate::root::name_resolver::type_builder::TypedFunction;
use crate::root::parser::line_info::LineInfo;
use lazy_static::lazy_static;
use unique_type_id::UniqueTypeId;

pub fn add_function_signatures(existing: &mut Vec<(Option<isize>, Box<dyn TypedFunction>)>) {
    let signatures: [(Option<isize>, Box<dyn TypedFunction>); 1] =
        [(None, Box::new(WindowsExit {}))];

    for s in signatures {
        existing.push(s);
    }
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "u16"]
pub struct WindowsExit {}
lazy_static! {
    static ref WINDOWS_EXIT_ARGS: [(String, TypeInfo); 1] =
        [(String::from("exit_code"), TypeInfo::new(Int::get_id(), 0))];
}
impl TypedFunction for WindowsExit {
    fn get_id(&self) -> isize {
        -(Self::id().0 as isize) - 1
    }

    fn get_name(&self) -> &str {
        "exit"
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        WINDOWS_EXIT_ARGS.as_ref()
    }

    fn get_line(&self) -> LineInfo {
        LineInfo::builtin()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        None
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }

    fn get_inline(&self, args: Vec<isize>) -> Vec<String> {
        vec![
            format!("mov rcx, qword [{}]", get_local_address(args[0])),
            "call ExitProcess".to_string(),
        ]
    }
}
