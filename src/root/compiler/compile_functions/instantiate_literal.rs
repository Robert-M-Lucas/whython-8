use crate::root::ast::literals::Literal;
use crate::root::compiler::compile_functions::evaluate::evaluate;
use crate::root::compiler::compile_functions::name_handler::NameHandler;
use crate::root::compiler::compile_functions::{FunctionHolder, Line};
use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::custom::types::int::Int;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::parser::line_info::LineInfo;
use either::{Either, Left, Right};

#[allow(dead_code)]
fn try_instantiate_literal(
    literal: Either<LocalVariable, (&Literal, &LineInfo)>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<LocalVariable>,
) -> Result<LocalVariable, ProcessorError> {
    match literal {
        Left(r) => Ok(r),
        Right(literal) => instantiate_variable(
            Left(literal),
            lines,
            name_handler,
            function_holder,
            return_into,
        ),
    }
}

pub fn instantiate_variable(
    literal: Either<(&Literal, &LineInfo), TypeInfo>,
    lines: &mut Vec<Line>,
    name_handler: &mut NameHandler,
    function_holder: &FunctionHolder,
    return_into: Option<LocalVariable>,
) -> Result<LocalVariable, ProcessorError> {
    let variable = if let Some(variable) = return_into {
        variable
    } else {
        let id = match &literal {
            Left((literal, info)) => literal.get_type_id(name_handler.type_table(), info)?,
            Right(id) => *id,
        };
        LocalVariable::from_type_info(name_handler.add_local_variable(None, id, lines)?, id)
    };

    // Indirect
    let true_id = variable.type_info.type_id;
    let indirection = variable.type_info.reference_depth;
    let id = if indirection != 0 {
        Int::get_id()
    } else {
        variable.type_info.type_id
    };
    let _type = name_handler.type_table().get_type(id).unwrap();
    let asm = match &literal {
        Left((Literal::Initialiser(name, attributes), line_info)) => {
            let line_info = *line_info;
            let _id = name_handler.type_table().get_id_by_name(name).ok_or(
                ProcessorError::TypeNotFound(line_info.clone(), name.clone()),
            )?;

            if id != _id {
                return Err(ProcessorError::BadEvaluatedType(
                    line_info.clone(),
                    _type.get_name().to_string(),
                    name.clone(),
                ));
            }

            if id < 0 {
                return Err(ProcessorError::AttemptedBuiltinInitialiser(
                    line_info.clone(),
                ));
            }

            // let size = _type.get_size(name_handler.type_table(), None)?; // ! Ensures non-circularity
            let attribute_types = _type.get_user_type().unwrap().get_attribute_types();
            if attributes.len() != attribute_types.len() {
                return Err(ProcessorError::IncorrectAttribCount(
                    line_info.clone(),
                    attribute_types.len(),
                    attributes.len(),
                ));
            }

            let mut addr_counter = variable.offset;
            for (attribute, attribute_type) in attributes.iter().zip(attribute_types) {
                evaluate(
                    attribute,
                    lines,
                    name_handler,
                    function_holder,
                    Some(LocalVariable::from_type_info(addr_counter, attribute_type)),
                )?;

                addr_counter += name_handler.type_table().get_type_size(attribute_type)? as isize;
            }

            Vec::new()
        }
        Left((literal, line_info)) => {
            if literal.get_type_id(name_handler.type_table(), line_info)? != TypeInfo::new(id, 0) {
                return Err(ProcessorError::BadLiteralType());
            }
            _type.instantiate(Some(literal), variable.offset)?
        }
        Right(_id) => _type.instantiate(None, variable.offset)?,
    };
    lines.push(Line::InlineAsm(asm));
    Ok(LocalVariable::new(variable.offset, true_id, indirection))
}
