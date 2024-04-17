use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{
    call_function, evaluate, instantiate_literal, FunctionHolder, Line,
};
use crate::root::compiler::local_variable::LocalVariable;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::parser::line_info::LineInfo;
use either::{Left, Right};

pub fn evaluate_symbol(
    symbol: &(BasicSymbol, LineInfo),
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<LocalVariable>,
) -> Result<Option<LocalVariable>, ProcessorError> {
    // println!("{:?}", symbol);
    Ok(match &symbol.0 {
        BasicSymbol::AbstractSyntaxTree(_) => panic!(),
        BasicSymbol::Operator(_) => {
            return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone()))
        }
        BasicSymbol::Literal(literal) => Some(instantiate_literal::instantiate_variable(
            Left((literal, &symbol.1)),
            lines,
            name_handler,
            function_holder,
            return_into,
        )?),
        BasicSymbol::BracketedSection(inner) => {
            evaluate::evaluate(inner, lines, name_handler, function_holder, return_into)?
        }
        BasicSymbol::Name(name) => {
            // println!("{:?}", name);
            match name_handler.resolve_name(function_holder, name, &symbol.1, lines)? {
                Left(new_variable) => {
                    if let Some(return_into) = return_into {
                        if return_into.type_info != new_variable.type_info {
                            return Err(ProcessorError::BadEvaluatedType(
                                symbol.1.clone(),
                                name_handler
                                    .type_table()
                                    .get_type(return_into.type_info.type_id)
                                    .unwrap()
                                    .get_indirect_name(return_into.type_info.reference_depth)
                                    .to_string(),
                                name_handler
                                    .type_table()
                                    .get_type(new_variable.type_info.type_id)
                                    .unwrap()
                                    .get_indirect_name(new_variable.type_info.reference_depth)
                                    .to_string(),
                            ));
                        }

                        lines.push(Line::Copy(
                            new_variable.offset,
                            return_into.offset,
                            name_handler
                                .type_table()
                                .get_type_size(return_into.type_info)?,
                        ));

                        Some(return_into)
                    } else {
                        Some(new_variable)
                    }
                }
                Right((function, default_args, args)) => call_function::call_function(
                    function,
                    &symbol.1,
                    default_args,
                    args,
                    lines,
                    name_handler,
                    function_holder,
                    return_into,
                )?,
            }
        }
        _other => return Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    })
}
