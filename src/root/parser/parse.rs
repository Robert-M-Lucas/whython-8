use std::cmp::min;
use std::fmt::{Display, Formatter};
use crate::root::parser::parse_toplevel;
use nom::IResult;
use nom_locate::LocatedSpan;
use nom_supreme::error::GenericErrorTree;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use derive_getters::Getters;
use crate::root::parser::parse_toplevel::TopLevelTokens;

pub type Span<'a> = LocatedSpan<&'a str, &'a Rc<PathBuf>>;

pub type ParseResult<'a, I = Span<'a>, O = Span<'a>, E = ErrorTree<'a>> = IResult<I, O, E>;
pub type ErrorTree<'a> = GenericErrorTree<
    Span<'a>,
    &'static str,
    &'static str,
    Box<dyn std::error::Error + Send + Sync + 'static>,
>;

#[derive(Debug, Clone, Getters, Hash)]
pub struct Location {
    path: Rc<PathBuf>,
    /// Offset in the line, counted from 0
    offset: usize,
    /// Line number, counted from 1
    line: u32,
}

impl Location {
    pub fn from_span(span: &Span) -> Location {
        Location {
            path: span.extra.clone(),
            offset: span.location_offset(),
            line: span.location_line(),
        }
    }
}

const CHAR_LIMIT: usize = 61;

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn fail(f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Failed to fetch file reference (has the file changed?")
        }

        let Ok(file) = fs::read_to_string(self.path.as_path()) else { return fail(f); };

        let mut line_iter = file.lines();

        if self.line > 1 {
            if self.line > 2 {
                writeln!(f, "{} |  ...", self.line - 2)?;
            }

            let Some(line) = line_iter.nth(self.line as usize - 2) else { return fail(f); };
            let line = if line.chars().count() > CHAR_LIMIT { format!("{} ...", line.chars().take(CHAR_LIMIT - 4).collect::<String>()) } else { line.to_string() };
            writeln!(f, "{} |  {}", self.line - 1, line)?;
        }

        let Some(line) = line_iter.next() else { return fail(f); };
        let (mut start, mut end) = (0usize, line.chars().count() - 1);

        if end > CHAR_LIMIT {
            let start_dist = self.offset - start;
            let end_dist = end - self.offset;

            if start_dist > end_dist {
                let take_from_start = min(start_dist, CHAR_LIMIT / 2);
                start += take_from_start;
                end -= CHAR_LIMIT - 1 - take_from_start;
            }
            else {
                let take_from_end = min(end_dist, CHAR_LIMIT / 2);
                end -= take_from_end;
                start = CHAR_LIMIT - 1 - take_from_end;
            }
        }

        end += 1;

        writeln!(f, "{} |  {}", self.line, line.chars().skip(start).take(end - start).collect::<String>())?;
        writeln!(f, "E |  {}^Here", (0..(self.offset - start)).map(|_| ' ').collect::<String>())?;

        if let Some(line) = line_iter.next() {
            let line = if line.chars().count() > CHAR_LIMIT { format!("{} ...", line.chars().take(CHAR_LIMIT - 4).collect::<String>()) } else { line.to_string() };
            writeln!(f, "{} |  {}", self.line + 1, line)?;
            if line_iter.next().is_some() {
                writeln!(f, "{} |  ...", self.line + 2)?;
            }
        }

        Ok(())
    }
}

pub fn parse(path: PathBuf) -> Result<Vec<TopLevelTokens>, ()> {
    let text = fs::read_to_string(&path).unwrap();
    let path = Rc::new(path);
    let base = Span::new_extra(&text, &path);

    let (remaining, output) = parse_toplevel::parse_toplevel(base).map_err(|_| ())?;
    debug_assert!(remaining.is_empty());

    Ok(output)
}
