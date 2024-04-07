use nom::character::complete::{multispace1, satisfy};
use nom::character::streaming::multispace0;
use nom::Parser;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_blocks::bracketed_section;
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens, parse_lines};

#[derive(Debug)]
pub struct WhileToken {
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens>
}

pub fn test_parse_while<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match (tag("while"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| parse_while(x).map(|(s, x)| (s, LineTokens::While(x))))),
        Err(e) => Err(e)
    }
}

pub fn parse_while(s: Span) -> ParseResult<Span, WhileToken> {
    let (s, l) = tag("while")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = bracketed_section(s)?;
    let (_, condition) = parse_evaluable(content, false)?;
    let (s, _) = multispace0(s)?;
    let (s, contents) = bracketed_section(s)?;

    let (_, lines) = parse_lines(contents)?;

    Ok((s, WhileToken {
        location: Location::from_span(l),
        condition,
        contents: lines,
    }))
}
