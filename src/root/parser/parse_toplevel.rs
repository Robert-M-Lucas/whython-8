use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_function::{test_parse_function, FunctionToken};
use crate::root::parser::parse_impl::{test_parse_impl, ImplToken};
use crate::root::parser::parse_struct::{test_parse_struct, StructToken};
use crate::root::parser::parse_util::discard_ignored;
use nom::branch::alt;
use nom::Parser;

#[derive(Debug)]
pub enum TopLevelTokens {
    Struct(StructToken),
    Impl(ImplToken),
    Function(FunctionToken),
}

pub type ToplevelTestFn<'a> = fn(Span<'a>) -> ParseResult<Span<'a>, TopLevelTokens>;

/// Parse a file into tokens (with imports removed)
pub fn parse_toplevel(s: Span) -> ParseResult<Span, Vec<TopLevelTokens>> {
    let mut s = s;
    let mut tokens = Vec::new();

    loop {
        let ns = s;

        let (ns, _) = discard_ignored(ns)?;

        if ns.is_empty() {
            return Ok((ns, tokens));
        }

        // Parse either a struct, impl, or function
        let (_, parse_fn) =
            alt((test_parse_struct, test_parse_impl, test_parse_function)).parse(ns)?;

        let (ns, token) = parse_fn(ns)?;

        tokens.push(token);

        s = ns;
    }
}
