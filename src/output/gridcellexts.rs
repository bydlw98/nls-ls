use compact_str::{format_compact, CompactString, ToCompactString};
use nls_term_grid::*;
use unicode_width::UnicodeWidthStr;

use crate::output::GridCell;

pub trait GridCellExts {
    fn error_cell(alignment: Alignment) -> Self;

    fn from_ascii_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self;

    fn from_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self;

    fn from_num_with_style<I: itoa::Integer>(value: I, ansi_style_str: Option<&str>) -> Self;

    fn with_capacity(capacity: usize) -> Self;

    fn append(&mut self, other: Self);

    fn push_str(&mut self, string: &str);

    fn push_str_with_width(&mut self, string: &str, width: usize);

    fn push_char(&mut self, ch: char);

    fn push_char_with_style(&mut self, ch: char, ansi_style_str: Option<&str>);
}

impl GridCellExts for GridCell {
    fn error_cell(alignment: Alignment) -> Self {
        Self {
            contents: CompactString::new_inline("?"),
            width: 1,
            alignment: alignment,
        }
    }

    fn from_ascii_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self {
        let width = value.len();
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format_compact!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
            None => value.to_compact_string(),
        };

        Self {
            contents: contents,
            width: width,
            alignment: Alignment::Left,
        }
    }

    fn from_str_with_style(value: &str, ansi_style_str: Option<&str>) -> Self {
        let width = UnicodeWidthStr::width(value);
        let contents = match ansi_style_str {
            Some(ansi_style_str) => format_compact!("\x1b[{}m{}\x1b[0m", ansi_style_str, value),
            None => value.to_compact_string(),
        };

        Self {
            contents: contents,
            width: width,
            alignment: Alignment::Left,
        }
    }

    fn from_num_with_style<I: itoa::Integer>(value: I, ansi_style_str: Option<&str>) -> Self {
        let mut buffer = itoa::Buffer::new();
        let value_string = buffer.format(value);
        let width = value_string.len();
        let contents = match ansi_style_str {
            Some(ansi_style_str) => {
                format_compact!("\x1b[{}m{}\x1b[0m", ansi_style_str, value_string)
            }
            None => value_string.to_compact_string(),
        };

        Self {
            contents: contents,
            width: width,
            alignment: Alignment::Right,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            contents: CompactString::with_capacity(capacity),
            ..Default::default()
        }
    }

    fn append(&mut self, other: Self) {
        self.contents.push_str(&other.contents);
        self.width += other.width;
    }

    fn push_str(&mut self, string: &str) {
        self.contents.push_str(string);
        self.width += UnicodeWidthStr::width(string);
    }

    fn push_str_with_width(&mut self, string: &str, width: usize) {
        self.contents.push_str(string);
        self.width += width;
    }

    fn push_char(&mut self, ch: char) {
        self.contents.push(ch);
        self.width += 1;
    }

    fn push_char_with_style(&mut self, ch: char, ansi_style_str: Option<&str>) {
        match ansi_style_str {
            Some(ansi_style_str) => {
                self.contents
                    .push_str(&format_compact!("\x1b[{}m{}\x1b[0m", ansi_style_str, ch));
                self.width += 1;
            }
            None => {
                self.contents.push(ch);
                self.width += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gridcellexts_error_cell() {
        let left_aligned_cell = GridCell::error_cell(Alignment::Left);
        let correct_left_aligned_cell = GridCell {
            contents: CompactString::from("?"),
            width: 1,
            alignment: Alignment::Left,
        };
        assert_eq!(left_aligned_cell, correct_left_aligned_cell);

        let right_aligned_cell = GridCell::error_cell(Alignment::Right);
        let correct_right_aligned_cell = GridCell {
            contents: CompactString::from("?"),
            width: 1,
            alignment: Alignment::Right,
        };
        assert_eq!(right_aligned_cell, correct_right_aligned_cell);
    }

    #[test]
    fn test_gridcellexts_from_ascii_str_with_style() {
        let cell_no_style = GridCell::from_ascii_str_with_style("1,   3", None);
        let correct_cell_no_style = GridCell {
            contents: CompactString::from("1,   3"),
            width: 6,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = GridCell::from_ascii_str_with_style("1,   3", Some("36"));
        let correct_cell_with_style = GridCell {
            contents: CompactString::from("\x1b[36m1,   3\x1b[0m"),
            width: 6,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_gridcellexts_from_str_with_style() {
        let cell_no_style = GridCell::from_str_with_style("main.rs", None);
        let correct_cell_no_style = GridCell {
            contents: CompactString::from("main.rs"),
            width: 7,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = GridCell::from_str_with_style("main.rs", Some("36"));
        let correct_cell_with_style = GridCell {
            contents: CompactString::from("\x1b[36mmain.rs\x1b[0m"),
            width: 7,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_gridcellexts_from_num_with_style() {
        let cell_no_style = GridCell::from_num_with_style(4096, None);
        let correct_cell_no_style = GridCell {
            contents: CompactString::from("4096"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_no_style, correct_cell_no_style);

        let cell_with_style = GridCell::from_num_with_style(4096, Some("36"));
        let correct_cell_with_style = GridCell {
            contents: CompactString::from("\x1b[36m4096\x1b[0m"),
            width: 4,
            alignment: Alignment::Right,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }

    #[test]
    fn test_gridcellexts_append() {
        let mut cell = GridCell::from_str_with_style("/bin -> ", None);
        let other_cell = GridCell::from_str_with_style("/usr/bin", None);
        let correct_cell = GridCell::from_str_with_style("/bin -> /usr/bin", None);
        cell.append(other_cell);

        assert_eq!(cell, correct_cell);
    }

    #[test]
    fn test_gridcellexts_push_str() {
        let mut cell = GridCell::from_str_with_style("/bin -> ", None);
        cell.push_str("/usr/bin");

        assert_eq!(
            cell,
            GridCell::from_str_with_style("/bin -> /usr/bin", None)
        );
    }

    #[test]
    fn test_gridcellexts_push_str_with_width() {
        let mut cell = GridCell::from_str_with_style("/bin -> ", None);
        cell.push_str_with_width("/usr/bin", 3);

        let correct_cell = GridCell {
            contents: CompactString::from("/bin -> /usr/bin"),
            width: 11,
            alignment: Alignment::Left,
        };

        assert_eq!(cell, correct_cell);
    }

    #[test]
    fn test_gridcellexts_push_char() {
        let mut cell = GridCell::from_str_with_style("src", None);
        cell.push_char('/');

        assert_eq!(cell, GridCell::from_str_with_style("src/", None));
    }

    #[test]
    fn test_gridcellexts_push_char_with_style() {
        let mut cell_no_style = GridCell::from_str_with_style("drwx", None);
        cell_no_style.push_char_with_style('r', None);
        assert_eq!(cell_no_style, GridCell::from_str_with_style("drwxr", None));

        let mut cell_with_style = GridCell::from_str_with_style("drwx", None);
        cell_with_style.push_char_with_style('r', Some("33;1"));
        let correct_cell_with_style = GridCell {
            contents: CompactString::from("drwx\x1b[33;1mr\x1b[0m"),
            width: 5,
            alignment: Alignment::Left,
        };
        assert_eq!(cell_with_style, correct_cell_with_style);
    }
}
