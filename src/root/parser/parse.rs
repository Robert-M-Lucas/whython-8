use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;

use crate::root::errors::WErr;
use crate::root::parser::handle_errors::handle_error;
use crate::root::parser::parse_toplevel;
use crate::root::parser::parse_toplevel::TopLevelTokens;

pub type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<Span<'a>, &'static str, &'static str, String>;

pub fn parse(path: PathBuf) -> Result<Vec<TopLevelTokens>, WErr> {
    let text = fs::read_to_string(&path).unwrap();
    let path = Rc::new(path);
    let base = Span::new_extra(&text, &path);

    let res = parse_toplevel::parse_toplevel(base);
    let (remaining, output) = handle_error(res)?;

    debug_assert!(remaining.is_empty());

    Ok(output)
}
