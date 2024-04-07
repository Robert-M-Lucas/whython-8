use std::path::PathBuf;
use std::{fs, io};

use same_file::is_same_file;
use thiserror::Error;

use ParseError::Nested;

use crate::root::ast::keywords::MOD_KEYWORD;
use crate::root::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::root::parser::file_reader::FileReader;
use crate::root::parser::line_info::LineInfo;
use crate::root::parser::normal_parser::parse_normal;
use crate::root::parser::parse::ParseError::ModNotFollowedByPath;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("File read error on path '{0}'")]
    FileRead(PathBuf, io::Error),
    #[error("Error: {1}\n{0}")]
    Nested(LineInfo, Box<ParseError>),
    #[error("Error: Operator '{1}' not recognised\n{0}")]
    OperatorNotRecognised(LineInfo, String),
    #[error("Error: 'mod' must be followed by a path\n{0}")]
    ModNotFollowedByPath(LineInfo),
    #[error("Error: Keyword ('{1}') cannot be followed by . or #\n{0}")]
    KeywordFollowed(LineInfo, String),
    #[error("Error: Closing '{1}' not found (started on line {2})\n{0}")]
    NotClosed(LineInfo, char, usize),
    #[error("Error: Unknown escape code '{1}'\n{0}")]
    UnknownEscapeCode(LineInfo, char),
    #[error("Error: String literal started on line {1} not closed\n{0}")]
    UnclosedString(LineInfo, usize),
    #[error("Error: Closing '{1}' found with no corresponding opening bracket\n{0}")]
    NoOpening(LineInfo, char),
    #[error("Error: Names cannot contain character '{1}' (UTF-8 Code: {2:?})\n{0}")]
    BadName(LineInfo, char, Vec<u8>),
    #[error("Initialiser must specify a type after '@'")]
    NoInitialiserType(LineInfo),
    #[error("Error: Initialiser type must be followed by braces containing attribute values\n{0}")]
    NoInitialiserContents(LineInfo),
    #[error("Error: Attribute cannot be empty (must be a value between commas)\n{0}")]
    NoInitialiserAttribute(LineInfo),
}

pub fn parse(
    path: PathBuf,
    asts: &mut Vec<BasicAbstractSyntaxTree>,
    files_followed: &mut Vec<PathBuf>,
) -> Result<(), ParseError> {
    for other_path in &*files_followed {
        if is_same_file(&path, other_path).map_err(|x| ParseError::FileRead(path.clone(), x))? {
            return Ok(());
        }
    }

    let data = fs::read_to_string(&path);

    if let Err(e) = data {
        return Err(ParseError::FileRead(path, e));
    }
    let mut reader = FileReader::new(path.clone(), data.unwrap());

    files_followed.push(path);

    // * IMPORT PHASE
    {
        while reader.read_until_char(' ').0 == MOD_KEYWORD {
            reader.move_to_next_char(' ');

            reader.checkpoint();
            let (file, _eof) = reader.move_read_to_next_char(';');
            let trimmed = file.trim();
            if trimmed.is_empty() {
                return Err(ModNotFollowedByPath(reader.get_line_info()));
            }

            if let Err(e) = parse(PathBuf::from(file), asts, files_followed) {
                return Err(Nested(reader.get_line_info(), Box::new(e)));
            }
        }
    }

    reader.checkpoint();
    let ast = parse_normal(&mut reader, BlockType::Base)?;

    let inner = match ast {
        BasicSymbol::AbstractSyntaxTree(inner) => inner,
        _ => panic!(),
    };

    asts.push(inner);

    Ok(())
}

#[derive(PartialEq)]
pub enum BlockType {
    Base,
    Braces,         // start line
    Brackets,       // start line
    SquareBrackets, // start line
}

// fn recursively_parse_symbols(reader: &mut FileReader, block_type: BlockType) -> Result<Symbol, ParseError> {
//     match block_type {
//         BlockType::String(start_line) => {
//             parse_string(reader, start_line)
//         }
//         _ => {
//             parse_normal(reader, block_type)
//         }
//     }
// }
