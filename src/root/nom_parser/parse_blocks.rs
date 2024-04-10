use crate::root::nom_parser::parse::{ParseResult, Span};
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

// ! BROKEN

pub fn braced_section(s: Span) -> ParseResult {
    (char('{'), take_until("}"), char('}'))
        .parse(s)
        .map(|(s, (_, y, _)): (Span, (char, Span, char))| (s, y))
}

pub fn bracketed_section(s: Span) -> ParseResult {
    (char('('), take_until(")"), char(')'))
        .parse(s)
        .map(|(s, (_, y, _)): (Span, (char, Span, char))| (s, y))
}
