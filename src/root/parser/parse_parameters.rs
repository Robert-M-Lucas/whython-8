use crate::root::parser::parse::{Location, ParseResult, Span};
use nom::character::complete::char;
use crate::root::parser::parse_function::parse_evaluable::{FullNameWithIndirectionToken, parse_full_name};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_util::discard_ignored;

pub type Parameters = Vec<(SimpleNameToken, FullNameWithIndirectionToken)>;

pub fn parse_parameters<'a>(s: Span<'a>, mut allow_self: Option<&SimpleNameToken>) -> ParseResult<'a, (), (Parameters, bool)> {
    let (mut s, _) = discard_ignored(s)?;

    let mut parameters = Vec::new();

    let mut has_self = false;

    while !s.is_empty() {
        let (ns, name) = parse_simple_name(s)?;

        let (ns, p_type) = if allow_self.is_some() && parameters.is_empty() && *name.name() == "self" {
            has_self = true;
            let s = allow_self.take().unwrap();
            (ns, FullNameWithIndirectionToken::from_simple(s.clone(), Some(s.clone()), name.location().clone()))
        }
        else {
            let (ns, _) = discard_ignored(ns)?;
            let (ns, _) = char(':')(ns)?;
            let (ns, _) = discard_ignored(ns)?;
            let t_location = Location::from_span(&ns);
            let (ns, type_name_token) = parse_full_name(ns, allow_self)?;
            let (ns, _) = discard_ignored(ns)?;
            (ns, type_name_token)
        };

        parameters.push((name, p_type));

        if ns.is_empty() {
            break;
        }

        let (ns, _) = char(',')(ns)?;
        let (ns, _) = discard_ignored(ns)?;
        s = ns;
    }

    Ok(((), (parameters, has_self)))
}
