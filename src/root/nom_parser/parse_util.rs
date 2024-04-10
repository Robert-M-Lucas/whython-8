use crate::root::nom_parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::nom_parser::parse_comments;
use nom::bytes::complete::{take_till, take_while};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::Tuple;
use nom::{IResult, InputTakeAtPosition, Parser};
use nom::character::complete::multispace1;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};

pub fn take_whitespace(s: Span) -> ParseResult {
    take_while(|c: char| c.is_whitespace())(s)
}

pub fn discard_ignored(s: Span) -> ParseResult<Span, ()> {
    let mut s = s;
    let mut ever_found = false;
    let mut found = true;
    while found {
        found = false;
        if let Ok((ns, _)) = parse_comments::parse_comment(s) {
            s = ns;
            found = true;
            ever_found = true;
        }

        if let Ok((ns, _)) = multispace1::<_, ErrorTree>(s) {
            s = ns;
            found = true;
            ever_found = true;
        }
    }

    Ok((s, ()))
}

pub fn require_ignored(s: Span) -> ParseResult<Span, ()> {
    let mut s = s;
    let mut ever_found = false;
    let mut found = true;
    while found {
        found = false;
        if let Ok((ns, _)) = parse_comments::parse_comment(s) {
            s = ns;
            found = true;
            ever_found = true;
        }

        if let Ok((ns, _)) = multispace1::<_, ErrorTree>(s) {
            s = ns;
            found = true;
            ever_found = true;
        }
    }

    if !ever_found {
        return Err(Error(
            ErrorTree::from_error_kind(s, ErrorKind::Space)
        ))
    }

    Ok((s, ()))
}

pub fn take_till_whitespace(s: Span) -> ParseResult {
    take_till(|c: char| c.is_whitespace())(s)
}

// ? Source: https://stackoverflow.com/a/76759023/10619498
// TODO: Does this work with
pub fn alt_many<I, O, E, P, Ps>(mut parsers: Ps) -> impl Parser<I, O, E>
where
    P: Parser<I, O, E>,
    I: Clone,
    for<'a> &'a mut Ps: IntoIterator<Item = &'a mut P>,
    E: ParseError<I>,
{
    move |input: I| {
        for parser in &mut parsers {
            if let r @ Ok(_) = parser.parse(input.clone()) {
                return r;
            }
        }
        nom::combinator::fail::<I, O, E>(input)
    }
}
