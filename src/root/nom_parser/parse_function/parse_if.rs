use itertools::multipeek;
use nom::character::complete::{multispace0, multispace1};
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::bracketed_section;
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens, parse_lines};

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

    let mut elifs = Vec::new();
    let mut s = s;

    loop {
        let (ns, _) = multispace0(s)?;

        if ns.is_empty() { break; }

        let ns = if let Ok((ns, _)) = tag("else") {
            ns
        }
        else {
            s = ns;
            break;
        };

        let (ns, condition) = if let Ok((ns, _)) = (multispace1, tag("if")).parse(ns) {
            let (ns, _) = multispace0(ns)?;
            let (ns, content) = bracketed_section(ns)?;
            let (_, condition) = parse_evaluable(content, false)?;
            (ns, Some(condition))
        }
        else {
            (ns, None)
        };

        let (ns, _) = multispace0(ns)?;

        let (ns, contents) = bracketed_section(ns)?;
        let (_, contents) = parse_lines(contents)?;

        // ? Handle else if
        if let Some(condition) = condition {
            elifs.push((condition, contents));
        }
        else { // ? Handle else
            return Ok((ns, IfToken {
                location: Location::from_span(l),
                if_condition,
                if_contents,
                elif_condition_contents: elifs,
                else_contents: Some(contents),
            }))
        }

        s = ns;
    }

    // ? Ended without else
    return Ok((s, IfToken {
        location: Location::from_span(l),
        if_condition,
        if_contents,
        elif_condition_contents: elifs,
        else_contents: None,
    }))
}
