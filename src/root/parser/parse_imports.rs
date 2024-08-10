use nom::bytes::complete::{tag, take_till};
use nom::character::complete::anychar;

use crate::root::errors::parser_errors::create_custom_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_util::discard_ignored;
use crate::root::parser::path_storage::{FileID, PathStorage};

pub fn parse_imports<'a>(
    s: Span<'a>,
    path_storage: &mut PathStorage,
    current_file: FileID,
) -> ParseResult<'a, Span<'a>, Vec<(FileID, Location)>> {
    let mut s = s;
    let mut found_paths = Vec::new();
    loop {
        let (ns, _) = discard_ignored(s)?;
        let mut is_use = true;

        let Ok((ns, _)) = tag::<_, _, ErrorTree>("use")(ns).or_else(|_| {
            is_use = false;
            tag::<_, _, ErrorTree>("import")(ns)
        }) else {
            return Ok((ns, found_paths));
        };

        let (ns, _) = discard_ignored(ns)?;
        let Ok((pre_s, path)) =
            take_till::<_, _, ErrorTree>(|c| c == ';' || c == '\n' || c == '\r')(ns)
        else {
            return Err(create_custom_error(
                "Did not find ending ';' when parsing path".to_string(),
                ns,
            ));
        };

        let (ns, next) = anychar::<_, ErrorTree>(pre_s).unwrap();
        if next != ';' {
            return Err(create_custom_error(
                "Use path cannot be broken by newline".to_string(),
                pre_s,
            ));
        }

        let (_, ids) = path_storage.get_id_and_add_to_file(current_file, is_use, path)?;
        
        for id in ids {
            found_paths.push((id, Location::from_span(&path)));
        }

        s = ns;
    }
}
