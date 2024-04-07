use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::base::{AssignmentToken, EvaluableToken, IfToken, InitialisationToken, WhileToken};
use crate::root::nom_parser::parse_function::parse_function;
use crate::root::nom_parser::parse_impl::parse_impl;
use crate::root::nom_parser::parse_struct::parse_struct;
use crate::root::nom_parser::parse_toplevel::TopLevelTokens;

#[derive(Debug)]
pub enum LineTokens {
    Initialisation(InitialisationToken),
    Assignment(AssignmentToken),
    If(IfToken),
    While(WhileToken),
    Return(String),
    Break(Location),
    NoOp(EvaluableToken)
}

pub fn parse_line(s: Span) -> ParseResult<Span, LineTokens> {
    alt((
        |x| Ok((x, LineTokens::Break(Location::from_span(x)))),
        |x| Ok((x, LineTokens::Break(Location::from_span(x)))),
    ))
        .parse(s)
}
