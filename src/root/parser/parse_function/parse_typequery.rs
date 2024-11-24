use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::require_ignored;
use derive_getters::Getters;
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

/// Token representing a return statement with location
#[derive(Debug, Getters)]
pub struct TypequeryToken {
    location: Location,
    querying: EvaluableToken,
}

/// Tests whether a line should be parsed as a return statement
pub fn test_parse_typequery<'b>(s: Span) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (tag("typequery"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_typequery(x, c).map(|(s, x)| (s, LineTokens::Typequery(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parses a return statement
pub fn parse_typequery<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, TypequeryToken> {
    let (s, l) = tag("typequery")(s)?;

    let (s, _) = require_ignored(s)?;

    let sl = s;
    
    // Parse contents
    let (s, value) = parse_evaluable(s, containing_class, true)?;
    Ok((
        s,
        TypequeryToken {
            location: Location::from_span(&sl),
            querying: value,
        },
    ))
}
