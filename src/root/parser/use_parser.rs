use std::path::PathBuf;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::anychar;

use crate::root::errors::parser_errors::create_custom_error;
use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_util::discard_ignored;

pub fn parse_uses(s: Span) -> ParseResult<Span, Vec<(PathBuf, Location)>> {
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

        let mut path_rem = path;
        while let Ok((rem, c)) = anychar::<_, ErrorTree>(path_rem) {
            if c.is_alphanumeric() || c == '_' || c == '/' {
                path_rem = rem;
                continue;
            }
            let mut utf8 = [0u8; 4];
            c.encode_utf8(&mut utf8);
            let mut utf8_str = "[".to_string();
            utf8_str += &utf8.map(|b| format!("{b:02X}")).join(", ");
            utf8_str.push(']');


            return Err(create_custom_error(
                format!("Invalid character in path '{}' - UTF-8 bytes: {}. Allowed characters are alphanumerics, '_' and '/'", c, utf8_str),
                path_rem,
            ));
        }

        let path_buf = PathBuf::from(format!("{}.why", path));
        found_paths.push((path_buf, Location::from_span(&path)));

        s = ns;
    }
}
