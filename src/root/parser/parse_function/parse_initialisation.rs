use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{
    parse_evaluable, parse_full_name, EvaluableToken, UnresolvedTypeRefToken,
};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use derive_getters::Getters;
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

// Token holding an initialiser
#[derive(Debug, Getters)]
pub struct InitialisationToken {
    #[allow(dead_code)]
    location: Location,
    name: SimpleNameToken,
    type_name: Option<UnresolvedTypeRefToken>,
    value: EvaluableToken,
}

/// Test if line should be parsed as initialiser
pub fn test_parse_initialisation<'b>(s: Span<'_>) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (tag("let"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_initialisation(x, c).map(|(s, x)| (s, LineTokens::Initialisation(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parse initialiser
pub fn parse_initialisation<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, InitialisationToken> {
    let (s, l) = tag("let")(s)?;
    let (s, _) = require_ignored(s)?;

    // Parse variable name
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    
    let (s, type_name) = if let Ok((s, _)) = char::<nom_locate::LocatedSpan<&str, _>, ErrorTree>(':')(s) {
        let (s, _) = discard_ignored(s)?;

        // Parse type
        let (s, type_name) = parse_full_name(s, containing_class)?;
        let (s, _) = discard_ignored(s)?;
        (s, Some(type_name))
    }
    else {
        (s, None)
    };

    let (s, _) = char('=')(s)?;
    let (s, _) = discard_ignored(s)?;
    
    // Parse value
    let (s, value) = parse_evaluable(s, containing_class, true)?;

    Ok((
        s,
        InitialisationToken {
            location: Location::from_span(&l),
            name,
            type_name,
            value,
        },
    ))
}
