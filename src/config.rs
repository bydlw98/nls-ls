use std::ffi::OsString;
use std::path::PathBuf;
use std::process;

use crate::utils;

#[derive(Debug)]
pub struct Config {
    pub is_atty: bool,
    pub git_ignore: bool,
    pub ignore_file: bool,
    pub ignore_hidden: bool,
    pub indicator_style: IndicatorStyle,
    pub output_format: OutputFormat,
    pub reverse: bool,
    pub show_current_and_parent_dirs: bool,
    pub sorting_order: SortingOrder,
    pub width: usize,
}

impl Config {
    pub fn init() -> (Self, Vec<PathBuf>) {
        let mut config = Self::default();
        let mut path_args_vec = Vec::with_capacity(4);

        if let Some(term_width) = utils::terminal_width() {
            config.is_atty = true;
            config.output_format = OutputFormat::Vertical;
            config.width = term_width;
        }
        config.parse_args(std::env::args_os(), &mut path_args_vec);

        path_args_vec.sort();

        (config, path_args_vec)
    }

    fn parse_args(
        &mut self,
        raw: impl IntoIterator<Item = impl Into<OsString>>,
        path_args_vec: &mut Vec<PathBuf>,
    ) {
        let raw = clap_lex::RawArgs::new(raw);
        let mut cursor = raw.cursor();
        raw.next(&mut cursor); // Skip the bin

        while let Some(arg) = raw.next(&mut cursor) {
            // if arg is "--"
            if arg.is_escape() {
                path_args_vec.extend(raw.remaining(&mut cursor).map(PathBuf::from));
            } else if let Some(mut shorts) = arg.to_short() {
                while let Some(short) = shorts.next_flag() {
                    match short {
                        Ok('a') => {
                            self.show_current_and_parent_dirs = true;
                            self.ignore_hidden = false;
                        }
                        Ok('A') => {
                            self.show_current_and_parent_dirs = false;
                            self.ignore_hidden = false;
                        }
                        Ok('C') => {
                            self.output_format = OutputFormat::Vertical;
                        }
                        Ok('F') => {
                            self.indicator_style = IndicatorStyle::Classify;
                        }
                        Ok('l') => {
                            self.output_format = OutputFormat::Long;
                        }
                        Ok('p') => {
                            self.indicator_style = IndicatorStyle::Slash;
                        }
                        Ok('r') => {
                            self.reverse = true;
                        }
                        Ok('S') => {
                            self.sorting_order = SortingOrder::Size;
                        }
                        Ok('t') => {
                            self.sorting_order = SortingOrder::Timestamp;
                        }
                        Ok('x') => {
                            self.output_format = OutputFormat::Across;
                        }
                        Ok('1') => {
                            self.output_format = OutputFormat::SingleColumn;
                        }
                        Ok(ch) => {
                            eprintln!("nls: Unexpected flag: '-{}'", ch);
                            process::exit(1);
                        }
                        Err(err) => {
                            eprintln!("nls: Unexpected flag: '-{}'", err.to_string_lossy());
                            process::exit(1);
                        }
                    }
                }
            } else if let Some((long, value)) = arg.to_long() {
                match long {
                    Ok("all") => {
                        self.show_current_and_parent_dirs = true;
                        self.ignore_hidden = false;
                    }
                    Ok("almost-all") => {
                        self.show_current_and_parent_dirs = false;
                        self.ignore_hidden = false;
                    }
                    Ok("classify") => {
                        self.indicator_style = IndicatorStyle::Classify;
                    }
                    Ok("ignore-file") => {
                        self.ignore_file = true;
                    }
                    Ok("gitignore") => {
                        self.git_ignore = true;
                    }
                    Ok("reverse") => {
                        self.reverse = true;
                    }
                    _ => {
                        eprintln!("nls: Unexpected flag '{}'", arg.display());
                        process::exit(1);
                    }
                }
            } else {
                path_args_vec.push(PathBuf::from(arg.to_value_os().to_owned()));
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_atty: false,
            git_ignore: false,
            ignore_file: false,
            ignore_hidden: true,
            indicator_style: IndicatorStyle::default(),
            output_format: OutputFormat::default(),
            reverse: false,
            show_current_and_parent_dirs: false,
            sorting_order: SortingOrder::default(),
            width: 80,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndicatorStyle {
    Classify,
    Slash,
    Never,
}

impl IndicatorStyle {
    pub const DIR: char = '/';
    pub const SYMLINK: char = '@';
    pub const EXEC: char = '*';
    #[cfg(unix)]
    pub const SOCKET: char = '=';
    #[cfg(unix)]
    pub const FIFO: char = '|';

    pub fn dir(&self) -> bool {
        *self != Self::Never
    }

    pub fn others(&self) -> bool {
        *self == Self::Classify
    }
}

impl Default for IndicatorStyle {
    fn default() -> Self {
        Self::Never
    }
}

#[derive(Debug)]
pub enum SortingOrder {
    FileName,
    Size,
    Timestamp
}

impl Default for SortingOrder {
    fn default() -> Self {
        Self::FileName
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputFormat {
    SingleColumn,
    Vertical,
    Across,
    Long,
}

impl OutputFormat {
    pub fn is_long(&self) -> bool {
        *self == Self::Long
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::SingleColumn
    }
}
