use std::ffi::OsString;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::process;

use crate::ls_colors::LsColors;
use crate::theme::ThemeConfig;

#[derive(Debug)]
pub struct Config {
    pub is_atty: bool,
    pub color: bool,
    pub dereference: bool,
    pub dereference_cmdline_symlink: bool,
    pub dereference_cmdline_symlink_dir: bool,
    pub git_ignore: bool,
    pub ignore_file: bool,
    pub ignore_glob_vec: Vec<String>,
    pub ignore_hidden: bool,
    pub indicator_style: IndicatorStyle,
    pub ls_colors: LsColors,
    pub mode_format: ModeFormat,
    pub numeric_uid_gid: bool,
    pub output_format: OutputFormat,
    pub recursive: bool,
    pub max_depth: Option<usize>,
    pub reverse: bool,
    pub list_current_and_parent_dirs: bool,
    pub list_dir: bool,
    pub list_inode: bool,
    pub list_allocated_size: bool,
    pub allocated_size_blocks: AllocatedSizeBlocks,
    pub list_owner: bool,
    pub list_group: bool,
    pub size_format: SizeFormat,
    pub sorting_order: SortingOrder,
    pub timestamp_used: TimestampUsed,
    pub theme: ThemeConfig,
}

impl Config {
    pub fn init() -> (Self, Vec<PathBuf>) {
        let mut config = Self::default();
        let mut path_args_vec = Vec::with_capacity(4);
        if io::stdout().is_terminal() {
            config.is_atty = true;
            config.color = true;
            config.output_format = OutputFormat::Vertical;
        }
        config.parse_args(std::env::args_os().skip(1), &mut path_args_vec);

        if config.color {
            config.ls_colors.init();
            config.theme = ThemeConfig::with_default_colors();
        }

        if !config.dereference_cmdline_symlink_dir {
            config.dereference_cmdline_symlink_dir = !(!config.list_dir
                || config.indicator_style.is_classify()
                || config.output_format.is_long())
        }

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
        // raw.next(&mut cursor); // Skip the bin

        while let Some(arg) = raw.next(&mut cursor) {
            // if arg is "--"
            if arg.is_escape() {
                path_args_vec.extend(raw.remaining(&mut cursor).map(PathBuf::from));
            } else if let Some(mut shorts) = arg.to_short() {
                while let Some(short) = shorts.next_flag() {
                    match short {
                        Ok('a') => {
                            self.list_current_and_parent_dirs = true;
                            self.ignore_hidden = false;
                        }
                        Ok('A') => {
                            self.list_current_and_parent_dirs = false;
                            self.ignore_hidden = false;
                        }
                        Ok('c') => {
                            self.timestamp_used = TimestampUsed::Changed;
                        }
                        Ok('C') => {
                            self.output_format = OutputFormat::Vertical;
                        }
                        Ok('d') => {
                            self.list_dir = false;
                        }
                        Ok('F') => {
                            self.indicator_style = IndicatorStyle::Classify;
                        }
                        Ok('g') => {
                            self.list_owner = false;
                            self.output_format = OutputFormat::Long;
                        }
                        Ok('h') => {
                            self.size_format = SizeFormat::HumanReadable;
                            self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                        }
                        Ok('H') => {
                            self.dereference = false;
                            self.dereference_cmdline_symlink = true;
                            self.dereference_cmdline_symlink_dir = true;
                        }
                        Ok('i') => {
                            self.list_inode = true;
                        }
                        Ok('I') => match shorts.next_value_os() {
                            Some(pattern) => {
                                self.ignore_glob_vec
                                    .push(format!("!{}", pattern.to_string_lossy()));
                            }
                            None => {
                                eprintln!("nls: '-I' requires an argument");
                                process::exit(1);
                            }
                        },
                        Ok('k') => {
                            if self.size_format.is_raw() {
                                self.allocated_size_blocks = AllocatedSizeBlocks::Kibibytes;
                            }
                        }
                        Ok('l') => {
                            self.output_format = OutputFormat::Long;
                        }
                        Ok('L') => {
                            self.dereference = true;
                            self.dereference_cmdline_symlink = true;
                            self.dereference_cmdline_symlink_dir = true;
                        }
                        Ok('n') => {
                            self.numeric_uid_gid = true;
                            self.output_format = OutputFormat::Long;
                        }
                        Ok('o') => {
                            self.list_group = false;
                            self.output_format = OutputFormat::Long;
                        }
                        Ok('p') => {
                            self.indicator_style = IndicatorStyle::Slash;
                        }
                        Ok('r') => {
                            self.reverse = true;
                        }
                        Ok('R') => {
                            self.recursive = true;
                        }
                        Ok('s') => {
                            self.list_allocated_size = true;
                        }
                        Ok('S') => {
                            self.sorting_order = SortingOrder::Size;
                        }
                        Ok('t') => {
                            self.sorting_order = SortingOrder::Timestamp;
                        }
                        Ok('u') => {
                            self.timestamp_used = TimestampUsed::Accessed;
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
                        self.list_current_and_parent_dirs = true;
                        self.ignore_hidden = false;
                    }
                    Ok("almost-all") => {
                        self.list_current_and_parent_dirs = false;
                        self.ignore_hidden = false;
                    }
                    Ok("allocated-bytes") => {
                        if self.size_format.is_raw() {
                            self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                        }
                    }
                    Ok("color") => match value {
                        Some(when) => {
                            if when == "always" {
                                self.color = true;
                            } else if when == "auto" {
                                self.color = self.is_atty;
                            } else if when == "never" {
                                self.color = false;
                            } else {
                                eprintln!(
                                    "nls: '{}' is an invalid argument for '--color'",
                                    when.to_string_lossy()
                                );
                                eprintln!(
                                    "     possible arguments are ['always', 'auto', 'never']"
                                );
                                process::exit(1);
                            }
                        }
                        None => self.color = true,
                    },
                    Ok("classify") => {
                        self.indicator_style = IndicatorStyle::Classify;
                    }
                    Ok("dereference") => {
                        self.dereference = true;
                        self.dereference_cmdline_symlink = true;
                        self.dereference_cmdline_symlink_dir = true;
                    }
                    Ok("dereference-command-line") => {
                        self.dereference = false;
                        self.dereference_cmdline_symlink = true;
                        self.dereference_cmdline_symlink_dir = true;
                    }
                    Ok("directory") => {
                        self.list_dir = false;
                    }
                    Ok("gitignore") => {
                        self.git_ignore = true;
                    }
                    Ok("help") => {
                        println!(
                            "{}",
                            include_str!(concat!(env!("OUT_DIR"), "/help-page.txt"))
                        );
                        process::exit(0);
                    }
                    Ok("human-readable") => {
                        self.size_format = SizeFormat::HumanReadable;
                        self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                    }
                    Ok("iec") => {
                        self.size_format = SizeFormat::Iec;
                        self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                    }
                    Ok("ignore-glob") => match value {
                        Some(pattern) => {
                            self.ignore_glob_vec
                                .push(format!("!{}", pattern.to_string_lossy()));
                        }
                        None => {
                            eprintln!("nls: '--ignore-glob' requires an argument");
                            process::exit(1);
                        }
                    },

                    Ok("ignore-file") => {
                        self.ignore_file = true;
                    }
                    Ok("inode") => {
                        self.list_inode = true;
                    }
                    Ok("kibibytes") => {
                        if self.size_format.is_raw() {
                            self.allocated_size_blocks = AllocatedSizeBlocks::Kibibytes;
                        }
                    }
                    Ok("max-depth") => match value {
                        Some(max_depth_os) => {
                            match max_depth_os.to_string_lossy().parse::<usize>() {
                                Ok(max_depth) => {
                                    self.max_depth = Some(max_depth);
                                }
                                Err(err) => {
                                    eprintln!(
                                        "nls: {:?} is not a valid input for '--max-depth': {}",
                                        max_depth_os, err
                                    );
                                    process::exit(1);
                                }
                            }
                        }
                        None => {
                            eprintln!("nls: '--max-depth' requires an argument");
                            process::exit(1);
                        }
                    },
                    Ok("mode") => match value {
                        Some(word) => {
                            if word == "native" {
                                self.mode_format.set_native();
                            } else if word == "pwsh" {
                                self.mode_format = ModeFormat::Pwsh;
                            } else if word == "rwx" {
                                self.mode_format = ModeFormat::Rwx;
                            } else {
                                eprintln!(
                                    "nls: '{}' is an invalid argument for '--mode'",
                                    word.to_string_lossy()
                                );
                                eprintln!("     possible arguments are ['native', 'pwsh', 'rwx']");
                                process::exit(1);
                            }
                        }
                        None => {
                            eprintln!("nls: '--mode' requires an argument");
                            eprintln!("     possible arguments are ['native', 'pwsh', 'rwx']");
                            process::exit(1);
                        }
                    },
                    Ok("numeric-uid-gid") => {
                        self.numeric_uid_gid = true;
                        self.output_format = OutputFormat::Long;
                    }
                    Ok("recursive") => {
                        self.recursive = true;
                    }
                    Ok("reverse") => {
                        self.reverse = true;
                    }
                    Ok("si") => {
                        self.size_format = SizeFormat::Si;
                        self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                    }
                    Ok("size") => {
                        self.list_allocated_size = true;
                    }
                    Ok("time") => match value {
                        Some(timestamp_used) => {
                            if timestamp_used == "accessed" || timestamp_used == "atime" {
                                self.timestamp_used = TimestampUsed::Accessed;
                            } else if timestamp_used == "changed" || timestamp_used == "ctime" {
                                self.timestamp_used = TimestampUsed::Changed;
                            } else if timestamp_used == "created" || timestamp_used == "btime" {
                                self.timestamp_used = TimestampUsed::Created;
                            } else if timestamp_used == "modified" || timestamp_used == "mtime" {
                                self.timestamp_used = TimestampUsed::Modified;
                            } else {
                                eprintln!(
                                    "nls: {:?} is not a valid input for '--time'",
                                    timestamp_used
                                );
                                eprintln!(
                                    "     possible arguments are ['accessed', 'changed', 'created', 'modified', 'atime', 'ctime', 'btime', 'mtime']"
                                );
                                process::exit(1);
                            }
                        }
                        None => {
                            eprintln!("nls: '--time' requires an argument");
                            eprintln!(
                                    "     possible arguments are ['accessed', 'changed', 'created', 'modified', 'atime', 'ctime', 'btime', 'mtime']"
                            );
                            process::exit(1);
                        }
                    },
                    Ok("version") => {
                        println!("nls-ls {}", env!("CARGO_PKG_VERSION"));
                        process::exit(0);
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
            color: false,
            dereference: false,
            dereference_cmdline_symlink: false,
            dereference_cmdline_symlink_dir: false,
            git_ignore: false,
            ignore_file: false,
            ignore_glob_vec: Vec::default(),
            ignore_hidden: true,
            indicator_style: IndicatorStyle::default(),
            ls_colors: LsColors::default(),
            mode_format: ModeFormat::default(),
            numeric_uid_gid: false,
            output_format: OutputFormat::default(),
            recursive: false,
            max_depth: None,
            reverse: false,
            list_dir: true,
            list_inode: false,
            list_allocated_size: false,
            allocated_size_blocks: AllocatedSizeBlocks::default(),
            list_owner: true,
            list_group: true,
            list_current_and_parent_dirs: false,
            size_format: SizeFormat::default(),
            sorting_order: SortingOrder::default(),
            timestamp_used: TimestampUsed::default(),
            theme: ThemeConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AllocatedSizeBlocks {
    Posix,
    Kibibytes,
    Raw,
}

impl Default for AllocatedSizeBlocks {
    fn default() -> Self {
        Self::Posix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn is_classify(&self) -> bool {
        *self == Self::Classify
    }
}

impl Default for IndicatorStyle {
    fn default() -> Self {
        Self::Never
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModeFormat {
    Pwsh,
    Rwx,
}

impl ModeFormat {
    pub fn set_native(&mut self) {
        if cfg!(windows) {
            *self = Self::Pwsh
        } else {
            *self = Self::Rwx;
        }
    }

    #[cfg(windows)]
    pub fn is_pwsh(&self) -> bool {
        *self == Self::Pwsh
    }

    pub fn is_rwx(&self) -> bool {
        *self == Self::Rwx
    }
}

impl Default for ModeFormat {
    fn default() -> Self {
        if cfg!(windows) {
            Self::Pwsh
        } else {
            Self::Rwx
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SizeFormat {
    Raw,
    HumanReadable,
    Iec,
    Si,
}

impl SizeFormat {
    fn is_raw(&self) -> bool {
        *self == Self::Raw
    }
}

impl Default for SizeFormat {
    fn default() -> Self {
        Self::Raw
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SortingOrder {
    FileName,
    Size,
    Timestamp,
}

impl SortingOrder {
    #[cfg(windows)]
    pub fn is_size(&self) -> bool {
        *self == Self::Size
    }
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

#[derive(Debug)]
pub enum TimestampUsed {
    Accessed,
    Changed,
    Created,
    Modified,
}

impl Default for TimestampUsed {
    fn default() -> Self {
        Self::Modified
    }
}
