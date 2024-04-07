use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::parse_name::{parse_full_name, parse_simple_name, NameToken};
use nom::character::complete::{char, multispace0};

pub type Parameters = Vec<(String, NameToken)>;

pub fn parse_parameters(s: Span) -> ParseResult<(), Parameters> {
    let (mut s, _) = multispace0(s)?;

    let mut parameters = Vec::new();

    while !s.is_empty() {
        let (ns, name) = parse_simple_name(s)?;
        let (ns, _) = multispace0(ns)?;
        let (ns, _) = char(':')(ns)?;
        let (ns, _) = multispace0(ns)?;
        let (ns, name_token) = parse_full_name(ns)?;
        let (ns, _) = multispace0(ns)?;

        parameters.push((name.to_string(), name_token));

        if ns.is_empty() {
            break;
        }

        let (ns, _) = char(',')(ns)?;
        let (ns, _) = multispace0(ns)?;
        s = ns;
    }

    Ok(((), parameters))
}
