use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::parse_function::{parse_function, test_parse_function, FunctionToken};
use crate::root::nom_parser::parse_impl::{parse_impl, test_parse_impl, ImplToken};
use crate::root::nom_parser::parse_struct::{parse_struct, test_parse_struct, StructToken};
use crate::root::nom_parser::parse_util;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::Parser;

#[derive(Debug)]
pub enum TopLevelTokens {
    Struct(StructToken),
    Impl(ImplToken),
    Function(FunctionToken),
}

pub type ToplevelTestFn<'a> = fn(Span<'a>) -> ParseResult<Span<'a>, TopLevelTokens>;

pub fn parse_toplevel(s: Span) -> ParseResult<Span, Vec<TopLevelTokens>> {
    let mut s = s;
    let mut tokens = Vec::new();

    loop {
        let ns = s;
        let (ns, _) = multispace0(ns)?;

        if ns.is_empty() {
            return Ok((ns, tokens));
        }

        let (_, parse_fn) =
            alt((test_parse_struct, test_parse_impl, test_parse_function)).parse(ns)?;

        let (ns, token) = parse_fn(ns)?;

        tokens.push(token);

        s = ns;
    }
}
