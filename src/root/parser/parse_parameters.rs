use nom::character::complete::char;

use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{
    parse_full_name, UnresolvedTypeRefToken,
};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::Indirection;

pub type Parameters = Vec<(SimpleNameToken, UnresolvedTypeRefToken)>;

#[derive(Debug, Copy, Clone)]
pub enum SelfType {
    None,
    CopySelf,
    RefSelf,
}

impl SelfType {
    pub fn uses_self(&self) -> bool {
        !matches!(&self, SelfType::None)
    }
}

pub fn parse_parameters<'a>(
    s: Span<'a>,
    mut allow_self: Option<&SimpleNameToken>,
) -> ParseResult<'a, (), (Parameters, SelfType)> {
    let (mut s, _) = discard_ignored(s)?;

    let mut parameters = Vec::new();

    let mut has_self = SelfType::None;
    let mut has_ref = false;

    while !s.is_empty() {
        let ns = if parameters.is_empty() {
            if let Ok((ns, _)) = char::<Span, ErrorTree>('&')(s) {
                has_ref = true;
                ns
            } else {
                s
            }
        } else {
            s
        };

        let (ns, name) = parse_simple_name(ns)?;

        let (ns, p_type) =
            if allow_self.is_some() && parameters.is_empty() && *name.name() == "self" {
                has_self = if has_ref {
                    SelfType::RefSelf
                } else {
                    SelfType::CopySelf
                };
                let s = allow_self.take().unwrap();
                let i = if has_ref {
                    Indirection(1)
                } else {
                    Indirection(0)
                };
                let p_type = UnresolvedTypeRefToken::from_simple_with_indirection(
                    s.clone(),
                    Some(s.clone()),
                    name.location().clone(),
                    i,
                );
                (ns, p_type)
            } else {
                let (ns, _) = discard_ignored(ns)?;
                let (ns, _) = char(':')(ns)?;
                let (ns, _) = discard_ignored(ns)?;
                // let t_location = Location::from_span(&ns);
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
