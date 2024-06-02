use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_name::{parse_full_name, parse_simple_name, UnresolvedNameToken};
use nom::character::complete::{char};
use crate::root::parser::parse_util::discard_ignored;

pub type Parameters = Vec<(String, UnresolvedNameToken)>;

pub fn parse_parameters(s: Span, mut allow_self: Option<UnresolvedNameToken>) -> ParseResult<(), Parameters> {
    let (mut s, _) = discard_ignored(s)?;

    let mut parameters = Vec::new();
    let self_name = allow_self.as_ref().and_then(|s| Some(s.base().to_string()));

    while !s.is_empty() {
        let (ns, name) = parse_simple_name(s)?;

        let (ns, type_name_token) = if allow_self.is_some() && parameters.is_empty() && *name.fragment() == "self" {
            (ns, allow_self.take().unwrap())
        }
        else {
            let (ns, _) = discard_ignored(ns)?;
            let (ns, _) = char(':')(ns)?;
            let (ns, _) = discard_ignored(ns)?;
            let (ns, type_name_token) = parse_full_name(ns, self_name.clone())?;
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
