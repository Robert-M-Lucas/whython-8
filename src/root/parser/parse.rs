use std::fs;
use std::path::Path;

use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;

use crate::root::errors::parser_errors::ParseError;
use crate::root::errors::WErr;
use crate::root::parser::handle_errors::handle_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse_toplevel;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::parser::path_storage::{FileID, PathStorage};
use crate::root::parser::use_parser::parse_uses;

pub type Span<'a> = LocatedSpan<&'a str, FileID>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<Span<'a>, &'static str, &'static str, String>;

pub fn parse(path_storage: &mut PathStorage) -> Result<Vec<TopLevelTokens>, WErr> {
    let mut path_queue = vec![(FileID::main_file(), Location::builtin())];
    let mut output = Vec::new();

    while let Some((file_id, location)) = path_queue.pop() {
        let reconstructed = path_storage.reconstruct_file(file_id);
        print!("\n  - {}", &reconstructed);
        let Ok(text) = fs::read_to_string(Path::new(&reconstructed)) else {
            return WErr::ne(
                ParseError::FailedToOpenFile(format!("{}", reconstructed)),
                location,
            );
        };

        let base = Span::new_extra(&text, file_id);

        let (after_use, new_files) = handle_error(parse_uses(base, path_storage), &path_storage)?;
        path_queue.extend(new_files);

        let res = parse_toplevel::parse_toplevel(after_use);
        let (remaining, new_output) = handle_error(res, &path_storage)?;
        debug_assert!(remaining.is_empty());
        output.extend(new_output);
    }
    println!();

    Ok(output)
}
