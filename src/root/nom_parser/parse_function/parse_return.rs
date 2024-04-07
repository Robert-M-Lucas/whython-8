use nom::character::complete::char;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_break::parse_break;
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct ReturnToken {
    location: Location,
    return_value: EvaluableToken
}

pub fn test_parse_return<'a>(s: Span) -> ParseResult<Span, LineTestFn<'a>> {
    if s.len() >= 6 && s.substring(0, 6) == "return" {
        Ok((s, |x| parse_return(x).map(|(s, r)| (s, LineTokens::Return(r)))))
    }
    else {
        Err(Error(
            TypeErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(
                    Expectation::Tag("return")
                ),
            }
        ))
    }
}

pub fn parse_return(s: Span) -> ParseResult<Span, ReturnToken> {
    let (s, l) = tag("return")(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, value) = parse_evaluable(s, true)?;
    Ok((s, ReturnToken { location: Location::from_span(l), return_value: value }))
}