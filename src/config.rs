use std::ffi::OsString;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::process;

use compact_str::{format_compact, CompactString};

use crate::ls_colors::LsColors;
use crate::theme::{IconTheme, ThemeConfig};

const HELP: &str = include_str!(concat!(env!("OUT_DIR"), "/help-page.txt"));
const VERSION: &str = concat!("nls-ls ", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
pub struct Config {
    pub is_atty: bool,
    pub color: bool,
    pub dereference: bool,
    pub dereference_cmdline_symlink: bool,
    pub dereference_cmdline_symlink_dir: bool,
    pub git_ignore: bool,
    pub ignore_file: bool,
    pub ignore_glob_vec: Vec<CompactString>,
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
    pub icons: IconTheme,
    pub theme: ThemeConfig,
}

impl Config {
    pub fn init() -> (Self, Vec<PathBuf>) {
        let mut config = Self::default();
        let mut path_args_vec = Vec::with_capacity(4);
        if io::stdout().is_terminal() {
            config.is_atty = true;
            config.color = true;
            config.icons = IconTheme::with_default_icons();
            config.output_format = OutputFormat::Vertical;
        }
        if let Err(err) = config.parse_args(std::env::args_os().skip(1), &mut path_args_vec) {
            eprintln!("nls: {}", err);
            process::exit(1);
        }

        if config.color {
            config.ls_colors = LsColors::with_colors();
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
    ) -> anyhow::Result<()> {
        use anyhow::anyhow;
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_args(raw);
        while let Some(arg) = parser.next()? {
            match arg {
                Value(value) => {
                    path_args_vec.push(value.into());
                }
                Short('a') | Long("all") => {
                    self.list_current_and_parent_dirs = true;
                    self.ignore_hidden = false;
                }
                Short('A') | Long("almost-all") => {
                    self.list_current_and_parent_dirs = false;
                    self.ignore_hidden = false;
                }
                Long("allocated-bytes") => {
                    if self.size_format.is_raw() {
                        self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                    }
                }
                Short('c') => {
                    self.timestamp_used = TimestampUsed::Changed;
                }
                Short('C') => {
                    self.output_format = OutputFormat::Vertical;
                }
                Long("color") => match parser.optional_value() {
                    Some(when) => {
                        if when == "always" {
                            self.color = true;
                        } else if when == "auto" {
                            self.color = self.is_atty;
                        } else if when == "never" {
                            self.color = false;
                        } else {
                            return Err(anyhow!(
                                "'{}' is an invalid argument for '--color'\n\
                                 possible arguments are ['always', 'auto', 'never']",
                                when.to_string_lossy()
                            ));
                        }
                    }
                    None => self.color = true,
                },
                Short('d') | Long("directory") => {
                    self.list_dir = false;
                }
                Short('F') | Long("classify") => {
                    self.indicator_style = IndicatorStyle::Classify;
                }
                Short('g') => {
                    self.list_owner = false;
                    self.output_format = OutputFormat::Long;
                }
                Long("gitignore") => {
                    self.git_ignore = true;
                }
                Short('h') | Long("human-readable") => {
                    self.size_format = SizeFormat::HumanReadable;
                    self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                }
                Short('H') | Long("dereference-command-line") => {
                    self.dereference = false;
                    self.dereference_cmdline_symlink = true;
                    self.dereference_cmdline_symlink_dir = true;
                }
                Long("help") => {
                    println!("{}", HELP);
                    process::exit(0);
                }
                Short('i') | Long("inode") => {
                    self.list_inode = true;
                }
                Short('I') | Long("ignore-glob") => {
                    let value_os = parser.value()?;
                    self.ignore_glob_vec
                        .push(format_compact!("!{}", value_os.to_string_lossy()));
                }
                Long("icons") => match parser.optional_value() {
                    Some(when) => {
                        if when == "always" {
                            self.icons = IconTheme::with_default_icons();
                        } else if when == "auto" {
                            if self.is_atty {
                                self.icons = IconTheme::with_default_icons();
                            } else {
                                self.icons = IconTheme::default();
                            }
                        } else if when == "never" {
                            self.icons = IconTheme::default();
                        } else {
                            return Err(anyhow!(
                                "'{}' is an invalid argument for '--icons'\n\
                                 possible arguments are ['always', 'auto', 'never']",
                                when.to_string_lossy()
                            ));
                        }
                    }
                    None => self.icons = IconTheme::with_default_icons(),
                },
                Long("iec") => {
                    self.size_format = SizeFormat::Iec;
                    self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                }
                Long("ignore-file") => {
                    self.ignore_file = true;
                }
                Short('k') | Long("kibibytes") => {
                    if self.size_format.is_raw() {
                        self.allocated_size_blocks = AllocatedSizeBlocks::Kibibytes;
                    }
                }
                Short('l') => {
                    self.output_format = OutputFormat::Long;
                }
                Short('L') | Long("dereference") => {
                    self.dereference = true;
                    self.dereference_cmdline_symlink = true;
                    self.dereference_cmdline_symlink_dir = true;
                }
                Long("max-depth") => {
                    let val: usize = parser.value()?.parse()?;
                    self.max_depth = Some(val);
                }
                Long("mode") => {
                    let word = parser.value()?;

                    if word == "native" {
                        self.mode_format.set_native();
                    } else if word == "pwsh" {
                        self.mode_format = ModeFormat::Pwsh;
                    } else if word == "rwx" {
                        self.mode_format = ModeFormat::Rwx;
                    } else {
                        return Err(anyhow!(
                            "'{}' is an invalid argument for '--mode'\n\
                             possible arguments are ['native', 'pwsh', 'rwx']",
                            word.to_string_lossy()
                        ));
                    }
                }
                Short('n') | Long("numeric-uid-gid") => {
                    self.numeric_uid_gid = true;
                    self.output_format = OutputFormat::Long;
                }
                Short('o') => {
                    self.list_group = false;
                    self.output_format = OutputFormat::Long;
                }
                Short('p') => {
                    self.indicator_style = IndicatorStyle::Slash;
                }
                Short('r') | Long("reverse") => {
                    self.reverse = true;
                }
                Short('R') | Long("recursive") => {
                    self.recursive = true;
                }
                Short('s') | Long("size") => {
                    self.list_allocated_size = true;
                }
                Short('S') => {
                    self.sorting_order = SortingOrder::Size;
                }
                Long("si") => {
                    self.size_format = SizeFormat::Si;
                    self.allocated_size_blocks = AllocatedSizeBlocks::Raw;
                }
                Short('t') => {
                    self.sorting_order = SortingOrder::Timestamp;
                }
                Long("time") => {
                    let word = parser.value()?;

                    if word == "accessed" || word == "atime" {
                        self.timestamp_used = TimestampUsed::Accessed;
                    } else if word == "changed" || word == "ctime" {
                        self.timestamp_used = TimestampUsed::Changed;
                    } else if word == "created" || word == "btime" {
                        self.timestamp_used = TimestampUsed::Created;
                    } else if word == "modified" || word == "mtime" {
                        self.timestamp_used = TimestampUsed::Modified;
                    } else {
                        return Err(anyhow!(
                            "'{}' is an invalid argument for '--time'\n\
                             possible arguments are ['accessed', 'changed', 'created', 'modified', 'atime', 'ctime', 'btime', 'mtime']",
                            word.to_string_lossy()
                        ));
                    }
                }
                Short('u') => {
                    self.timestamp_used = TimestampUsed::Accessed;
                }
                Long("version") => {
                    println!("{}", VERSION);
                    process::exit(0);
                }
                Short('x') => {
                    self.output_format = OutputFormat::Across;
                }
                Short('1') => {
                    self.output_format = OutputFormat::SingleColumn;
                }
                _ => {
                    return Err(anyhow!(arg.unexpected()));
                }
            }
        }

        Ok(())
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
            icons: IconTheme::default(),
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
