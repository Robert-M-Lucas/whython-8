use crate::root::errors::parser_errors::ParseError;
use crate::root::errors::WErr;
use crate::root::parser::parse_toplevel;
use crate::root::parser::parse_toplevel::TopLevelTokens;
use color_print::cformat;
use lazy_static::lazy_static;
use nom::{IResult, InputTake};
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;
use std::cmp::min;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Write};
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

pub type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<
    Span<'a>,
    &'static str,
    &'static str,
    Box<dyn std::error::Error + Send + Sync + 'static>,
>;

lazy_static! {
    static ref BUILTIN_PATH: &'static OsStr = OsStr::new("builtin");
}

#[derive(Debug, Clone, Hash)]
struct InnerLocation {
    path: Rc<PathBuf>,
    /// Offset in the line, counted from 0
    offset: usize,
    /// Line number, counted from 1
    line: u32,
}

#[derive(Debug, Clone)]
pub struct ErrorL;
#[derive(Debug, Clone)]
pub struct WarningL;

pub type Location = LocationTyped<ErrorL>;

#[derive(Debug, Clone, Hash)]
pub struct LocationTyped<ErrorType = ErrorL> {
    error_type: PhantomData<ErrorType>,
    inner_location: Option<InnerLocation>,
}

impl LocationTyped<ErrorL> {
    pub fn to_warning(self) -> LocationTyped<WarningL> {
        LocationTyped {
            error_type: Default::default(),
            inner_location: self.inner_location,
        }
    }
}

impl<ErrorType> LocationTyped<ErrorType> {
    pub fn from_span(span: &Span) -> LocationTyped<ErrorType> {
        LocationTyped {
            error_type: Default::default(),
            inner_location: Some(InnerLocation {
                path: span.extra.clone(),
                offset: span.location_offset(),
                line: span.location_line(),
            }),
        }
    }

    pub fn from_span_end(span: &Span) -> LocationTyped<ErrorType> {
        let (span, _) = &span.take_split(span.len());

        LocationTyped {
            error_type: Default::default(),
            inner_location: Some(InnerLocation {
                path: span.extra.clone(),
                offset: span.location_offset(),
                line: span.location_line(),
            }),
        }
    }

    pub fn path(&self) -> Option<&Rc<PathBuf>> {
        self.inner_location.as_ref().map(|l| &l.path)
    }

    pub fn builtin() -> LocationTyped<ErrorType> {
        LocationTyped {
            error_type: Default::default(),
            inner_location: None,
        }
    }

    pub fn is_builtin(&self) -> bool {
        self.inner_location.is_none()
    }

    fn fmt_choice(&self, f: &mut Formatter<'_>, is_warning: bool) -> std::fmt::Result {
        // TODO: Inefficient!
        // (Maybe fine because it is a 'bad' path?)

        if self.inner_location.is_none() {
            writeln!(f, "{}", cformat!("<c,bold>Builtin Definition</>"))?;
            return Ok(());
        }
        let location = self.inner_location.as_ref().unwrap();

        writeln!(f, "{}", cformat!("<c,bold>In File:</>"))?;
        writeln!(f, "    {}", location.path.as_path().to_string_lossy())?;
        writeln!(f, "{}", cformat!("<c,bold>At:</>"))?;

        fn fail(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Failed to fetch file reference (has the file changed?)")
        }

        let Ok(file) = fs::read_to_string(location.path.as_path()) else {
            return fail(f);
        };

        let mut offset = 0usize;
        let mut chars = file.chars();
        for _ in 0..location.offset {
            let Some(c) = chars.next() else {
                return fail(f);
            };
            if c == '\n' {
                offset = 0;
            } else {
                offset += 1;
            }
        }

        let mut line_iter = file.lines();

        let largest_num_len = format!("{}", location.line + 2).len();

        if location.line > 1 {
            if location.line > 2 {
                writeln!(
                    f,
                    "{:0width$} |  ...",
                    location.line - 2,
                    width = largest_num_len
                )?;
            }

            let Some(line) = line_iter.nth(location.line as usize - 2) else {
                return fail(f);
            };
            let line = if line.chars().count() > CHAR_LIMIT {
                format!(
                    "{} ...",
                    line.chars().take(CHAR_LIMIT - 4).collect::<String>()
                )
            } else {
                line.to_string()
            };
            writeln!(
                f,
                "{:0width$} |  {}",
                location.line - 1,
                line,
                width = largest_num_len
            )?;
        }

        let Some(line) = line_iter.next() else {
            return fail(f);
        };
        let (mut start, mut end) = (0usize, line.chars().count() - 1);

        if end > CHAR_LIMIT {
            let start_dist = offset - start;
            let end_dist = end - offset;

            if start_dist > end_dist {
                let take_from_start = min(start_dist, CHAR_LIMIT / 2);
                start += take_from_start;
                end -= CHAR_LIMIT - 1 - take_from_start;
            } else {
                let take_from_end = min(end_dist, CHAR_LIMIT / 2);
                end -= take_from_end;
                start = CHAR_LIMIT - 1 - take_from_end;
            }
        }

        end += 1;

        writeln!(
            f,
            "{:0width$} |  {}",
            location.line,
            line.chars()
                .skip(start)
                .take(end - start)
                .collect::<String>(),
            width = largest_num_len
        )?;

        if is_warning {
            let warn_line = format!(
                "{:0width$} |  {}^Here",
                "W",
                (0..(offset - start)).map(|_| ' ').collect::<String>(),
                width = largest_num_len
            );
            writeln!(f, "{}", cformat!("<y,bold>{}</>", warn_line))?;
        } else {
            let err_line = format!(
                "{:0width$} |  {}^Here",
                "E",
                (0..(offset - start)).map(|_| ' ').collect::<String>(),
                width = largest_num_len
            );
            writeln!(f, "{}", cformat!("<r,bold>{}</>", err_line))?;
        }

        if let Some(line) = line_iter.next() {
            let line = if line.chars().count() > CHAR_LIMIT {
                format!(
                    "{} ...",
                    line.chars().take(CHAR_LIMIT - 4).collect::<String>()
                )
            } else {
                line.to_string()
            };
            writeln!(
                f,
                "{:0width$} |  {}",
                location.line + 1,
                line,
                width = largest_num_len
            )?;
            if line_iter.next().is_some() {
                writeln!(
                    f,
                    "{:0width$} |  ...",
                    location.line + 2,
                    width = largest_num_len
                )?;
            }
        }

        Ok(())
    }
}

const CHAR_LIMIT: usize = 61;

impl Display for LocationTyped<ErrorL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_choice(f, false)
    }
}

impl Display for LocationTyped<WarningL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_choice(f, true)
    }
}

pub fn parse(path: PathBuf) -> Result<Vec<TopLevelTokens>, WErr> {
    let text = fs::read_to_string(&path).unwrap();
    let path = Rc::new(path);
    let base = Span::new_extra(&text, &path);

    let (remaining, output) = match parse_toplevel::parse_toplevel(base) {
        Ok(v) => v,
        Err(e) => {
            // TODO:
            println!("{:?}", e);
            return WErr::ne(ParseError::ParserErrorsNotImplemented, Location::builtin());
        }
    };
    debug_assert!(remaining.is_empty());

    Ok(output)
}
