use nom_supreme::tag::complete::tag;
use nom::bytes::complete::{is_not, take_until};
use nom::sequence::{pair, Tuple};
use nom::Parser;
use crate::root::nom_parser::parse::{ParseResult, Span};

fn peol_comment(s: Span) -> ParseResult
{
    pair(tag("//"), is_not("\n\r")).parse(s)
        .map(|(s, (_, y)): (Span, (Span, Span))| (s, y))
}

fn pinline_comment(s: Span) -> ParseResult {
    (
        tag("/*"),
        take_until("*/"),
        tag("*/")
    ).parse(s)
        .map(|(s, (_, y, _)): (Span, (Span, Span, Span))| (s, y))
}

pub fn parse_comment(s: Span) -> ParseResult {
    pinline_comment(s).or_else(|_| peol_comment(s))
}
