use nom::Err::Error;
use nom::error::{ErrorKind, ParseError};
use nom::Parser;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::tag::TagError;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, ErrorTree};


const OPERATOR_MAPS: [(&str, OperatorTokens, bool); 3] = [
    ("+", OperatorTokens::Add, true),
    ("-", OperatorTokens::Subtract, true),
    ("!", OperatorTokens::Not, false)
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
    for (operator, token, _) in OPERATOR_MAPS {
        if let Ok((s, x)) = tag::<_, _, ErrorTree>(operator)(s) {
            return Ok((s, OperatorToken { location: Location::from_span(x), operator: token }))
        }
    }

    Err(Error(
        GenericErrorTree::Alt(
            OPERATOR_MAPS.iter().map(|(t, _, _)| GenericErrorTree::from_tag(s, *t)).collect()
        )
    ))
}
