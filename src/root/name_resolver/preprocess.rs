use crate::root::ast::keywords::Keyword;
use crate::root::basic_ast::punctuation::Punctuation;
use crate::root::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol, NameType};

use crate::root::parser::line_info::LineInfo;
use crate::root::name_resolver::processor::ProcessorError;

use std::vec::IntoIter;

pub type PreProcessFunction = (
    String,
    Vec<(String, LineInfo, String, usize, LineInfo)>,
    Option<((String, usize), LineInfo)>,
    Vec<(BasicSymbol, LineInfo)>,
);

#[derive(Clone, strum_macros::Display, Debug)]
pub enum PreprocessSymbol {
    Struct(
        LineInfo,
        String,
        Vec<(String, LineInfo, (String, usize), LineInfo)>,
    ),
    Impl(LineInfo, String, Vec<(PreProcessFunction, LineInfo)>),
    Fn(LineInfo, PreProcessFunction),
}

pub fn preprocess(
    ast: Vec<BasicAbstractSyntaxTree>,
) -> Result<Vec<PreprocessSymbol>, ProcessorError> {
    let mut output = Vec::new();

    for tree in ast {
        let mut tree = tree.into_iter();
        loop {
            let next = tree.next();
            if next.is_none() {
                break;
            }
            let (first_symbol, first_line) = next.unwrap();

            match first_symbol {
                BasicSymbol::Keyword(keyword) => match keyword {
                    Keyword::Struct => {
                        output.push(parse_struct(first_line, &mut tree)?);
                    }
                    Keyword::Impl => {
                        output.push(parse_impl(first_line, &mut tree)?);
                    }
                    Keyword::Fn => {
                        output.push(parse_fn(first_line, &mut tree, None)?);
                    }
                    _ => {}
                },
                BasicSymbol::AbstractSyntaxTree(_) => panic!(),
                _symbol => return Err(ProcessorError::BadTopLevelSymbol(first_line)),
            }
        }
    }

    Ok(output)
}

fn parse_struct(
    start_line_info: LineInfo,
    tree: &mut IntoIter<(BasicSymbol, LineInfo)>,
) -> Result<PreprocessSymbol, ProcessorError> {
    let (name, name_line) = tree
        .next()
        .ok_or(ProcessorError::StructNoName(start_line_info.clone()))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(ProcessorError::StructNoName(name_line)),
    };

    if name.len() > 1 {
        return Err(ProcessorError::MultipartNameDef(name_line));
    }
    let name = name.remove(0);

    let (contents, contents_line) = tree
        .next()
        .ok_or(ProcessorError::StructNoBraces(name_line))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(ProcessorError::StructNoBraces(contents_line)),
    };
    let mut contents = contents.into_iter();

    let mut attributes = Vec::new();
    let mut first = true;

    loop {
        let mut first_item = contents.next();
        if first_item.is_none() {
            break;
        }

        if !first {
            let (tmp_first_item, tmp_line) = first_item.unwrap();
            if !matches!(
                tmp_first_item,
                BasicSymbol::Punctuation(Punctuation::ListSeparator)
            ) {
                return Err(ProcessorError::StructNoAttrSeparator(tmp_line));
            }
            first_item = contents.next();
            if first_item.is_none() {
                break;
            }
        }

        let (attr_name, attr_name_line) = first_item.unwrap();
        let attr_name = match attr_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(ProcessorError::MultipartNameDef(attr_name_line));
                }
                name.remove(0)
            }
            _ => return Err(ProcessorError::StructExpectedAttributeName(attr_name_line)),
        };
        if attr_name.3 != 0 {
            return Err(ProcessorError::NameWithRefPrefix(attr_name_line));
        }
        let Some((colon, colon_line)) = contents.next() else {
            return Err(ProcessorError::NameTypeNotDefined(attr_name_line));
        };
        if !matches!(colon, BasicSymbol::Punctuation(Punctuation::Colon)) {
            return Err(ProcessorError::NameTypeNotDefined(colon_line));
        }

        let Some((attr_type, attr_type_line)) = contents.next() else {
            return Err(ProcessorError::NameTypeNotDefined(colon_line));
        };
        let attr_type = match attr_type {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(ProcessorError::MultipartNameDef(attr_type_line));
                }
                name.remove(0)
            }
            _ => return Err(ProcessorError::NameTypeNotDefined(attr_type_line)),
        };

        attributes.push((
            attr_name.0,
            attr_name_line,
            (attr_type.0, attr_type.3),
            attr_type_line,
        ));
        first = false;
    }

    Ok(PreprocessSymbol::Struct(
        start_line_info,
        name.0,
        attributes,
    ))
}

