use nom::sequence::Tuple;
use nom::bytes::complete::{take_till, take_while};
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom::error::ParseError;
use nom::{InputTakeAtPosition, IResult, Parser};
use crate::root::nom_parser::parse::{ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_comments;

pub fn take_whitespace(s: Span) -> ParseResult {
    take_while(|c: char| c.is_whitespace())(s)
}

// pub fn discard_ignored(s: Span) -> (Span, bool) {
//     let mut s = s;
//     let mut ever_found = false;
//     let mut found = true;
//     while found {
//         found = false;
//         if let Ok((ns, _)) = parse_comments::parse_comment(s) {
//             s = ns;
//             found = true;
//             ever_found = true;
//         }
//         if let Ok((ns, p)) = take_whitespace(s) {
//             if !p.is_empty() {
//                 s = ns;
//                 found = true;
//                 ever_found = true;
//             }
//         }
//     }
//
//     (s, ever_found)
// }

// pub fn require_ignored(s: Span) -> ParseResult<Span, ()> {
//     let (s, i) = multispace0(s)?;
//     if !i {
//         return Err(nom::Err::Error(
//             TypeErrorTree::Base {
//                 location: s,
//                 kind: BaseErrorKind::Expected(Expectation::Space),
//             }
//         ))
//     }
//     Ok((s, ()))
// }

pub fn take_till_whitespace(s: Span) -> ParseResult
{
    take_till(|c: char| c.is_whitespace())(s)
}
