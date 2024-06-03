use crate::root::parser::parse_toplevel;
use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use derive_getters::Getters;
use crate::root::parser::parse_toplevel::TopLevelTokens;

pub type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<
    Span<'a>,
    &'static str,
    &'static str,
    Box<dyn std::error::Error + Send + Sync + 'static>,
>;

#[derive(Debug, Clone, Getters, Hash)]
pub struct Location {
    path: Rc<PathBuf>,
    offset: usize,
    line: u32,
}

impl Location {
    pub fn from_span(span: &Span) -> Location {
        Location {
            path: span.extra.clone(),
            offset: span.location_offset(),
            line: span.location_line(),
        }
    }
}

pub fn parse(path: PathBuf) -> Result<Vec<TopLevelTokens>, ()> {
    let text = fs::read_to_string(&path).unwrap();
    let path = Rc::new(path);
    let base = Span::new_extra(&text, &path);

    let (remaining, output) = parse_toplevel::parse_toplevel(base).map_err(|_| ())?;
    debug_assert!(remaining.is_empty());

    Ok(output)
}