fn parse_impl(
    start_line_info: LineInfo,
    tree: &mut IntoIter<(BasicSymbol, LineInfo)>,
) -> Result<PreprocessSymbol, ProcessorError> {
    let (name, name_line) = tree
        .next()
        .ok_or(ProcessorError::ImplNoName(start_line_info.clone()))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(ProcessorError::ImplNoName(name_line)),
    };

    if name.len() > 1 {
        return Err(ProcessorError::MultipartTypeName(name_line));
    }
    let name = name.remove(0);

    let (contents, contents_line) = tree.next().ok_or(ProcessorError::ImplNoBraces(name_line))?;
    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(ProcessorError::ImplNoBraces(contents_line)),
    };
    let mut contents = contents.into_iter();

    let mut functions = Vec::new();

    loop {
        let symbol = contents.next();
        if symbol.is_none() {
            break;
        }
        let (symbol, symbol_line) = symbol.unwrap();
        match symbol {
            BasicSymbol::Keyword(Keyword::Fn) => {
                let function =
                    parse_fn(start_line_info.clone(), &mut contents, Some(name.0.clone()))?;
                let function = match function {
                    PreprocessSymbol::Fn(_, function) => function,
                    _ => panic!(),
                };
                functions.push((function, symbol_line));
            }
            _ => return Err(ProcessorError::ImplNonFnContent(symbol_line)),
        }
    }

    Ok(PreprocessSymbol::Impl(start_line_info, name.0, functions))
}

fn parse_fn(
    start_line_info: LineInfo,
    tree: &mut IntoIter<(BasicSymbol, LineInfo)>,
    mut self_type: Option<String>,
) -> Result<PreprocessSymbol, ProcessorError> {
    let (name, name_line) = tree
        .next()
        .ok_or(ProcessorError::FnNoName(start_line_info.clone()))?;
    let mut name = match name {
        BasicSymbol::Name(name) => name,
        _ => return Err(ProcessorError::FnNoName(name_line)),
    };

    if name.len() > 1 {
        return Err(ProcessorError::MultipartNameDef(name_line));
    }
    let (name, _, name_type, indirection) = name.remove(0);
    if indirection != 0 {
        return Err(ProcessorError::NameWithRefPrefix(name_line));
    }

    let parameters = match name_type {
        NameType::Normal => return Err(ProcessorError::FnNoBraces(name_line)),
        NameType::Function(arguments) => arguments,
    };

    // let (arguments, arguments_line) = tree.next().ok_or()?;
    // let arguments = match arguments {
    //     BasicSymbol::BracketedSection(contents) => contents,
    //     _ => {
    //         return Err(Syntax(
    //             path,
    //             arguments_line,
    //             "function name must be followed by brackets ('()')".to_string(),
    //         ))
    //     }
    // };

    let mut parameters_processed = Vec::new();
    let mut last_line = name_line;

    for parameter in parameters {
        let mut parameter = parameter.into_iter();

        let Some((first_item, first_line)) = parameter.next() else {
            return Err(ProcessorError::FnParamsTrailingComma(last_line));
        };

        let arg_name = first_item;
        let arg_line = first_line;
        let arg_name = match arg_name {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(ProcessorError::MultipartNameDef(arg_line));
                }
                name.remove(0).0
            }
            _ => return Err(ProcessorError::FnExpectedParameterName(arg_line)),
        };
        let (param_type, param_type_line) = if arg_name == "self" {
            if self_type.is_none() {
                return Err(ProcessorError::FnBadSelf(arg_line));
            }

            ((self_type.take().unwrap(), 1), arg_line.clone())
        } else {
            let Some((colon, colon_line)) = parameter.next() else {
                return Err(ProcessorError::NameTypeNotDefined(arg_line));
            };

            if !matches!(colon, BasicSymbol::Punctuation(Punctuation::Colon)) {
                return Err(ProcessorError::NameTypeNotDefined(colon_line));
            }

            let Some((param_type, param_type_line)) = parameter.next() else {
                return Err(ProcessorError::NameTypeNotDefined(colon_line));
            };
            let param_type = match param_type {
                BasicSymbol::Name(mut name) => {
                    if name.len() > 1 {
                        return Err(ProcessorError::MultipartTypeName(param_type_line));
                    }
                    let part = name.remove(0);
                    (part.0, part.3)
                }
                _ => return Err(ProcessorError::NameTypeNotDefined(param_type_line)),
            };
            (param_type, param_type_line)
        };

        parameters_processed.push((
            arg_name,
            arg_line,
            param_type.0,
            param_type.1,
            param_type_line.clone(),
        )); // TODO:
        last_line = param_type_line;
    }

    let (mut contents, mut contents_line) = tree
        .next()
        .ok_or(ProcessorError::FnNoBracesOrReturn(last_line))?;

    let return_type = if matches!(&contents, BasicSymbol::Punctuation(Punctuation::Tilda)) {
        let Some((next_symbol, next_line)) = tree.next() else {
            return Err(ProcessorError::FnExpectedReturnType(contents_line));
        };
        contents_line = next_line.clone();
        match next_symbol {
            BasicSymbol::Name(mut name) => {
                if name.len() > 1 {
                    return Err(ProcessorError::MultipartTypeName(next_line));
                }
                Some((name.remove(0), next_line))
            }
            _ => return Err(ProcessorError::FnExpectedReturnType(next_line)),
        }
    } else {
        None
    };

    if return_type.is_some() {
        (contents, contents_line) = tree
            .next()
            .ok_or(ProcessorError::FnNoBraces(contents_line))?;
    }

    let contents = match contents {
        BasicSymbol::BracedSection(contents) => contents,
        _ => return Err(ProcessorError::FnNoBraces(contents_line)),
    };

    Ok(PreprocessSymbol::Fn(
        start_line_info,
        (
            name,
            parameters_processed,
            return_type.map(|x| ((x.0 .0, x.0 .3), x.1)),
            contents,
        ),
    ))
}
