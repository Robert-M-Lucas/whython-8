use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;
use crate::root::nom_parser::parse::{ParseResult, Span};

pub fn braced_section(s: Span) -> ParseResult {
    (
        char('{'),
        take_until("}"),
        char('}')
    ).parse(s)
        .map(|(s, (_, y, _)): (Span, (char, Span, char))| (s, y))
}