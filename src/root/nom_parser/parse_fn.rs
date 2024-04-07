pub mod base;

use nom_supreme::tag::complete::tag;
use nom::Parser;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_parameters::Parameters;
use crate::root::nom_parser::parse_toplevel::TopLevelTokens;

#[derive(Debug)]
pub struct FunctionToken {
    location: Location,
    name: String,
    return_type: String,
    parameters: Parameters,
    // contents: Vec<LineTokens<'a>>
}

pub fn parse_function(s: Span) -> ParseResult<Span, FunctionToken> {
    todo!();

    // println!("{:?}", s);
    // tag("fn").parse(s).map(|(s, _)| (s, TopLevelTokens::Test))
}

