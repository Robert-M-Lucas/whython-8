use nom_supreme::error::{BaseErrorKind, StackContext};

use crate::root::errors::parser_errors::ParseError;
use crate::root::errors::WErr;
use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult};
use crate::root::parser::path_storage::PathStorage;

pub fn handle_error<A, B>(
    res: ParseResult<A, B>,
    path_storage: &PathStorage,
) -> Result<(A, B), WErr> {
    match res {
        Ok(v) => Ok(v),
        Err(e) => match &e {
            nom::Err::Incomplete(_n) => WErr::ne(
                ParseError::ParserIncompleteErrorsNotImplemented, // TODO
                Location::builtin(),
            ),
            nom::Err::Error(y) | nom::Err::Failure(y) => Err(handle_error_tree(y, path_storage)),
        },
    }
}

fn handle_error_tree(e: &ErrorTree, path_storage: &PathStorage) -> WErr {
    match e {
        ErrorTree::Base { location, kind } => match kind {
            BaseErrorKind::Expected(smth) => WErr::n(
                ParseError::Expected(smth.to_string()),
                Location::from_span(location),
            ),
            BaseErrorKind::Kind(k) => WErr::n(
                ParseError::NomErrorKind(k.description().to_string()),
                Location::from_span(location),
            ),
            BaseErrorKind::External(e) => WErr::n(e, Location::from_span(location)),
        },
        ErrorTree::Stack { base, contexts } => {
            let mut sb = "Base Error:\n".to_string();
            for l in handle_error_tree(base, path_storage)
                .with_context(path_storage)
                .to_string()
                .lines()
            {
                sb += "     ";
                sb += l;
                sb += "\n";
            }

            for (s, c) in contexts {
                sb += "\nIn:\n";

                let e = match c {
                    StackContext::Kind(k) => k.description().to_string(),
                    StackContext::Context(c) => c.to_string(),
                };

                for l in WErr::n(e, Location::from_span(s))
                    .with_context(path_storage)
                    .to_string()
                    .lines()
                {
                    sb += "    ";
                    sb += l;
                    sb += "\n";
                }
            }

            WErr::locationless(sb)
        }
        ErrorTree::Alt(z) => {
            let mut sb = "Failed multiple parsers -\n".to_string();

            for (i, e) in z.iter().enumerate() {
                sb += &format!("{}:\n", i + 1);

                let werr = handle_error_tree(e, path_storage)
                    .with_context(path_storage)
                    .to_string();
                for line in werr.lines() {
                    sb += "    ";
                    sb += line;
                    sb += "\n";
                }
            }

            WErr::locationless(sb)
        }
    }
}
