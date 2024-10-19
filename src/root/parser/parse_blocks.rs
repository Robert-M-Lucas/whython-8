use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char as nchar};
use nom::error::{ErrorKind, ParseError};
use nom::{InputTake, Offset};

use crate::root::errors::parser_errors::create_custom_error;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_util::discard_ignored;

// ! BROKEN

/// Represents terminators e.g. '(' and ')'
pub struct Terminator {
    pub opening: char,
    pub closing: char,
    /// Represents whether this terminator's contents are code or not e.g. a string
    pub code_inner: bool,
    pub escape_char: Option<char>,
}

pub const BRACE_TERMINATOR: Terminator = Terminator {
    opening: '{',
    closing: '}',
    code_inner: true,
    escape_char: None,
};

pub const BRACKET_TERMINATOR: Terminator = Terminator {
    opening: '(',
    closing: ')',
    code_inner: true,
    escape_char: None,
};

pub const STRING_TERMINATOR: Terminator = Terminator {
    opening: '"',
    closing: '"',
    code_inner: false,
    escape_char: Some('\\'),
};

pub const DEFAULT_TERMINATORS: [Terminator; 3] =
    [BRACE_TERMINATOR, BRACKET_TERMINATOR, STRING_TERMINATOR];

/// Gets the content of a given terminator using the default terminator set to intelligently handle
/// other terminators
pub fn parse_default_terminator_content<'a>(
    s: Span<'a>,
    terminator: &Terminator,
) -> ParseResult<'a> {
    parse_terminator(s, terminator, &DEFAULT_TERMINATORS)
}

/// Gets the contents of a terminator using `all_terminators` to smartly handle other terminators
/// that may occur
pub fn parse_terminator<'a>(
    s: Span<'a>,
    terminator: &Terminator,
    all_terminators: &[Terminator],
) -> ParseResult<'a> {
    let (initial_span, _) = nchar(terminator.opening)(s)?;

    let mut depth = 0;

    let mut s = initial_span;

    'main: loop {
        // Don't discard whitespace in the case of strings, for example
        if terminator.code_inner {
            s = discard_ignored(s)?.0;
        }

        // Done
        if s.is_empty() {
            break;
        }

        // Go up a level as previous block is closed
        if let Ok((ns, _)) = nchar::<_, ErrorTree>(terminator.closing)(s) {
            if depth == 0 {
                // Done as last block closed
                return Ok((ns, initial_span.take_split(initial_span.offset(&s)).1));
            } else {
                s = ns;
                depth -= 1;
                continue;
            }
        }

        // Enter new block
        if let Ok((ns, _)) = nchar::<_, ErrorTree>(terminator.opening)(s) {
            s = ns;
            depth += 1;
            continue;
        }

        // Handle escape chars
        if let Some(escape) = terminator.escape_char {
            let (ns, c) = anychar(s)?;
            if c == escape {
                let (ns, _) = anychar(ns)?;
                s = ns;
                continue;
            }
        }

        // Handle recursive terminator parsing if current block is code
        if terminator.code_inner {
            for t in all_terminators {
                if nchar::<_, ErrorTree>(t.opening)(s).is_ok() {
                    s = parse_terminator(s, t, all_terminators)?.0;
                    continue 'main;
                }

                if nchar::<_, ErrorTree>(t.closing)(s).is_ok() {
                    // Unopened section closed
                    return Err(create_custom_error(
                        format!(
                            "Found closing tag '{}' before '{}' for opening tag '{}'",
                            t.closing, terminator.closing, terminator.opening
                        ),
                        initial_span,
                    ));
                }
            }

            s = anychar(s)?.0;
        } else {
            s = anychar(s)?.0
        }
    }

    // Exited closing terminator
    Err(nom::Err::Error(ErrorTree::from_char(s, terminator.closing)))
}

/// Take until a tag or the end of the span not including occurrences in `DEFAULT_TERMINATORS`
/// blocks
pub fn take_until_or_end_discard_smart<'a>(s: Span<'a>, until: &str) -> ParseResult<'a> {
    let original = s;
    let mut s = s;
    let mut found = false;
    'outer: while !s.is_empty() {
        if let Ok((ns, _)) = tag::<_, _, ErrorTree>(until)(s) {
            found = true;
            s = ns;
            break;
        }

        let c = s.chars().next().unwrap();

        for t in &DEFAULT_TERMINATORS {
            if t.opening == c {
                s = parse_default_terminator_content(s, t)?.0;
                continue 'outer;
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

/// Take until a tag not including occurrences in `DEFAULT_TERMINATORS` blocks
#[allow(dead_code)]
pub fn take_until_discard_smart<'a>(s: Span<'a>, until: &str) -> ParseResult<'a> {
    let original = s;
    let mut s = s;
    'outer: loop {
        if s.is_empty() {
            return Err(nom::Err::Error(ErrorTree::from_error_kind(
                original,
                ErrorKind::TakeUntil,
            )));
        }

        if let Ok((ns, _)) = tag::<_, _, ErrorTree>(until)(s) {
            s = ns;
            break;
        }

        let c = s.chars().next().unwrap();

        for t in &DEFAULT_TERMINATORS {
            if t.opening == c {
                s = parse_default_terminator_content(s, t)?.0;
                continue 'outer;
            }
        }

        s = s.take_split(1).0;
    }

    let offset = original.offset(&s);
    let (end, inner) = original.take_split(offset);
    let inner = inner.take(inner.len() - until.len());

    Ok((end, inner))
}
