use crate::root::parser::parse::Span;
use color_print::cformat;
use nom::InputTake;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

pub enum ToLocation<'a> {
    Location(Location),
    Span(Span<'a>),
}

impl<'a> ToLocation<'a> {
    pub fn from_location(location: Location) -> ToLocation<'a> {
        ToLocation::Location(location)
    }

    pub fn from_span(span: Span<'a>) -> ToLocation<'a> {
        ToLocation::Span(span)
    }

    pub fn to_location(self) -> Location {
        match self {
            ToLocation::Location(location) => location,
            ToLocation::Span(s) => Location::from_span(&s),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct InnerLocation {
    /// Path of file
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
enum ErrorLocation {
    Location(InnerLocation),
    Builtin,
    None,
}

#[derive(Debug, Clone, Hash)]
pub struct LocationTyped<ErrorType = ErrorL> {
    error_type: PhantomData<ErrorType>,
    inner_location: ErrorLocation,
}

impl LocationTyped<ErrorL> {
    pub fn into_warning(self) -> LocationTyped<WarningL> {
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
            inner_location: ErrorLocation::Location(InnerLocation {
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
            inner_location: ErrorLocation::Location(InnerLocation {
                path: span.extra.clone(),
                offset: span.location_offset(),
                line: span.location_line(),
            }),
        }
    }

    pub fn path(&self) -> Option<&Rc<PathBuf>> {
        match &self.inner_location {
            ErrorLocation::Location(l) => Some(&l.path),
            ErrorLocation::Builtin => None,
            ErrorLocation::None => None,
        }
    }

    pub fn builtin() -> LocationTyped<ErrorType> {
        LocationTyped {
            error_type: Default::default(),
            inner_location: ErrorLocation::Builtin,
        }
    }

    pub fn none() -> LocationTyped<ErrorType> {
        LocationTyped {
            error_type: Default::default(),
            inner_location: ErrorLocation::None,
        }
    }

    pub fn has_location(&self) -> bool {
        !matches!(self.inner_location, ErrorLocation::None)
    }

    fn fmt_choice(&self, f: &mut Formatter<'_>, is_warning: bool) -> std::fmt::Result {
        let location = match &self.inner_location {
            ErrorLocation::Builtin => {
                writeln!(f, "{}", cformat!("<c,bold>Builtin Definition</>"))?;
                return Ok(());
            }
            ErrorLocation::None => {
                writeln!(f, "{}", cformat!("<c,bold>No Location</>"))?;
                return Ok(());
            }
            ErrorLocation::Location(l) => l,
        };

        writeln!(f, "{}", cformat!("<c,bold>In File:</>"))?;
        writeln!(f, "    {}", location.path.as_path().to_string_lossy())?;
        writeln!(f, "{}", cformat!("<c,bold>At:</>"))?;

        fn fail(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Failed to fetch file reference (has the file changed?)")
        }

        let Ok(file) = fs::read_to_string(location.path.as_path()) else {
            return fail(f);
        };

        if location.line == (file.lines().count() + 1) as u32 {
            return writeln!(f, "{}", cformat!("    <c, bold>End Of File</>"));
        }

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
