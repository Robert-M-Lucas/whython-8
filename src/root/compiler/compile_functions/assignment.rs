use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{evaluate, operators, FunctionHolder, Line};
use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::parser::line_info::LineInfo;
use either::Left;

pub fn process_assignment(
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    line: &[(BasicSymbol, LineInfo)],
    is_ref: bool,
) -> Result<(), ProcessorError> {
    if line.len() < 2 {
        panic!()
    }

    match &line[1].0 {
        BasicSymbol::Assigner(assigner) => {
            let name = match &line[0].0 {
                BasicSymbol::Name(name) => name,
                _ => return Err(ProcessorError::NonNameAssignment(line[0].1.clone())),
            };
            let Left(variable) =
                name_handler.resolve_name(function_holder, name, &line[0].1, lines)?
            else {
                return Err(ProcessorError::AssignToNonVariable(line[0].1.clone()));
            };

            let (original_variable, variable) = if is_ref {
                if variable.type_info.reference_depth == 0 {
                    return Err(ProcessorError::CantDerefNonRef(line[0].1.clone()));
                }

                let new_type = TypeInfo::new(
                    variable.type_info.type_id,
                    variable.type_info.reference_depth - 1,
                );
                let non_ref = name_handler
                    .add_local_variable(None, new_type, lines)
                    .unwrap();
                lines.push(Line::DynFromCopy(
                    variable.offset,
                    non_ref,
                    name_handler.type_table().get_type_size(new_type)?,
                ));
                (
                    Some(variable),
                    LocalVariable::from_type_info(non_ref, new_type),
                )
            } else {
                (None, variable)
            };

            if line.len() < 3 {
                return Err(ProcessorError::NoAssignmentRHS(line[1].1.clone()));
            }
            if let Some(assigner) = assigner {
                let result =
                    evaluate::evaluate(&line[2..], lines, name_handler, function_holder, None)?
                        .ok_or(ProcessorError::DoesntEvaluate(line[2].1.clone()))?;
                operators::evaluate_operation(
                    variable,
                    (assigner, &line[1].1),
                    Some(result),
                    lines,
                    name_handler,
                    function_holder,
                    Some(variable),
                )?;
            } else {
                evaluate::evaluate(
                    &line[2..],
                    lines,
                    name_handler,
                    function_holder,
                    Some(variable),
                )?;
            }

            if is_ref {
                lines.push(Line::DynToCopy(
                    variable.offset,
                    original_variable.unwrap().offset,
                    name_handler
                        .type_table()
                        .get_type_size(variable.type_info)?,
                ));
            }

            Ok(())
        }
        _ => {
            panic!()
        }
    }
}
