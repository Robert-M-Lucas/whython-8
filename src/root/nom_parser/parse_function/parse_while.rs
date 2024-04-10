use nom::character::complete::{multispace0, multispace1, satisfy};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::bracketed_section;
use crate::root::nom_parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::nom_parser::parse_function::parse_line::{parse_lines, LineTestFn, LineTokens};

#[derive(Debug)]
pub struct WhileToken {
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens>,
}

pub fn test_parse_while<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match (tag("while"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_while(x).map(|(s, x)| (s, LineTokens::While(x)))
        })),
        Err(e) => Err(e),
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

    Ok((
        s,
        WhileToken {
            location: Location::from_span(l),
            condition,
            contents: lines,
        },
    ))
}
