use std::path::PathBuf;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::anychar;

use crate::root::errors::parser_errors::create_custom_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_util::discard_ignored;
use crate::root::parser::path_storage::{FileID, PathStorage};

pub fn parse_uses<'a>(s: Span<'a>, path_storage: &mut PathStorage) -> ParseResult<'a, Span<'a>, Vec<(FileID, Location)>> {
    let mut s = s;
    let mut found_paths = Vec::new();
    loop {
        let (ns, _) = discard_ignored(s)?;
        let Ok((ns, _)) = tag::<_, _, ErrorTree>("use")(ns) else {
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

        let (_, (id, new)) = path_storage.get_file_path_id_checked(path)?;
        
        if new {
            found_paths.push((id, Location::from_span(&path)));
        }
        
        s = ns;
    }
}
