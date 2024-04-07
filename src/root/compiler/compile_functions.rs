mod assignment;
mod call_function;
mod evaluate;
mod evaluate_symbol;
mod instantiate_literal;
mod name_handler;
mod operators;
mod process_lines;
mod reference;

use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::generate_asm::compile_user_function;
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::custom::get::{
    get_custom_function_implementations, get_custom_function_signatures,
};
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::{TypeTable, TypedFunction};
use crate::root::parser::line_info::LineInfo;
use name_handler::NameHandler;
use std::collections::HashMap;

pub enum Line {
    ReturnCall(isize, isize, Vec<(isize, usize)>, usize, isize),
    NoReturnCall(isize, isize, Vec<(isize, usize)>, usize),
    Copy(isize, isize, usize),
    DynFromCopy(isize, isize, usize),
    DynToCopy(isize, isize, usize),
    Return(Option<(isize, usize)>),
    HeapAlloc(usize, isize),
    HeapDealloc(isize, isize),
    InlineAsm(Vec<String>),
    #[cfg(debug_assertions)]
    Annotation(String),
}

pub struct UserFunction {
    pub id: isize,
    pub name: String,
    pub local_variable_size: usize,
    pub arg_count: usize,
    pub lines: Vec<Line>,
}

impl Function for UserFunction {
    fn get_asm(&self) -> String {
        compile_user_function(self)
    }

    fn get_id(&self) -> isize {
        self.id
    }
}

pub trait Function {
    fn get_asm(&self) -> String;
    fn get_id(&self) -> isize;
}

pub struct FunctionHolder {
    functions: HashMap<isize, Box<dyn TypedFunction>>,
    functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
}

impl FunctionHolder {
    pub fn new(
        functions: HashMap<isize, Box<dyn TypedFunction>>,
        functions_table: HashMap<Option<isize>, HashMap<String, isize>>,
    ) -> FunctionHolder {
        FunctionHolder {
            functions,
            functions_table,
        }
    }

    pub fn get_function(
        &self,
        _type: Option<TypeInfo>,
        name: &str,
    ) -> Option<&Box<dyn TypedFunction>> {
        self.functions_table
            .get(&_type.map(|x| x.type_id))
            .and_then(|x| x.get(name).map(|x| self.functions.get(x).unwrap()))
    }

    pub fn functions(&self) -> &HashMap<isize, Box<dyn TypedFunction>> {
        &self.functions
    }

    pub fn functions_table(&self) -> &HashMap<Option<isize>, HashMap<String, isize>> {
        &self.functions_table
    }
}

pub fn compile_functions(
    mut function_name_map: HashMap<Option<isize>, HashMap<String, isize>>,
    mut functions: HashMap<isize, Box<dyn TypedFunction>>,
    type_table: TypeTable,
) -> Result<Vec<Box<dyn Function>>, ProcessorError> {
    let mut function_contents: HashMap<isize, Vec<(BasicSymbol, LineInfo)>> = HashMap::new();
    for (id, func) in &mut functions {
        function_contents.insert(*id, func.take_contents());
    }
    for (t, f) in get_custom_function_signatures() {
        if function_name_map.get_mut(&t).is_none() {
            function_name_map.insert(t, HashMap::new());
        }
        function_name_map
            .get_mut(&t)
            .unwrap()
            .insert(f.get_name().to_string(), f.get_id());
        functions.insert(f.get_id(), f);
    }

    let function_holder = FunctionHolder::new(functions, function_name_map);
    let mut name_handler = NameHandler::new(type_table);
    let mut processed_functions = get_custom_function_implementations();
    name_handler.use_function_id(0);

    for (id, contents) in function_contents {
        let function = function_holder.functions.get(&id).unwrap();
        let name = function.get_name().to_string();
        name_handler.reset();
        name_handler.set_args(function.get_args_positioned(name_handler.type_table())?);
        let return_type = function.get_return_type();
        let mut lines = Vec::new();

        let last_return = process_lines::process_lines(
            &contents,
            id,
            return_type,
            &mut lines,
            &mut name_handler,
            &function_holder,
            None,
        )?;

        // TODO: Don't call if there is a return on the last line
        name_handler.destroy_local_variables(&mut lines)?;

        if return_type.is_some() && !last_return {
            return Err(ProcessorError::NoReturnStatement(function.get_line()));
        }

        processed_functions.push(Box::new(UserFunction {
            id,
            local_variable_size: name_handler.local_variable_space(),
            arg_count: function_holder
                .functions()
                .get(&id)
                .unwrap()
                .get_args()
                .len(),
            lines,
            name,
        }));
    }

    let processed_functions = processed_functions
        .into_iter()
        .filter(|f| name_handler.used_functions().contains(&f.get_id()))
        .collect();
    Ok(processed_functions)
}
