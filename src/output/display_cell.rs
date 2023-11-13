use std::fmt;

use unicode_width::UnicodeWidthStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DisplayCell {
    pub contents: String,
    pub width: usize,
    pub alignment: Alignment,
}

impl DisplayCell {
    pub fn error_cell(alignment: Alignment) -> Self {
        Self {
            contents: String::from('?'),
            width: 1,
            alignment: alignment,
        }
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
            alignment: Alignment::Left,
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
            alignment: Alignment::Left,
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
            alignment: Alignment::Right,
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
            alignment: Alignment::Right,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            contents: String::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn append(&mut self, other: Self) {
        self.contents.push_str(&other.contents);
        self.width += other.width;
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

    pub fn write<F: fmt::Write>(&self, f: &mut F, width: usize) -> fmt::Result {
        let pad_width: usize = if width <= self.width {
            0
        } else {
            width - self.width
        };

        // Check if pad width is 0
        if pad_width == 0 {
            // if pad width is 0, we do not need to do padding
            write!(f, "{}", self.contents)
        } else if self.alignment == Alignment::Left {
            write!(f, "{}{}", self.contents, " ".repeat(pad_width))
        } else {
            write!(f, "{}{}", " ".repeat(pad_width), self.contents)
        }
    }
}

impl From<String> for DisplayCell {
    fn from(value: String) -> Self {
        let width = UnicodeWidthStr::width(&*value);

        Self {
            contents: value,
            width: width,
            alignment: Alignment::Left,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_displaycell_error_cell() {
        let left_aligned_cell = DisplayCell::error_cell(Alignment::Left);
        let correct_left_aligned_cell = DisplayCell {
            contents: String::from("?"),
            width: 1,
            alignment: Alignment::Left,
        };
        assert_eq!(left_aligned_cell, correct_left_aligned_cell);

        let right_aligned_cell = DisplayCell::error_cell(Alignment::Right);
        let correct_right_aligned_cell = DisplayCell {
            contents: String::from("?"),
            width: 1,
            alignment: Alignment::Right,
        };
        assert_eq!(right_aligned_cell, correct_right_aligned_cell);
    }

    #[test]
    fn test_displaycell_from_ascii_str_with_style() {
        let cell_no_style = DisplayCell::from_ascii_str_with_style("1,   3", None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("1,   3"),
            width: 6,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_ascii_str_with_style("1,   3", Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m1,   3\x1b[0m"),
            width: 6,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_str_with_style() {
        let cell_no_style = DisplayCell::from_str_with_style("main.rs", None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("main.rs"),
            width: 7,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_str_with_style("main.rs", Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36mmain.rs\x1b[0m"),
            width: 7,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_num_with_style() {
        let cell_no_style = DisplayCell::from_num_with_style(4096, None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("4096"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_num_with_style(4096, Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m4096\x1b[0m"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[cfg(windows)]
    #[test]
    fn test_displaycell_from_u128_with_style() {
        let cell_no_style = DisplayCell::from_u128_with_style(4096, None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("4096"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_u128_with_style(4096, Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m4096\x1b[0m"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_append() {
        let mut cell = DisplayCell::from(String::from("/bin -> "));
        let other_cell = DisplayCell::from(String::from("/usr/bin"));
        let correct_cell = DisplayCell::from(String::from("/bin -> /usr/bin"));
        cell.append(other_cell);

        assert_eq!(cell, correct_cell);
    }

    #[test]
    fn test_displaycell_push_str() {
        let mut cell = DisplayCell::from(String::from("/bin -> "));
        cell.push_str("/usr/bin");

        assert_eq!(cell, DisplayCell::from(String::from("/bin -> /usr/bin")));
    }

    #[test]
    fn test_displaycell_push_str_with_width() {
        let mut cell = DisplayCell::from(String::from("/bin -> "));
        cell.push_str_with_width("/usr/bin", 3);

        let correct_cell = DisplayCell {
            contents: String::from("/bin -> /usr/bin"),
            width: 11,
            alignment: Alignment::Left,
        };

        assert_eq!(cell, correct_cell);
    }

    #[test]
    fn test_displaycell_push_char() {
        let mut cell = DisplayCell::from(String::from("src"));
        cell.push_char('/');

        assert_eq!(cell, DisplayCell::from(String::from("src/")));
    }

    #[test]
    fn test_displaycell_push_char_with_style() {
        let mut cell_no_style = DisplayCell::from(String::from("drwx"));
        cell_no_style.push_char_with_style('r', None);
        assert_eq!(cell_no_style, DisplayCell::from(String::from("drwxr")));

        let mut cell_with_style = DisplayCell::from(String::from("drwx"));
        cell_with_style.push_char_with_style('r', Some("33;1"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("drwx\x1b[33;1mr\x1b[0m"),
            width: 5,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_string_for_displaycell() {
        let cell = DisplayCell::from(String::from("src"));
        let correct_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            alignment: Alignment::Left,
        };

        assert_eq!(cell, correct_cell);
    }
}
