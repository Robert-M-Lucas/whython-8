use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;

use crate::root::errors::parser_errors::ParseError;
use crate::root::errors::WErr;
use crate::root::parser::handle_errors::handle_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse_toplevel;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::parser::use_parser::parse_uses;

pub type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<Span<'a>, &'static str, &'static str, String>;

pub fn parse(path: PathBuf) -> Result<Vec<TopLevelTokens>, WErr> {
    let mut path_queue = vec![(path, Location::builtin())];
    let mut output = Vec::new();

    while let Some((path, location)) = path_queue.pop() {
        print!("\n  - {}", path.display());
        let Ok(text) = fs::read_to_string(path.as_path()) else {
            return WErr::ne(
                ParseError::FailedToOpenFile(format!("{}", path.display())),
                location,
            );
        };

        let path = Rc::new(path);
        let base = Span::new_extra(&text, &path);

        let (after_use, found_paths) = handle_error(parse_uses(base))?;
        path_queue.extend(found_paths);

        let res = parse_toplevel::parse_toplevel(after_use);
        let (remaining, new_output) = handle_error(res)?;
        debug_assert!(remaining.is_empty());
        output.extend(new_output);
    }
    println!();

    Ok(output)
}
