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
pub struct ReturnToken {
    location: Location,
    return_value: Option<EvaluableToken>,
}

/// Tests whether a line should be parsed as a return statement
pub fn test_parse_return<'b>(s: Span) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (tag("return"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_return(x, c).map(|(s, x)| (s, LineTokens::Return(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parses a return statement
pub fn parse_return<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, ReturnToken> {
    let (s, l) = tag("return")(s)?;

    if char::<_, ErrorTree>(';')(s).is_ok() {
        return Ok((
            s,
            ReturnToken {
                location: Location::from_span(&l),
                return_value: None,
            },
        ));
    }

    let (s, _) = require_ignored(s)?;

    // Parse contents
    let (s, value) = parse_evaluable(s, containing_class, true)?;
    Ok((
        s,
        ReturnToken {
            location: Location::from_span(&l),
            return_value: Some(value),
        },
    ))
}
