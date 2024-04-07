use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::base::{AssignmentToken, IfToken, InitialisationToken, WhileToken};
use crate::root::nom_parser::parse_function::parse_break::{BreakToken, parse_break};
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};

#[derive(Debug)]
pub enum LineTokens {
    Initialisation(InitialisationToken),
    Assignment(AssignmentToken),
    If(IfToken),
    While(WhileToken),
    Return(ReturnToken),
    Break(BreakToken),
    NoOp(EvaluableToken)
}

pub fn parse_line(s: Span) -> ParseResult<Span, LineTokens> {
    alt((
        |x| parse_break(x).map(|(s, b)| (s, LineTokens::Break(b))),
        |x| parse_evaluable(x, true).map(|(s, e)| (s, LineTokens::NoOp(e))),
    ))
        .parse(s)
}
