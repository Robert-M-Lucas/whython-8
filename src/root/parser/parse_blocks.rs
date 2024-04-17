use std::collections::HashMap;
use itertools::Itertools;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use nom::bytes::complete::take_until;
use nom::character::complete::{anychar, char as nchar};
use nom::Err::Error;
use nom::error::{ErrorKind, ParseError};
use nom::{InputIter, InputTake, Offset};
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;
use crate::root::parser::parse_util::discard_ignored;

// ! BROKEN

const DEFAULT_TERMINATORS: [(char, char, bool); 3] = [
    ('{', '}', true),
    ('(', ')', true),
    ('"', '"', false)
];

pub fn default_section(s: Span, section_char: char) -> ParseResult {
    let x = section(
        s,
        DEFAULT_TERMINATORS.iter().find_position(|(c, _, _)| *c == section_char).unwrap().0,
        &DEFAULT_TERMINATORS
    );
    // println!("----------\n{:?}\n**{}\n\n**{}\n\n**{}", section_char, s.fragment(), x.as_ref().unwrap().0.fragment(), x.as_ref().unwrap().1.fragment());
    x
}

pub fn section<'a>(s: Span<'a>, terminator: usize, all_terminators: &[(char, char, bool)]) -> ParseResult<'a> {
    let (initial_span, _) = nchar(all_terminators[terminator].0)(s)?;

    let allow_subsections = all_terminators[terminator].2;

    let mut depth = 0;

    let mut s = initial_span;

    'main: loop {
        s = discard_ignored(s)?.0;

        if s.is_empty() {
            break;
        }

        if let Ok((ns, _)) = nchar::<_, ErrorTree>(all_terminators[terminator].1)(s) {
            if depth == 0 {
                return Ok((ns, initial_span.take_split(initial_span.offset(&s)).1));
            } else {
                s = ns;
                depth -= 1;
                continue;
            }
        }

        if let Ok((ns, _)) = nchar::<_, ErrorTree>(all_terminators[terminator].0)(s) {
            s = ns;
            depth += 1;
            continue;
        }

        if allow_subsections {
            for (pos, t) in all_terminators.iter().enumerate() {
                if pos == terminator { continue; }

                if let Ok(_) = nchar::<_, ErrorTree>(t.0)(s) {
                    s = section(s, pos, all_terminators)?.0;
                    continue 'main;
                }

                if let Ok(_) = nchar::<_, ErrorTree>(t.1)(s) {
                    // Unopened section closed
                    todo!()
                }
            }

            s = anychar(s)?.0;
        }
    }

    Err(Error(ErrorTree::from_char(s, all_terminators[terminator].1)))
}


