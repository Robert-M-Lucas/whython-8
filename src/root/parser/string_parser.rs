use crate::root::ast::literals::Literal;
use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::parser::escape_codes::get_escape_code;
use crate::root::parser::file_reader::FileReader;
use crate::root::parser::parse::ParseError;
use crate::root::parser::parse::ParseError::UnclosedString;

const ESCAPE_CHAR: char = '\\';
const STRING_LITERAL_TERMINATOR: char = '"';

pub fn parse_string(reader: &mut FileReader) -> Result<BasicSymbol, ParseError> {
    let start_line = reader.line();
    let mut string = String::new();

    let mut escape = false;

    let mut eof = true;

    loop {
        let next = reader.move_read_any();
        if next.is_none() {
            break;
        }
        let next = next.unwrap();

        if escape {
            let char = get_escape_code(next);
            if char.is_none() {
                return Err(ParseError::UnknownEscapeCode(
                    reader.get_line_info_current(),
                    next,
                ));
            }
            string.push(char.unwrap());
            escape = false;
            continue;
        }

        if next == ESCAPE_CHAR {
            escape = true;
            continue;
        }

        if next == STRING_LITERAL_TERMINATOR {
            eof = false;
            break;
        }

        string.push(next)
    }

    if eof {
        return Err(UnclosedString(reader.get_line_info_current(), start_line));
    }

    Ok(BasicSymbol::Literal(Literal::String(string)))
}
