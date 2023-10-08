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
    pub fn error_cell(left_aligned: bool) -> Self {
        Self {
            contents: String::from('?'),
            width: 1,
            pad_width: 0,
            left_aligned: left_aligned,
        }
    }

    pub fn left_aligned(mut self, left_aligned: bool) -> Self {
        self.left_aligned = left_aligned;

        self
    }

    pub fn from_ascii_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self {
        let width = value.len();
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
            None => value.to_string(),
        };

        Self {
            contents: contents,
            width: width,
            pad_width: 0,
            left_aligned: true,
        }
    }

    pub fn from_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self {
        let width = UnicodeWidthStr::width(value);
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
            None => value.to_string(),
        };

        Self {
            contents: contents,
            width: width,
            pad_width: 0,
            left_aligned: true,
        }
    }

    pub fn from_num_with_style(value: u64, ansi_style_str: Option<&str>) -> Self {
        let value_string = value.to_string();
        let width = value_string.len();
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value_string),
            None => value_string,
        };

        Self {
            contents: contents,
            width: width,
            pad_width: 0,
            left_aligned: false,
        }
    }

    #[cfg(windows)]
    pub fn from_u128_with_style(value: u128, ansi_style_str: Option<&str>) -> DisplayCell {
        let value_string = value.to_string();
        let width = value_string.len();
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value_string),
            None => value_string,
        };

        Self {
            contents: contents,
            width: width,
            pad_width: 0,
            left_aligned: false,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            contents: String::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn append(&mut self, other: Self) {
        self.contents.push_str(&other.contents);
        self.width += other.width;
    }

    pub fn pad_to_width(&mut self, width: usize) {
        if width <= self.width {
            self.pad_width = 0;
        } else {
            self.pad_width = width - self.width;
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.contents.push_str(string);
        self.width += UnicodeWidthStr::width(string);
    }

    pub fn push_str_with_width(&mut self, string: &str, width: usize) {
        self.contents.push_str(string);
        self.width += width;
    }

    pub fn push_char(&mut self, ch: char) {
        self.contents.push(ch);
        self.width += 1;
    }

    pub fn push_char_with_style(&mut self, ch: char, ansi_style_str: Option<&str>) {
        match ansi_style_str {
            Some(ansi_style_str) => {
                self.contents
                    .push_str(&format!("\x1b[{}m{}\x1b[0m", ansi_style_str, ch));
                self.width += 1;
            }
            None => {
                self.contents.push(ch);
                self.width += 1;
            }
        }
    }

    pub fn paint(&mut self, ansi_style_str: &str) {
        self.contents
            .insert_str(0, &format!("\x1b[{}m", ansi_style_str));
        self.contents.push_str("\x1b[0m");
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
        let width = UnicodeWidthStr::width(&*value);

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
        } else if self.left_aligned {
            write!(f, "{}{}", self.contents, " ".repeat(self.pad_width))
        } else {
            write!(f, "{}{}", " ".repeat(self.pad_width), self.contents)
        }
    }
}
