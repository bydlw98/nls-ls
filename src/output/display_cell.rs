use std::fmt;

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_displaycell_error_cell() {
        let left_aligned_cell = DisplayCell::error_cell(true);
        let correct_left_aligned_cell = DisplayCell {
            contents: String::from("?"),
            width: 1,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(left_aligned_cell, correct_left_aligned_cell);

        let right_aligned_cell = DisplayCell::error_cell(false);
        let correct_right_aligned_cell = DisplayCell {
            contents: String::from("?"),
            width: 1,
            pad_width: 0,
            left_aligned: false,
        };
        assert_eq!(right_aligned_cell, correct_right_aligned_cell);
    }

    #[test]
    fn test_displaycell_left_aligned() {
        let left_aligned_cell = DisplayCell::from(String::from("src")).left_aligned(true);
        let correct_left_aligned_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(left_aligned_cell, correct_left_aligned_cell);

        let right_aligned_cell = DisplayCell::from(String::from("src")).left_aligned(false);
        let correct_right_aligned_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 0,
            left_aligned: false,
        };
        assert_eq!(right_aligned_cell, correct_right_aligned_cell);
    }

    #[test]
    fn test_displaycell_from_ascii_str_with_style() {
        let cell_no_style = DisplayCell::from_ascii_str_with_style("1,   3", None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("1,   3"),
            width: 6,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_ascii_str_with_style("1,   3", Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m1,   3\x1b[0m"),
            width: 6,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_str_with_style() {
        let cell_no_style = DisplayCell::from_str_with_style("main.rs", None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("main.rs"),
            width: 7,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_str_with_style("main.rs", Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36mmain.rs\x1b[0m"),
            width: 7,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_num_with_style() {
        let cell_no_style = DisplayCell::from_num_with_style(4096, None);
        let correct_cell_no_style = DisplayCell {
            contents: String::from("4096"),
            width: 4,
            pad_width: 0,
            left_aligned: false,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_num_with_style(4096, Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m4096\x1b[0m"),
            width: 4,
            pad_width: 0,
            left_aligned: false,
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
            pad_width: 0,
            left_aligned: false,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = DisplayCell::from_u128_with_style(4096, Some("36"));
        let correct_cell_with_style = DisplayCell {
            contents: String::from("\x1b[36m4096\x1b[0m"),
            width: 4,
            pad_width: 0,
            left_aligned: false,
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
    fn test_displaycell_pad_to_width() {
        let mut cell = DisplayCell::from(String::from("root"));
        cell.pad_to_width(10);
        // there are 6 spaces after root
        assert_eq!(cell.to_string(), "root      ");

        let mut cell = DisplayCell::from(String::from("root"));
        // 1 is smaller than current cell width
        cell.pad_to_width(1);
        assert_eq!(cell.to_string(), "root");
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
            pad_width: 0,
            left_aligned: true,
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
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_displaycell_from_string_for_displaycell() {
        let cell = DisplayCell::from(String::from("src"));
        let correct_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 0,
            left_aligned: true,
        };

        assert_eq!(cell, correct_cell);
    }

    #[test]
    fn test_displaycell_from_displaycell_for_term_grid_cell() {
        let cell = DisplayCell::from(String::from("src"));
        let term_grid_cell = term_grid::Cell::from(cell);
        let correct_term_grid_cell = term_grid::Cell {
            contents: String::from("src"),
            width: 3,
        };

        assert_eq!(term_grid_cell, correct_term_grid_cell);
    }

    #[test]
    fn test_displaycell_fmt_display_fmt() {
        let zero_pad_width_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 0,
            left_aligned: true,
        };
        assert_eq!(zero_pad_width_cell.to_string(), "src");

        let left_aligned_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 2,
            left_aligned: true,
        };
        // there are 2 spaces after src
        assert_eq!(left_aligned_cell.to_string(), "src  ");

        let right_aligned_cell = DisplayCell {
            contents: String::from("src"),
            width: 3,
            pad_width: 2,
            left_aligned: false,
        };
        // there are 2 spaces before src
        assert_eq!(right_aligned_cell.to_string(), "  src");
    }
}
