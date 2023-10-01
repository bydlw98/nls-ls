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
    pub fn error_left_aligned() -> Self {
        Self {
            contents: String::from('?'),
            width: 1,
            pad_width: 0,
            left_aligned: true,
        }
    }

    pub fn error_right_aligned() -> Self {
        Self {
            contents: String::from('?'),
            width: 1,
            pad_width: 0,
            left_aligned: false,
        }
    }
    pub fn from_ascii_string(value: String, left_aligned: bool) -> DisplayCell {
        let width = value.len();

        Self {
            contents: value,
            width: width,
            left_aligned: left_aligned,
            ..Default::default()
        }
    }

    pub fn from_ascii_str_with_style(
        value: &str,
        ansi_style_str: Option<&str>,
        left_aligned: bool,
    ) -> Self {
        let width = value.len();
        match ansi_style_str {
            Some(ansi_style_str) => Self {
                contents: format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
            None => Self {
                contents: value.to_string(),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
        }
    }

    pub fn from_str_with_style(
        value: &str,
        ansi_style_str: Option<&str>,
        left_aligned: bool,
    ) -> Self {
        let width = UnicodeWidthStr::width(value);
        match ansi_style_str {
            Some(ansi_style_str) => Self {
                contents: format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
            None => Self {
                contents: value.to_string(),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
        }
    }

    pub fn from_num_with_style(
        value: u64,
        ansi_style_str: Option<&str>,
        left_aligned: bool,
    ) -> DisplayCell {
        let value_string = value.to_string();
        let width = UnicodeWidthStr::width(&*value_string);
        match ansi_style_str {
            Some(ansi_style_str) => Self {
                contents: format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value_string),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
            None => Self {
                contents: value_string,
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
        }
    }

    #[cfg(windows)]
    pub fn from_u128_with_style(
        value: u128,
        ansi_style_str: Option<&str>,
        left_aligned: bool,
    ) -> DisplayCell {
        let value_string = value.to_string();
        let width = UnicodeWidthStr::width(&*value_string);
        match ansi_style_str {
            Some(ansi_style_str) => Self {
                contents: format!("\x1b[{}m{}\x1b[0m", ansi_style_str, value_string),
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
            None => Self {
                contents: value_string,
                width: width,
                pad_width: 0,
                left_aligned: left_aligned,
            },
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

    pub fn push_ascii_str(&mut self, string: &str) {
        self.width += string.len();
        self.contents.push_str(string);
    }

    pub fn push_str(&mut self, string: &str) {
        self.contents.push_str(string);
        self.width += string.width();
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
                self.contents.push_str("\x1b[");
                self.contents.push_str(ansi_style_str);
                self.contents.push('m');
                self.contents.push(ch);
                self.contents.push_str("\x1b[0m");
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
