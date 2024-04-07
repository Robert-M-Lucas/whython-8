use crate::root::ast::keywords::Keyword;
use crate::root::ast::operators::Operator;
use crate::root::basic_ast::punctuation::Punctuation;
use crate::root::basic_ast::symbol::{BasicSymbol, NameType};
use crate::root::compiler::compile_functions::assignment::process_assignment;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{evaluate, FunctionHolder, Line};
use crate::root::compiler::generate_asm::{get_function_sublabel, get_local_address};
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::Type;
use crate::root::parser::line_info::LineInfo;

use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::custom::types::bool::Bool;
#[cfg(debug_assertions)]
use itertools::Itertools;
#[cfg(debug_assertions)]
use std::fs;

pub fn process_lines(
    section: &[(BasicSymbol, LineInfo)],
    current_id: isize,
    return_type: Option<TypeInfo>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    mut break_label: Option<String>,
) -> Result<bool, ProcessorError> {
    let mut last_return = false;

    #[cfg(debug_assertions)]
    let mut prev_line = None;
    for line in section.split(|x| matches!(x.0, BasicSymbol::Punctuation(Punctuation::Semicolon))) {
        if line.is_empty() {
            continue;
        }

        #[cfg(debug_assertions)]
        {
            let new_line = line.last().unwrap().1.line();
            let file = line.last().unwrap().1.file().unwrap();
            let start = if let Some(prev_line) = prev_line {
                prev_line
            } else {
                line.first().unwrap().1.line()
            };

            let text = fs::read_to_string(file.as_path()).unwrap();
            let text = &text.lines().collect_vec()[(start - 1)..(new_line)];

            for l in text {
                lines.push(Line::Annotation(l.to_string()));
            }

            prev_line = Some(new_line + 1);
        }

        last_return = false;

        if line.len() > 2
            && matches!(&line[0].0, BasicSymbol::Operator(Operator::Product))
            && matches!(&line[2].0, BasicSymbol::Assigner(_))
        {
            process_assignment(lines, name_handler, function_holder, &line[1..], true)?;
            continue;
        }

        if line.len() > 1 && matches!(&line[1].0, BasicSymbol::Assigner(_)) {
            process_assignment(lines, name_handler, function_holder, line, false)?;
            continue;
        }

        match &line[0].0 {
            BasicSymbol::Keyword(Keyword::Return) => {
                last_return = true;
                if line.len() == 1 {
                    if return_type.is_none() {
                        name_handler.destroy_local_variables(lines)?;
                        lines.push(Line::Return(None));
                        continue;
                    } else {
                        return Err(ProcessorError::NoneReturnOnTypedFunction(line[0].1.clone()));
                    }
                } else if return_type.is_none() {
                    return Err(ProcessorError::TypeReturnOnVoidFunction(line[1].1.clone()));
                }
                let return_type = return_type.unwrap();

                let return_into = name_handler.add_local_variable(None, return_type, lines)?;
                let return_value = evaluate::evaluate(
                    &line[1..],
                    lines,
                    name_handler,
                    function_holder,
                    Some(LocalVariable::from_type_info(return_into, return_type)),
                )?;
                if return_value.is_none() {
                    return Err(ProcessorError::DoesntEvaluate(line[1].1.clone()));
                }
                let return_value = return_value.unwrap();
                if return_type != return_value.type_info {
                    return Err(ProcessorError::BadReturnType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(return_type.type_id)
                            .unwrap()
                            .get_indirect_name(return_type.reference_depth)
                            .to_string(),
                        name_handler
                            .type_table()
                            .get_type(return_value.type_info.type_id)
                            .unwrap()
                            .get_indirect_name(return_value.type_info.reference_depth)
                            .to_string(),
                    ));
                }
                name_handler.destroy_local_variables(lines)?;
                lines.push(Line::Return(Some((
                    return_value.offset,
                    name_handler
                        .type_table()
                        .get_type_size(return_value.type_info)?,
                ))));
            }
            BasicSymbol::Keyword(Keyword::Let) => {
                if line.len() < 2 {
                    return Err(ProcessorError::LetNoName(line[0].1.clone()));
                }
                let BasicSymbol::Name(name) = &line[1].0 else {
                    return Err(ProcessorError::LetNoName(line[1].1.clone()));
                };

                if name.len() > 1 {
                    return Err(ProcessorError::MultipartNameDef(line[1].1.clone()));
                }
                let name = &name[0];
                if name.3 != 0 {
                    return Err(ProcessorError::NameWithRefPrefix(line[0].1.clone()));
                }

                if !matches!(&name.2, NameType::Normal) {
                    return Err(ProcessorError::LetNoName(line[0].1.clone()));
                }
                let name = &name.0;

                if line.len() < 4 {
                    return Err(ProcessorError::NameTypeNotDefined(line[1].1.clone()));
                }
                if !matches!(&line[2].0, BasicSymbol::Punctuation(Punctuation::Colon)) {
                    return Err(ProcessorError::NameTypeNotDefined(line[2].1.clone()));
                }

                let BasicSymbol::Name(type_name) = &line[3].0 else {
                    return Err(ProcessorError::NameTypeNotDefined(line[3].1.clone()));
                };

                if type_name.len() > 1 {
                    return Err(ProcessorError::MultipartTypeName(line[3].1.clone()));
                }
                let type_name = &type_name[0];

                if !matches!(&type_name.2, NameType::Normal) {
                    return Err(ProcessorError::NameTypeNotDefined(line[3].1.clone()));
                }

                let type_id = name_handler
                    .type_table()
                    .get_id_by_name(&type_name.0)
                    .ok_or(ProcessorError::TypeNotFound(
                        line[3].1.clone(),
                        type_name.0.clone(),
                    ))?;
                let addr = name_handler.add_local_variable(
                    None,
                    TypeInfo::new(type_id, type_name.3),
                    lines,
                )?;

                if line.len() < 6 {
                    return Err(ProcessorError::LetNoValue(line[3].1.clone()));
                }
                if !matches!(&line[4].0, BasicSymbol::Assigner(None)) {
                    return Err(ProcessorError::LetNoValue(line[4].1.clone()));
                }

                evaluate::evaluate(
                    &line[5..],
                    lines,
                    name_handler,
                    function_holder,
                    Some(LocalVariable::new(addr, type_id, type_name.3)),
                )?;

                name_handler.name_variable(name.clone(), addr, TypeInfo::new(type_id, type_name.3));
            }
            BasicSymbol::Keyword(Keyword::While) => {
                if line.len() < 2 {
                    return Err(ProcessorError::WhileNoBrackets(line[0].1.clone()));
                }

                let BasicSymbol::BracketedSection(expr) = &line[1].0 else {
                    return Err(ProcessorError::WhileNoBrackets(line[1].1.clone()));
                };
                let start_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                let end_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                break_label = Some(end_label.clone());

                lines.push(Line::InlineAsm(vec![format!("{}:", start_label)]));
                let evaluated =
                    evaluate::evaluate(expr, lines, name_handler, function_holder, None)?
                        .ok_or(ProcessorError::DoesntEvaluate(line[1].1.clone()))?;
                lines.push(Line::InlineAsm(vec![
                    format!("mov rax, qword [{}]", get_local_address(evaluated.offset)),
                    "cmp rax, 0".to_string(),
                    format!("jnz {}", end_label),
                ]));

                if evaluated.type_info != TypeInfo::new(Bool::new().get_id(), 0) {
                    return Err(ProcessorError::BadConditionType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(evaluated.type_info.type_id)
                            .unwrap()
                            .get_indirect_name(evaluated.type_info.reference_depth)
                            .to_string(),
                    ));
                }
                if line.len() < 3 {
                    return Err(ProcessorError::WhileNoBraces(line[1].1.clone()));
                }
                let BasicSymbol::BracedSection(inner) = &line[2].0 else {
                    return Err(ProcessorError::WhileNoBraces(line[2].1.clone()));
                };
                process_lines(
                    inner,
                    current_id,
                    return_type,
                    lines,
                    name_handler,
                    function_holder,
                    break_label.clone(),
                )?;
                lines.push(Line::InlineAsm(vec![
                    format!("jmp {}", start_label),
                    format!("{}:", end_label),
                ]));

                if line.len() > 3 {
                    return Err(ProcessorError::WhileMoreAfterBraces(line[3].1.clone()));
                }
            }
            BasicSymbol::Keyword(Keyword::If) => {
                if line.len() < 2 {
                    return Err(ProcessorError::IfElifNoBrackets(line[0].1.clone()));
                }

                let BasicSymbol::BracketedSection(expr) = &line[1].0 else {
                    return Err(ProcessorError::IfElifNoBrackets(line[1].1.clone()));
                };
                let mut next_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());
                let end_label =
                    get_function_sublabel(current_id, &name_handler.get_uid().to_string());

                let evaluated =
                    evaluate::evaluate(expr, lines, name_handler, function_holder, None)?
                        .ok_or(ProcessorError::DoesntEvaluate(line[1].1.clone()))?;
                if evaluated.type_info != TypeInfo::new(Bool::new().get_id(), 0) {
                    return Err(ProcessorError::BadConditionType(
                        line[1].1.clone(),
                        name_handler
                            .type_table()
                            .get_type(evaluated.type_info.type_id)
                            .unwrap()
                            .get_indirect_name(evaluated.type_info.reference_depth)
                            .to_string(),
                    ));
                }
                lines.push(Line::InlineAsm(vec![
                    format!("mov rax, qword [{}]", get_local_address(evaluated.offset)),
                    "cmp rax, 0".to_string(),
                    format!("jnz {}", next_label),
                ]));
                if line.len() < 3 {
                    return Err(ProcessorError::IfElifElseNoBraces(line[1].1.clone()));
                }
                let BasicSymbol::BracedSection(inner) = &line[2].0 else {
                    return Err(ProcessorError::IfElifElseNoBraces(line[2].1.clone()));
                };
                process_lines(
                    inner,
                    current_id,
                    return_type,
                    lines,
                    name_handler,
                    function_holder,
                    break_label.clone(),
                )?;

                let mut i = 3;
                let mut ended = false;
                while line.len() > i {
                    lines.push(Line::InlineAsm(vec![
                        format!("jmp {}", end_label),
                        format!("{}:", next_label),
                    ]));

                    next_label =
                        get_function_sublabel(current_id, &name_handler.get_uid().to_string());

                    match &line[i].0 {
                        BasicSymbol::Keyword(Keyword::Elif) => {
                            if ended {
                                return Err(ProcessorError::IfElifAfterElse(line[i].1.clone()));
                            }
                            i += 1;
                            if line.len() <= i {
                                return Err(ProcessorError::IfElifNoBrackets(
                                    line[i - 1].1.clone(),
                                ));
                            }

                            let BasicSymbol::BracketedSection(expr) = &line[i].0 else {
                                return Err(ProcessorError::IfElifNoBrackets(line[i].1.clone()));
                            };

                            let evaluated = evaluate::evaluate(
                                expr,
                                lines,
                                name_handler,
                                function_holder,
                                None,
                            )?
                            .ok_or(ProcessorError::DoesntEvaluate(line[i].1.clone()))?;
                            if evaluated.type_info != TypeInfo::new(Bool::new().get_id(), 0) {
                                return Err(ProcessorError::BadConditionType(
                                    line[i].1.clone(),
                                    name_handler
                                        .type_table()
                                        .get_type(evaluated.type_info.type_id)
                                        .unwrap()
                                        .get_indirect_name(evaluated.type_info.reference_depth)
                                        .to_string(),
                                ));
                            }
                            lines.push(Line::InlineAsm(vec![
                                format!("mov rax, qword [{}]", get_local_address(evaluated.offset)),
                                "cmp rax, 0".to_string(),
                                format!("jnz {}", next_label),
                            ]));

                            i += 1;
                            if line.len() <= i {
                                return Err(ProcessorError::IfElifElseNoBraces(
                                    line[i - 1].1.clone(),
                                ));
                            }
                            let BasicSymbol::BracedSection(inner) = &line[i].0 else {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i].1.clone()));
                            };
                            process_lines(
                                inner,
                                current_id,
                                return_type,
                                lines,
                                name_handler,
                                function_holder,
                                break_label.clone(),
                            )?;
                            i += 1;
                        }
                        BasicSymbol::Keyword(Keyword::Else) => {
                            ended = true;
                            i += 1;
                            if line.len() <= i {
                                return Err(ProcessorError::IfElifElseNoBraces(
                                    line[i - 1].1.clone(),
                                ));
                            }
                            let BasicSymbol::BracedSection(inner) = &line[i].0 else {
                                return Err(ProcessorError::IfElifElseNoBraces(line[i].1.clone()));
                            };
                            process_lines(
                                inner,
                                current_id,
                                return_type,
                                lines,
                                name_handler,
                                function_holder,
                                break_label.clone(),
                            )?;
                            i += 1;
                        }
                        _ => return Err(ProcessorError::ElseMoreAfterBraces(line[i].1.clone())),
                    }
                }

                lines.push(Line::InlineAsm(vec![
                    format!("{}:", next_label),
                    format!("{}:", end_label),
                ]));
            }
            BasicSymbol::Keyword(Keyword::Elif | Keyword::Else) => {
                return Err(ProcessorError::RawElifElse(line[0].1.clone()))
            }
            BasicSymbol::Keyword(Keyword::Break) => {
                if line.len() > 1 {
                    return Err(ProcessorError::BreakLineNotEmpty(line[1].1.clone()));
                }
                let Some(break_label) = &break_label else {
                    return Err(ProcessorError::NothingToBreak(line[0].1.clone()));
                };
                lines.push(Line::InlineAsm(vec![format!("jmp {}", break_label)]));
            }
            _ => {
                evaluate::evaluate(line, lines, name_handler, function_holder, None)?;
            }
        };
    }

    Ok(last_return)
}
