use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{evaluate_symbol, operators, FunctionHolder, Line};
use crate::root::compiler::local_variable::LocalVariable;
use crate::root::parser::line_info::LineInfo;
use crate::root::name_resolver::processor::ProcessorError;

pub fn evaluate(
    section: &[(BasicSymbol, LineInfo)],
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<LocalVariable>,
) -> Result<Option<LocalVariable>, ProcessorError> {
    // addr, type
    Ok(if section.len() == 1 {
        evaluate_symbol::evaluate_symbol(
            &section[0],
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else if section.len() == 2 {
        let op = operators::evaluate_operator(&section[0])?;
        let Some(value) = evaluate_symbol::evaluate_symbol(
            &section[1],
            lines,
            name_handler,
            function_holder,
            None,
        )?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[1].1.clone()));
        };
        operators::evaluate_operation(
            value,
            (op, &section[1].1),
            None,
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else if section.len() == 3 {
        let Some(lhs) = evaluate_symbol::evaluate_symbol(
            &section[0],
            lines,
            name_handler,
            function_holder,
            None,
        )?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[0].1.clone()));
        };
        let op = operators::evaluate_operator(&section[1])?;
        let Some(rhs) = evaluate_symbol::evaluate_symbol(
            &section[2],
            lines,
            name_handler,
            function_holder,
            None,
        )?
        else {
            return Err(ProcessorError::DoesntEvaluate(section[2].1.clone()));
        };
        operators::evaluate_operation(
            lhs,
            (op, &section[1].1),
            Some(rhs),
            lines,
            name_handler,
            function_holder,
            return_into,
        )?
    } else {
        return Err(ProcessorError::BadEvaluableLayout(section[3].1.clone()));
    })
}
