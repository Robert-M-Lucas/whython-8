use crate::root::ast::literals::Literal;
use crate::root::ast::operators::Operator;
use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::compile_functions::instantiate_literal::instantiate_variable;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{FunctionHolder, Line};
use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::parser::line_info::LineInfo;
use either::{Left, Right};

pub fn evaluate_operator(symbol: &(BasicSymbol, LineInfo)) -> Result<&Operator, ProcessorError> {
    match &symbol.0 {
        BasicSymbol::Operator(operator) => Ok(operator),
        _ => Err(ProcessorError::BadEvaluableLayout(symbol.1.clone())),
    }
}

pub fn evaluate_operation(
    lhs: LocalVariable,
    op: (&Operator, &LineInfo),
    rhs: Option<LocalVariable>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<LocalVariable>,
) -> Result<Option<LocalVariable>, ProcessorError> {
    Ok(Some(match &op.0 {
        Operator::Not => {
            let func = function_holder
                .get_function(Some(lhs.type_info), "not")
                .ok_or(ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(lhs.type_info.reference_depth)
                        .to_string(),
                ))?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();
            if func_args.len() != 1 {
                return Err(ProcessorError::SingleOpFunctionNotFound(
                    op.1.clone(),
                    "not".to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(lhs.type_info.reference_depth)
                        .to_string(),
                ));
            }
            let output = if let Some(return_into) = return_into {
                return_into
            } else {
                instantiate_variable(
                    Right(
                        func.get_return_type()
                            .ok_or(ProcessorError::SingleOpFunctionNotFound(
                                op.1.clone(),
                                "not".to_string(),
                                name_handler
                                    .type_table()
                                    .get_type(lhs.type_info.type_id)
                                    .unwrap()
                                    .get_indirect_name(lhs.type_info.reference_depth)
                                    .to_string(),
                            ))?,
                    ),
                    lines,
                    name_handler,
                    function_holder,
                    None,
                )?
            };
            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(
                    func.get_inline(vec![lhs.offset, output.offset]),
                ));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    -(name_handler.local_variable_space() as isize),
                    vec![(
                        lhs.offset,
                        name_handler.type_table().get_type_size(lhs.type_info)?,
                    )],
                    name_handler.type_table().get_type_size(output.type_info)?,
                    output.offset,
                ));
            }
            output
        }
        op_ => {
            if matches!(op_, Operator::And) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.type_info.type_id != lhs.type_info.type_id
                        || return_into.type_info.reference_depth
                            != lhs.type_info.reference_depth + 1
                    {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(return_into.type_info.reference_depth)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(lhs.type_info.reference_depth + 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    LocalVariable::new(
                        name_handler.add_local_variable(
                            None,
                            TypeInfo::new(lhs.type_info.type_id, lhs.type_info.reference_depth + 1),
                            lines,
                        )?,
                        lhs.type_info.type_id,
                        lhs.type_info.reference_depth + 1,
                    )
                };
                lines.push(Line::InlineAsm(Int::instantiate_local_ref(
                    lhs.offset,
                    return_into.offset,
                )));
                return Ok(Some(return_into));
            }

            // Get ref
            if matches!(op_, Operator::Product) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if lhs.type_info.reference_depth == 0 {
                        return Err(ProcessorError::CantDerefNonRef(op.1.clone()));
                    }
                    if return_into.type_info.type_id != lhs.type_info.type_id
                        || return_into.type_info.reference_depth
                            != lhs.type_info.reference_depth - 1
                    {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(return_into.type_info.reference_depth)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(lhs.type_info.reference_depth - 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    LocalVariable::new(
                        name_handler.add_local_variable(
                            None,
                            TypeInfo::new(lhs.type_info.type_id, lhs.type_info.reference_depth + 1),
                            lines,
                        )?,
                        lhs.type_info.type_id,
                        lhs.type_info.reference_depth - 1,
                    )
                };

                lines.push(Line::DynFromCopy(
                    lhs.offset,
                    return_into.offset,
                    name_handler
                        .type_table()
                        .get_type_size(return_into.type_info)?,
                ));
                return Ok(Some(return_into));
            }

            // Heap alloc
            if matches!(op_, Operator::HeapAlloc) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.type_info.type_id != lhs.type_info.type_id
                        || return_into.type_info.reference_depth
                            != lhs.type_info.reference_depth + 1
                    {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(return_into.type_info.reference_depth)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(lhs.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(lhs.type_info.reference_depth + 1)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    LocalVariable::new(
                        name_handler.add_local_variable(
                            None,
                            TypeInfo::new(lhs.type_info.type_id, lhs.type_info.reference_depth + 1),
                            lines,
                        )?,
                        lhs.type_info.type_id,
                        lhs.type_info.reference_depth + 1,
                    )
                };
                let size = name_handler.type_table().get_type_size(lhs.type_info)?;
                lines.push(Line::HeapAlloc(size, return_into.offset));
                lines.push(Line::DynToCopy(lhs.offset, return_into.offset, size));
                return Ok(Some(return_into));
            }

            // Heap dealloc
            if matches!(op_, Operator::HeapDealloc) && rhs.is_none() {
                let return_into = if let Some(return_into) = return_into {
                    if return_into.type_info != TypeInfo::new(Bool::get_id(), 0) {
                        return Err(ProcessorError::BadEvaluatedType(
                            op.1.clone(),
                            name_handler
                                .type_table()
                                .get_type(return_into.type_info.type_id)
                                .unwrap()
                                .get_indirect_name(return_into.type_info.reference_depth)
                                .to_string(),
                            name_handler
                                .type_table()
                                .get_type(Bool::get_id())
                                .unwrap()
                                .get_indirect_name(0)
                                .to_string(),
                        ));
                    }
                    return_into
                } else {
                    LocalVariable::new(
                        name_handler.add_local_variable(
                            None,
                            TypeInfo::new(Bool::get_id(), 0),
                            lines,
                        )?,
                        Bool::get_id(),
                        0,
                    )
                };

                if lhs.type_info.reference_depth == 0 {
                    return Err(ProcessorError::CantDeallocateNonRef(op.1.clone()));
                }

                lines.push(Line::HeapDealloc(lhs.offset, return_into.offset));
                return Ok(Some(return_into));
            }

            // Negate int
            let (lhs, rhs) = if matches!(op_, Operator::Subtract)
                && rhs.is_none()
                && (lhs.type_info == TypeInfo::new(Int::get_id(), 0)
                    || lhs.type_info == TypeInfo::new(Float::get_id(), 0))
            {
                (
                    instantiate_variable(
                        if lhs.type_info.type_id == Int::get_id() {
                            Left((&Literal::Int(0), &op.1))
                        } else {
                            Left((&Literal::Float(0.0), &op.1))
                        },
                        lines,
                        name_handler,
                        function_holder,
                        None,
                    )
                    .unwrap(),
                    lhs,
                )
            } else {
                (
                    lhs,
                    rhs.ok_or(ProcessorError::BadOperatorPosition(
                        op.1.clone(),
                        op.0.clone(),
                    ))?,
                )
            };

            let func_name = match op_ {
                Operator::Add => "add",
                Operator::Subtract => "sub",
                Operator::Product => "mul",
                Operator::Divide => "div",
                Operator::Modulo => "mod",
                Operator::Greater => "gt",
                Operator::Less => "lt",
                Operator::GreaterEqual => "ge",
                Operator::LessEqual => "le",
                Operator::Equal => "eq",
                Operator::NotEqual => "ne",
                Operator::Or => "or",
                Operator::And => "and",
                Operator::HeapAlloc => "heap allocate",
                Operator::HeapDealloc => "heap deallocate",
                Operator::Not => panic!(),
            };

            // println!("{}vs{} {} {}vs{}", lhs.1.0, Float::get_id(), func_name, rhs.1.1, Float::get_id());

            let func = function_holder
                .get_function(Some(lhs.type_info), func_name)
                .ok_or(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(lhs.type_info.reference_depth)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(rhs.type_info.reference_depth)
                        .to_string(),
                ))?;
            name_handler.use_function(func);
            let func_args = func.get_args();
            let func_id = func.get_id();

            if func_args.len() != 2 || func_args[1].1 != rhs.type_info {
                return Err(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(lhs.type_info.reference_depth)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(rhs.type_info.reference_depth)
                        .to_string(),
                ));
            }

            let ret_type = func
                .get_return_type()
                .ok_or(ProcessorError::OpFunctionNotFound(
                    op.1.clone(),
                    func_name.to_string(),
                    name_handler
                        .type_table()
                        .get_type(lhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(lhs.type_info.reference_depth)
                        .to_string(),
                    name_handler
                        .type_table()
                        .get_type(rhs.type_info.type_id)
                        .unwrap()
                        .get_indirect_name(rhs.type_info.reference_depth)
                        .to_string(),
                ))?;

            let output = if let Some(return_into) = return_into {
                if return_into.type_info != ret_type {
                    return Err(ProcessorError::BadEvaluatedType(
                        op.1.clone(),
                        name_handler
                            .type_table()
                            .get_type(return_into.type_info.type_id)
                            .unwrap()
                            .get_indirect_name(return_into.type_info.reference_depth)
                            .to_string(),
                        name_handler
                            .type_table()
                            .get_type(ret_type.type_id)
                            .unwrap()
                            .get_indirect_name(ret_type.reference_depth)
                            .to_string(),
                    ));
                }
                return_into
            } else {
                instantiate_variable(Right(ret_type), lines, name_handler, function_holder, None)?
            };

            let func = function_holder.functions().get(&func_id).unwrap();
            if func.is_inline() {
                lines.push(Line::InlineAsm(func.get_inline(vec![
                    lhs.offset,
                    rhs.offset,
                    output.offset,
                ])));
            } else {
                lines.push(Line::ReturnCall(
                    func.get_id(),
                    -(name_handler.local_variable_space() as isize),
                    vec![
                        (
                            lhs.offset,
                            name_handler.type_table().get_type_size(lhs.type_info)?,
                        ),
                        (
                            rhs.offset,
                            name_handler.type_table().get_type_size(rhs.type_info)?,
                        ),
                    ],
                    name_handler.type_table().get_type_size(output.type_info)?,
                    output.offset,
                ));
            }
            output
        }
    }))
}
