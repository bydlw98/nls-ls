use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt};
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use compact_str::{CompactString, ToCompactString};
use nls_term_grid::Alignment;

use crate::config::{Config, TimestampUsed};
#[cfg(unix)]
use crate::os::unix::*;
#[cfg(windows)]
use crate::os::windows::*;
use crate::output::*;
use crate::utils::systemtime_to_unix_timestamp;

#[derive(Debug, Default)]
pub struct EntryBuf {
    file_name: CompactString,
    file_name_key: String,
    path: PathBuf,
    metadata: Option<Metadata>,
    allocated_size: Option<u64>,
    size: Option<u64>,
    timestamp: Option<i64>,
    #[cfg(unix)]
    ino: Option<u64>,
    #[cfg(windows)]
    windows_metadata: WindowsMetadata,
    #[cfg(windows)]
    follow_links: bool,
}

impl EntryBuf {
    pub fn from_direntry(dent: ignore::DirEntry, config: &Config) -> Self {
        let file_name = if dent.depth() == 0 {
            CompactString::new_inline(".")
        } else {
            dent.file_name().to_string_lossy().to_compact_string()
        };

        let follow_links = if cfg!(windows) {
            dent.path_is_symlink() && config.dereference
        } else {
            config.dereference
        };
        let metadata = if follow_links {
            match dent.path().metadata() {
                Ok(metadata) => Some(metadata),
                Err(err) => {
                    eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                    None
                }
            }
        } else {
            match dent.metadata() {
                Ok(metadata) => Some(metadata),
                Err(err) => {
                    eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                    None
                }
            }
        };

        #[cfg(unix)]
        let ino = if config.dereference { None } else { dent.ino() };

        let mut entrybuf = Self {
            file_name: file_name,
            path: dent.into_path(),
            metadata: metadata,
            #[cfg(unix)]
            ino: ino,
            #[cfg(windows)]
            follow_links: follow_links,
            ..Default::default()
        };
        entrybuf.init(config);

        entrybuf
    }

    pub fn from_cmdline_path(path: &Path, config: &Config) -> Self {
        let file_name = path.display().to_compact_string();
        let metadata_result = if config.dereference_cmdline_symlink {
            path.metadata()
        } else {
            path.symlink_metadata()
        };
        let metadata = match metadata_result {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                None
            }
        };

        let mut entrybuf = Self {
            file_name: file_name,
            path: path.to_path_buf(),
            metadata: metadata,
            #[cfg(windows)]
            follow_links: config.dereference_cmdline_symlink,
            ..Default::default()
        };
        entrybuf.init(config);

