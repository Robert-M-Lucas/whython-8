use std::collections::HashMap;
use std::fs;
use std::path::Path;

use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;

use crate::root::errors::parser_errors::ParseError;
use crate::root::errors::WErr;
use crate::root::parser::handle_errors::handle_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse_imports::parse_imports;
use crate::root::parser::parse_toplevel;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use crate::root::parser::path_storage::{FileID, PathStorage};

pub type Span<'a> = LocatedSpan<&'a str, FileID>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<Span<'a>, &'static str, &'static str, String>;

/// Parses files into tokens
pub fn parse(path_storage: &mut PathStorage) -> Result<HashMap<FileID, Vec<TopLevelTokens>>, WErr> {
    // Paths to process
    let mut path_queue = vec![(FileID::MAIN_FILE, Location::builtin())];
    let mut output = HashMap::new();

    // Iterate as long as there are files to process
    while let Some((file_id, location)) = path_queue.pop() {
        // Get path
        let reconstructed = path_storage.reconstruct_file(file_id);
        print!("\n  - {}", &reconstructed);
        let Ok(text) = fs::read_to_string(Path::new(&reconstructed)) else {
            return WErr::ne(
                ParseError::FailedToOpenFile(reconstructed.to_string()),
                location,
            );
        };

        let base = Span::new_extra(&text, file_id);
        
        // Parse imports
        let (after_use, new_files) =
            handle_error(parse_imports(base, path_storage, file_id), path_storage)?;
        path_queue.extend(new_files);
        
        // Parse contents
        let res = parse_toplevel::parse_toplevel(after_use);
        let (remaining, new_output) = handle_error(res, path_storage)?;
        debug_assert!(remaining.is_empty());
        output.insert(file_id, new_output);
    }
    println!();

    Ok(output)
}
