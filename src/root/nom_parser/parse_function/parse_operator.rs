use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::Err::Error;
use nom::Parser;
use nom_supreme::error::GenericErrorTree;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};

const OPERATOR_MAPS: [(&str, OperatorTokens); 3] = [
    ("+", OperatorTokens::Add),
    ("-", OperatorTokens::Subtract),
    ("!", OperatorTokens::Not)
];

#[derive(Debug)]
pub struct OperatorToken {
    location: Location,
    operator: OperatorTokens,
}

#[derive(Debug)]
pub enum OperatorTokens {
    Add,
    Subtract,
    Not
}

pub fn parse_operator(s: Span) -> ParseResult<Span, OperatorToken> {
    for (operator, token) in OPERATOR_MAPS {
        if let Ok((s, x)) = tag(operator)(s) {
            return Ok((s, OperatorToken { location: Location::from_span(x), operator: token }))
        }
    }


    todo!()
}
