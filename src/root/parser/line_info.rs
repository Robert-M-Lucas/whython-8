use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LineInfo {
    file: Option<Rc<PathBuf>>,
    line: usize,
    char_start: usize,
}

#[allow(dead_code)]
impl LineInfo {
    pub fn new(file: Rc<PathBuf>, line: usize, char_start: usize) -> LineInfo {
        LineInfo {
            file: Some(file),
            line,
            char_start,
        }
    }

    pub fn builtin() -> LineInfo {
        LineInfo {
            file: None,
            line: 0,
            char_start: 0,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn file(&self) -> Option<Rc<PathBuf>> {
        self.file.clone()
    }
}

impl Display for LineInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.file.is_none() {
            writeln!(f, "In builtin function")?;
            return Ok(());
        }

        writeln!(f, "In file: {:?}", self.file.as_ref().unwrap().as_os_str())?;
        let line_text = self.line.to_string();
        write!(f, "{}| ", line_text)?;
        let t = fs::read_to_string(self.file.as_ref().unwrap().as_ref()).unwrap();
        let line = t.split('\n').nth(self.line - 1).unwrap();

        let mut changed_start = false;
        let mut start = 0;
        if self.char_start > 20 {
            start = self.char_start - 10;
            changed_start = true;
        }

        let mut changed_end = false;
        let mut end = line.chars().count() - 1;
        if line.chars().count() - self.char_start > 10 {
            end = self.char_start + 10;
            changed_end = true;
        }

        if changed_start {
            write!(f, "... ")?;
        }

        write!(
            f,
            "{}",
            &line[line.char_indices().nth(start).unwrap().0
                ..line.char_indices().nth(end).unwrap().0]
        )?;
        if changed_end {
            write!(f, " ...")?;
        }
        writeln!(f)?;

        let mut offset = line_text.len() + 2;
        if changed_start {
            offset += 4 + 10;
        } else {
            offset += self.char_start;
        }
        for _ in 0..offset {
            write!(f, " ")?;
        }
        write!(f, "^ here")?;

        Ok(())
    }
}
