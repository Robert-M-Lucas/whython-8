use nom::character::complete::multispace0;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_blocks::bracketed_section;
use crate::root::nom_parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::nom_parser::parse_function::parse_line::{parse_lines, LineTestFn, LineTokens};

#[derive(Debug)]
pub struct IfToken {
    location: Location,
    if_condition: EvaluableToken,
    if_contents: Vec<LineTokens>,
    elif_condition_contents: Vec<(EvaluableToken, Vec<LineTokens>)>,
    else_contents: Option<Vec<LineTokens>>,
}

pub fn test_parse_if<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match tag("if")(s) {
        Ok(_) => Ok((s, |x| parse_if(x).map(|(s, x)| (s, LineTokens::If(x))))),
        Err(e) => Err(e),
    }
}

pub fn parse_if(s: Span) -> ParseResult<Span, IfToken> {
    let (s, l) = tag("if")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = bracketed_section(s)?;
    let (_, if_condition) = parse_evaluable(content, false)?;
    let (s, _) = multispace0(s)?;
    let (s, contents) = bracketed_section(s)?;
    let (_, if_contents) = parse_lines(contents)?;

    todo!()
}