        entrybuf
    }

    pub fn from_named_path(path_name: &str, path: &Path, config: &Config) -> Self {
        let metadata_result = if config.dereference_cmdline_symlink {
            path.metadata()
        } else {
            path.symlink_metadata()
        };
        let metadata = match metadata_result {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '..': {}", err);
                None
            }
        };

        let mut entrybuf = Self {
            file_name: CompactString::from(path_name),
            path: path.to_path_buf(),
            metadata: metadata,
            #[cfg(windows)]
            follow_links: config.dereference_cmdline_symlink,
            ..Default::default()
        };
        entrybuf.init(config);

        entrybuf
    }

    pub fn init(&mut self, config: &Config) {
        self.file_name_key = self.file_name.to_lowercase();
        if let Some(metadata) = &self.metadata {
            self.size = Some(metadata.len());
        }

        #[cfg(unix)]
        self.init_unix(config);

        #[cfg(windows)]
        self.init_windows(config);

        #[cfg(not(any(unix, windows)))]
        self.init_others(config);
    }

    #[cfg(unix)]
    fn init_unix(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.ino = Some(metadata.ino());
            self.allocated_size = Some(get_allocated_size(metadata, config));

            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => Some(metadata.atime()),
                TimestampUsed::Changed => Some(metadata.ctime()),
                TimestampUsed::Created => systemtime_to_unix_timestamp(metadata.created()),
                TimestampUsed::Modified => Some(metadata.mtime()),
            };
        }
    }

    #[cfg(windows)]
    fn init_windows(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => systemtime_to_unix_timestamp(metadata.accessed()),
                TimestampUsed::Changed => None,
                TimestampUsed::Created => systemtime_to_unix_timestamp(metadata.created()),
                TimestampUsed::Modified => systemtime_to_unix_timestamp(metadata.modified()),
            };
        }

        self.windows_metadata = WindowsMetadata::get(&self.path, self.follow_links, config);
        self.allocated_size = self.windows_metadata.allocated_size(config);

        if let Some(size) = self.windows_metadata.size() {
            self.size = Some(size);
        }
    }

    #[cfg(not(any(unix, windows)))]
    fn init_others(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => systemtime_to_unix_timestamp(metadata.accessed()),
                TimestampUsed::Changed => None,
                TimestampUsed::Created => systemtime_to_unix_timestamp(metadata.created()),
                TimestampUsed::Modified => systemtime_to_unix_timestamp(metadata.modified()),
            };
        }
    }

    pub fn file_name_key(&self) -> &str {
        &self.file_name_key
    }

    pub fn file_name_cell(&self, config: &Config) -> GridCell {
        match &self.metadata {
            Some(metadata) => format_filename(&self.path, &self.file_name, metadata, config),
            None => GridCell::from_str_with_style(&self.file_name, None),
        }
    }

    #[cfg(unix)]
    pub fn ino_cell(&self, config: &Config) -> GridCell {
        let inode_style = config.theme.inode_style();

        match &self.ino {
            Some(ino) => GridCell::from_num_with_style(*ino, inode_style),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    #[cfg(windows)]
    pub fn ino_cell(&self, config: &Config) -> GridCell {
        let inode_style = config.theme.inode_style();

        match get_file_id_identifier(&self.path, self.follow_links) {
            Ok(file_id) => GridCell::from_num_with_style(file_id, inode_style),
            Err(err) => {
                eprintln!(
                    "nls: unable to get inode number of '{}': {}",
                    self.path.display(),
                    err
                );

                GridCell::error_cell(Alignment::Right)
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn ino_cell(&self, config: &Config) -> GridCell {
        let inode_style = config.theme.inode_style();

        match &self.metadata {
            Some(_) => GridCell::from_ascii_str_with_style('-', inode_style),
            None => GridCell::error_cell(false),
        }
    }

    pub fn allocated_size(&self) -> Option<u64> {
        self.allocated_size
    }

    pub fn allocated_size_cell(&self, config: &Config) -> GridCell {
        match &self.allocated_size {
            Some(allocated_size) => format_size(*allocated_size, config),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    #[cfg(unix)]
    pub fn mode_cell(&self, config: &Config) -> GridCell {
        match &self.metadata {
            Some(metadata) => {
                if config.mode_format.is_rwx() {
                    rwx_mode_cell(metadata.mode(), config)
                } else {
                    pwsh_mode_cell(metadata.mode(), &self.file_name, &self.path, config)
                }
            }
            None => GridCell::from_ascii_str_with_style("??????????", None),
        }
    }

    #[cfg(windows)]
    pub fn mode_cell(&self, config: &Config) -> GridCell {
        if config.mode_format.is_pwsh() {
            pwsh_mode_cell(
                self.metadata
                    .as_ref()
                    .map(|metadata| Some(metadata.file_attributes()))
                    .unwrap_or(None),
                config,
            )
        } else {
            self.windows_metadata.rwx_mode_cell(
                self.metadata
                    .as_ref()
                    .map(|metadata| Some(metadata.file_type()))
                    .unwrap_or(None),
                config,
            )
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn mode_cell(&self, config: &Config) -> GridCell {
        match &self.metadata {
            Some(metadata) => {
                let file_type = metadata.file_type();
                let ls_colors = &config.ls_colors;
                if file_type.is_file() {
                    GridCell::from_ascii_str_with_style("-", ls_colors.file_style())
                } else if file_type.is_dir() {
                    GridCell::from_ascii_str_with_style("d", ls_colors.dir_style())
                } else if file_type.is_symlink() {
                    GridCell::from_ascii_str_with_style("l", ls_colors.symlink_style())
                } else {
                    GridCell::from_ascii_str_with_style("?", None)
                }
            }
            None => GridCell::from_ascii_str_with_style("?", None),
        }
    }

    #[cfg(unix)]
    pub fn nlink_cell(&self, config: &Config) -> GridCell {
        let nlink_style = config.theme.nlink_style();
        match &self.metadata {
            Some(metadata) => GridCell::from_num_with_style(metadata.nlink(), nlink_style),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    #[cfg(windows)]
    pub fn nlink_cell(&self, config: &Config) -> GridCell {
        self.windows_metadata.nlink_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn nlink_cell(&self, config: &Config) -> GridCell {
        let nlink_style = config.theme.nlink_style();
        match &self.metadata {
            Some(_) => GridCell::from_ascii_str_with_style('1', nlink_style),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    #[cfg(unix)]
    pub fn owner_cell(&self, config: &Config) -> GridCell {
        match &self.metadata {
            Some(metadata) => get_username_cell_by_uid(metadata.uid(), config),
            None => GridCell::error_cell(Alignment::Left),
        }
    }

    #[cfg(windows)]
    pub fn owner_cell(&self, config: &Config) -> GridCell {
        self.windows_metadata.owner_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn owner_cell(&self, config: &Config) -> GridCell {
        let owner_style = config.theme.owner_style();
        match &self.metadata {
            Some(_) => GridCell::from_ascii_str_with_style("-", owner_style),
            None => GridCell::error_cell(Alignment::Left),
        }
    }

    #[cfg(unix)]
    pub fn group_cell(&self, config: &Config) -> GridCell {
        match &self.metadata {
            Some(metadata) => get_groupname_cell_by_gid(metadata.gid(), config),
            None => GridCell::error_cell(Alignment::Left),
        }
    }

    #[cfg(windows)]
    pub fn group_cell(&self, config: &Config) -> GridCell {
        self.windows_metadata.group_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn group_cell(&self, config: &Config) -> GridCell {
        let group_style = config.theme.group_style();
        match &self.metadata {
            Some(_) => GridCell::from_ascii_str_with_style("-", group_style),
            None => GridCell::error_cell(true),
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn size_cell(&self, config: &Config) -> GridCell {
        #[cfg(unix)]
        if let Some(metadata) = &self.metadata {
            let file_type = metadata.file_type();
            if file_type.is_block_device() || file_type.is_char_device() {
                return format_rdev(metadata.rdev(), config);
            }
        }

        match &self.size {
            Some(size) => format_size(*size, config),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn timestamp_cell(&self, config: &Config) -> GridCell {
        match &self.timestamp {
            Some(timestamp) => format_timestamp(*timestamp, config),
            None => GridCell::error_cell(Alignment::Left),
        }
    }
}
