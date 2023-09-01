use std::fmt;

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct DisplayCell {
    contents: String,
    width: usize,
    pad_width: usize,
    left_aligned: bool,
}

impl DisplayCell {
    pub fn from_ascii_string(value: String, left_aligned: bool) -> DisplayCell {
        let width = value.len();

        Self {
            contents: value,
            width: width,
            left_aligned: left_aligned,
            ..Default::default()
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            contents: String::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn pad_to_width(&mut self, width: usize) {
        if width <= self.width {
            self.pad_width = 0;
        } else {
            self.pad_width = width - self.width;
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.contents.push(ch);
        self.width += 1;
    }
}

impl Default for DisplayCell {
    fn default() -> Self {
        Self {
            contents: String::default(),
            width: 0,
            pad_width: 0,
            left_aligned: true,
        }
    }
}

impl From<String> for DisplayCell {
    fn from(value: String) -> Self {
        let width = value.width();

        Self {
            contents: value,
            width: width,
            pad_width: 0,
            left_aligned: true,
        }
    }
}

impl From<DisplayCell> for term_grid::Cell {
    fn from(value: DisplayCell) -> Self {
        Self {
            contents: value.contents,
            width: value.width,
        }
    }
}

impl fmt::Display for DisplayCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Check if pad width is 0
        if self.pad_width == 0 {
            // if pad width is 0, we do not need to do padding
            write!(f, "{}", self.contents)
        } else {
            if self.left_aligned {
                write!(f, "{}{}", self.contents, " ".repeat(self.pad_width))
            } else {
                write!(f, "{}{}", " ".repeat(self.pad_width), self.contents)
            }
        }
    }
}
