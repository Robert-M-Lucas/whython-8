use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::parse_name::{parse_full_name, parse_simple_name, NameToken};
use nom::character::complete::{char};
use crate::root::nom_parser::parse_util::discard_ignored;

pub type Parameters = Vec<(String, NameToken)>;

pub fn parse_parameters(s: Span, mut allow_self: Option<NameToken>) -> ParseResult<(), Parameters> {
    let (mut s, _) = discard_ignored(s)?;

    let mut parameters = Vec::new();

    while !s.is_empty() {
        let (ns, name) = parse_simple_name(s)?;

        let (ns, type_name_token) = if allow_self.is_some() && parameters.is_empty() && *name.fragment() == "self" {
            (ns, allow_self.take().unwrap())
        }
        else {
            let (ns, _) = discard_ignored(ns)?;
            let (ns, _) = char(':')(ns)?;
            let (ns, _) = discard_ignored(ns)?;
            let (ns, type_name_token) = parse_full_name(ns)?;
            let (ns, _) = discard_ignored(ns)?;
            (ns, type_name_token)
        };

        parameters.push((name.to_string(), type_name_token));

        if ns.is_empty() {
            break;
        }

        let (ns, _) = char(',')(ns)?;
        let (ns, _) = discard_ignored(ns)?;
        s = ns;
    }

    Ok(((), parameters))
}
