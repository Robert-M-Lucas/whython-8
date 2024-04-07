use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::{parse_util};
use crate::root::nom_parser::parse_fn::{FunctionToken, parse_function};
use crate::root::nom_parser::parse_impl::{ImplToken, parse_impl};
use crate::root::nom_parser::parse_struct::{parse_struct, StructToken};

#[derive(Debug)]
pub enum TopLevelTokens {
    Struct(StructToken),
    Impl(ImplToken),
    Function(FunctionToken),
}

pub fn parse_toplevel(s: Span) -> ParseResult<Span, Vec<TopLevelTokens>> {
    let mut s = s;
    let mut tokens = Vec::new();

    loop {
        let ns = s;
        let (ns, _) = parse_util::discard_ignored(ns);

        if ns.is_empty() {
            return Ok((ns, tokens))
        }

        let (ns, token) = alt((
            |x| parse_impl(x).map(|(s, i)| (s, TopLevelTokens::Impl(i))),
            |x| parse_function(x).map(|(s, f)| (s, TopLevelTokens::Function(f))),
            |x| parse_struct(x).map(|(s, stru)| (s, TopLevelTokens::Struct(stru))),
        ))
            .parse(ns)?;

        tokens.push(token);

        s = ns;
    }
}
