use crate::root::parser::line_info::LineInfo;

use std::path::PathBuf;
use std::rc::Rc;

pub struct FileReader {
    path: Rc<PathBuf>,
    data: String,
    cursor: usize,
    line_start: usize,
    line: usize,
    checkpoint: (usize, usize),
}

#[allow(dead_code)]
impl FileReader {
    pub fn new(path: PathBuf, data: String) -> FileReader {
        FileReader {
            path: Rc::new(path),
            data,
            cursor: 0,
            line_start: 0,
            line: 1,
            checkpoint: (1, 0),
        }
    }

    pub fn get_line_info(&self) -> LineInfo {
        LineInfo::new(self.path.clone(), self.checkpoint.0, self.checkpoint.1)
    }

    pub fn get_line_info_changed(&self, line: usize, char_start: usize) -> LineInfo {
        LineInfo::new(self.path.clone(), line, char_start)
    }

    pub fn get_line_info_current(&self) -> LineInfo {
        if self.cursor - self.line_start == 0 {
            LineInfo::new(self.path.clone(), self.line, self.cursor - self.line_start)
        } else {
            LineInfo::new(
                self.path.clone(),
                self.line,
                self.cursor - self.line_start - 1,
            )
        }
    }

    pub fn get_line_char(&self) -> (usize, usize) {
        if self.cursor - self.line_start < 2 {
            (self.line, self.cursor - self.line_start)
        } else {
            (self.line, self.cursor - self.line_start - 2)
        }
    }

    pub fn checkpoint(&mut self) -> (usize, usize) {
        self.checkpoint = self.get_line_char();
        self.checkpoint
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn get_path(&self) -> Rc<PathBuf> {
        self.path.clone()
    }

    pub fn move_read_any(&mut self) -> Option<char> {
        let c = self.data.chars().nth(self.cursor);
        if c.is_some() {
            self.cursor += 1;
            if c.unwrap() == '\n' {
                self.line += 1;
                self.line_start = self.cursor;
            }
        }
        c
    }

    pub fn read_until_char(&self, c: char) -> (String, bool) {
        let mut out = String::new();

        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            if char == c {
                eof = false;
                break;
            }
            out.push(char);
        }

        (out, eof)
    }

    pub fn skip_until_newline(&mut self) -> (String, bool) {
        let mut out = String::new();

        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        let mut i = 0;
        for char in chars {
            i += 1;
            if char == '\n' || char == '\r' {
                eof = false;
                break;
            }
            out.push(char);
        }

        self.cursor += i;

        (out, eof)
    }

    pub fn move_to_next_char(&mut self, c: char) {
        let chars = self.data.chars().skip(self.cursor);

        for char in chars {
            self.cursor += 1;
            if char == '\n' {
                self.line += 1;
                self.line_start = self.cursor;
            }
            if char == c {
                break;
            }
        }
    }

    pub fn move_read_to_next_char(&mut self, c: char) -> (String, bool) {
        let mut out = String::new();

        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            self.cursor += 1;

            if char == '\n' {
                self.line += 1;
                self.line_start = self.cursor;
            }

            if char == c {
                eof = false;
                break;
            }
            out.push(char);
        }

        (out, eof)
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            if !char.is_whitespace() {
                eof = false;
                break;
            }

            self.cursor += 1;
        }

        eof
    }

    pub fn all_read(&self) -> bool {
        self.cursor >= self.data.len()
    }
}
