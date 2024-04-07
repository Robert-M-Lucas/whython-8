use crate::root::compiler::compile_functions::Function;
use crate::root::custom::functions::{print, windows};
use crate::root::custom::types::{bool, float, int};
use crate::root::name_resolver::type_builder::TypedFunction;

pub fn get_custom_function_signatures() -> Vec<(Option<isize>, Box<dyn TypedFunction>)> {
    let mut signatures = Vec::new();

    bool::add_function_signatures(&mut signatures);
    int::add_function_signatures(&mut signatures);
    float::add_function_signatures(&mut signatures);

    windows::add_function_signatures(&mut signatures);
    print::add_function_signatures(&mut signatures);

    signatures
}

pub fn get_custom_function_implementations() -> Vec<Box<dyn Function>> {
    let mut functions = Vec::new();

    print::add_function_implementations(&mut functions);

    functions
}
