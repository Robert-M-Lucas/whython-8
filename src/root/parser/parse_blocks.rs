use std::path::PathBuf;
use std::rc::Rc;
use itertools::Itertools;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use nom::character::complete::{anychar, char as nchar};
use nom::error::{ErrorKind, ParseError};
use nom::{InputTake, Offset};
use nom::bytes::complete::{tag, take_until};
use nom_locate::LocatedSpan;
use crate::root::parser::parse_util::discard_ignored;

// ! BROKEN

pub struct Terminator {
    pub opening: char,
    pub closing: char,
    pub allow_recursive: bool,
    pub escape_char: Option<char>
}

pub const BRACE_TERMINATOR: Terminator = Terminator {
    opening: '{',
    closing: '}',
    allow_recursive: true,
    escape_char: None,
};

pub const BRACKET_TERMINATOR: Terminator = Terminator {
    opening: '(',
    closing: ')',
    allow_recursive: true,
    escape_char: None,
};

pub const STRING_TERMINATOR: Terminator = Terminator {
    opening: '"',
    closing: '"',
    allow_recursive: false,
    escape_char: Some('\\'),
};

pub const DEFAULT_TERMINATORS: [Terminator; 3] = [
    BRACE_TERMINATOR,
    BRACKET_TERMINATOR,
    STRING_TERMINATOR
];

pub fn parse_terminator_default_set<'a, 'b>(s: Span<'a>, terminator: &'b Terminator) -> ParseResult<'a> {
    parse_terminator(
        s,
        terminator,
        &DEFAULT_TERMINATORS
    )
}

pub fn parse_terminator<'a, 'b, 'c>(s: Span<'a>, terminator: &'b Terminator, all_terminators: &'c [Terminator]) -> ParseResult<'a> {
    let (initial_span, _) = nchar(terminator.opening)(s)?;

    let allow_subsections = terminator.allow_recursive;

    let mut depth = 0;

    let mut s = initial_span;

    'main: loop {
        s = discard_ignored(s)?.0;

        if s.is_empty() {
            break;
        }

        if let Ok((ns, _)) = nchar::<_, ErrorTree>(terminator.closing)(s) {
            if depth == 0 {
                return Ok((ns, initial_span.take_split(initial_span.offset(&s)).1));
            } else {
                s = ns;
                depth -= 1;
                continue;
            }
        }

        if let Ok((ns, _)) = nchar::<_, ErrorTree>(terminator.opening)(s) {
            s = ns;
            depth += 1;
            continue;
        }

        if allow_subsections {
            for t in all_terminators {
                if let Ok(_) = nchar::<_, ErrorTree>(t.opening)(s) {
                    s = parse_terminator(s, t, all_terminators)?.0;
                    continue 'main;
                }

                if let Ok(_) = nchar::<_, ErrorTree>(t.closing)(s) {
                    // Unopened section closed
                    todo!()
                }
            }

            s = anychar(s)?.0;
        }
    }

    Err(nom::Err::Error(ErrorTree::from_char(s, terminator.closing)))
}

pub fn take_until_or_end_discard_smart<'a>(s: Span<'a>, until: &str) -> ParseResult<'a> {
    let original = s.clone();
    let mut s = s;
    let mut found = false;
    while !s.is_empty() {
        if let Ok((ns, _)) = tag::<&str, LocatedSpan<&str, &Rc<PathBuf>>, ErrorTree>(until)(s) {
            found = true;
            s = ns;
            break;
        }

        let c = s.chars().next().unwrap();

        for t in &DEFAULT_TERMINATORS {
            if t.opening == c {
                s = parse_terminator_default_set(s, t)?.0;
                continue;
            }
        }

        s = s.take_split(1).0;
    }

    let offset = original.offset(&s);
    let (end, mut inner) = original.take_split(offset);
    if found {
        inner = inner.take(inner.len() - until.len());
    }

    Ok((end, inner))
}

pub fn take_until_discard_smart<'a>(s: Span<'a>, until: &str) -> ParseResult<'a> {
    let original = s.clone();
    let mut s = s;
    loop {
        if s.is_empty() {
            return Err(nom::Err::Error(ErrorTree::from_error_kind(original, ErrorKind::TakeUntil)));
        }

        if let Ok((ns, _)) = tag::<&str, LocatedSpan<&str, &Rc<PathBuf>>, ErrorTree>(until)(s) {
            s = ns;
            break;
        }

        let c = s.chars().next().unwrap();

        for t in &DEFAULT_TERMINATORS {
            if t.opening == c {
                s = parse_terminator_default_set(s, t)?.0;
                continue;
            }
        }

        s = s.take_split(1).0;
    }

    let offset = original.offset(&s);
    let (end, inner) = original.take_split(offset);
    let inner = inner.take(inner.len() - until.len());

    Ok((end, inner))
}
