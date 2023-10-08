use std::fs::{self, Metadata};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::config::{Config, TimestampUsed};
#[cfg(unix)]
use crate::os::unix::*;
#[cfg(windows)]
use crate::os::windows::*;
use crate::output::*;
use crate::utils::systemtime_to_unix_timestamp;

#[derive(Debug, Default)]
pub struct EntryBuf {
    file_name: String,
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
            String::from(".")
        } else {
            dent.file_name().to_string_lossy().to_string()
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
        let file_name = path.display().to_string();
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

    pub fn from_parent_of_path(path: &Path, config: &Config) -> Self {
        let parent_path = path.join("..");
        let metadata_result = if config.dereference_cmdline_symlink {
            parent_path.metadata()
        } else {
            parent_path.symlink_metadata()
        };
        let metadata = match metadata_result {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '..': {}", err);
                None
            }
        };

        let mut entrybuf = Self {
            file_name: String::from(".."),
            path: parent_path,
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

    fn get_symlink_target_cell(&self, config: &Config) -> DisplayCell {
        let mut symlink_target_cell = DisplayCell::with_capacity(128);
        symlink_target_cell.push_str_with_width(" -> ", 4);

        match fs::read_link(&self.path) {
            Ok(target_name) => match self.path.metadata() {
                Ok(target_metadata) => {
                    let target_file_name_cell =
                        format_filename(&target_name.to_string_lossy(), &target_metadata, config);
                    symlink_target_cell.append(target_file_name_cell);
                }
                Err(err) => {
                    symlink_target_cell.push_str(&target_name.to_string_lossy());
                    eprintln!(
                        "nls: unable to get link metadata of '{}': {}",
                        self.path.display(),
                        err
                    );
                }
            },
            Err(err) => {
                symlink_target_cell.push_char('?');
                eprintln!("nls: unable to readlink '{}': {}", self.path.display(), err);
            }
        }

        symlink_target_cell
    }

    pub fn file_name_key(&self) -> &str {
        &self.file_name_key
    }

    pub fn file_name_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => {
                let mut file_name_cell = format_filename(&self.file_name, metadata, config);

                if metadata.is_symlink() && config.output_format.is_long() {
                    file_name_cell.append(self.get_symlink_target_cell(config));
                }

                file_name_cell
            }
            None => DisplayCell::from(self.file_name.clone()),
        }
    }

    #[cfg(unix)]
    pub fn ino_cell(&self, config: &Config) -> DisplayCell {
        let inode_style = config.theme.inode_style();

        match &self.ino {
            Some(ino) => DisplayCell::from_num_with_style(*ino, inode_style),
            None => DisplayCell::error_cell(false),
        }
    }

    #[cfg(windows)]
    pub fn ino_cell(&self, config: &Config) -> DisplayCell {
        let inode_style = config.theme.inode_style();

        match get_file_id_identifier(&self.path, self.follow_links) {
            Ok(file_id) => DisplayCell::from_u128_with_style(file_id, inode_style),
            Err(err) => {
                eprintln!(
                    "nls: unable to get inode number of '{}': {}",
                    self.path.display(),
                    err
                );

                DisplayCell::error_cell(false)
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn ino_cell(&self, config: &Config) -> DisplayCell {
        let inode_style = config.theme.inode_style();

        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_str_with_style('-', inode_style),
            None => DisplayCell::error_cell(false),
        }
    }

    pub fn allocated_size(&self) -> Option<u64> {
        self.allocated_size
    }

    pub fn allocated_size_cell(&self, config: &Config) -> DisplayCell {
        match &self.allocated_size {
            Some(allocated_size) => format_size(*allocated_size, config),
            None => DisplayCell::error_cell(false),
        }
    }

    #[cfg(unix)]
    pub fn mode_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => {
                if config.mode_format.is_rwx() {
                    rwx_mode_cell(metadata.mode(), config)
                } else {
                    pwsh_mode_cell(metadata.mode(), &self.file_name, &self.path, config)
                }
            }
            None => DisplayCell::from_ascii_str_with_style("??????????", None),
        }
    }

    #[cfg(windows)]
    pub fn mode_cell(&self, config: &Config) -> DisplayCell {
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
    pub fn mode_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => {
                let file_type = metadata.file_type();
                let ls_colors = &config.ls_colors;
                if file_type.is_file() {
                    DisplayCell::from_ascii_str_with_style("-", ls_colors.file_style())
                } else if file_type.is_dir() {
                    DisplayCell::from_ascii_str_with_style("d", ls_colors.dir_style())
                } else if file_type.is_symlink() {
                    DisplayCell::from_ascii_str_with_style("l", ls_colors.symlink_style())
                } else {
                    DisplayCell::from_ascii_str_with_style("?", None)
                }
            }
            None => DisplayCell::from_ascii_str_with_style("?", None),
        }
    }

    #[cfg(unix)]
    pub fn nlink_cell(&self, config: &Config) -> DisplayCell {
        let nlink_style = config.theme.nlink_style();
        match &self.metadata {
            Some(metadata) => DisplayCell::from_num_with_style(metadata.nlink(), nlink_style),
            None => DisplayCell::error_cell(false),
        }
    }

    #[cfg(windows)]
    pub fn nlink_cell(&self, config: &Config) -> DisplayCell {
        self.windows_metadata.nlink_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn nlink_cell(&self, config: &Config) -> DisplayCell {
        let nlink_style = config.theme.nlink_style();
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_str_with_style('1', nlink_style),
            None => DisplayCell::error_cell(false),
        }
    }

    #[cfg(unix)]
    pub fn owner_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => get_username_cell_by_uid(metadata.uid(), config),
            None => DisplayCell::error_cell(true),
        }
    }

    #[cfg(windows)]
    pub fn owner_cell(&self, config: &Config) -> DisplayCell {
        self.windows_metadata.owner_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn owner_cell(&self, config: &Config) -> DisplayCell {
        let owner_style = config.theme.owner_style();
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_str_with_style("-", owner_style),
            None => DisplayCell::error_cell(true),
        }
    }

    #[cfg(unix)]
    pub fn group_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => get_groupname_cell_by_gid(metadata.gid(), config),
            None => DisplayCell::error_cell(true),
        }
    }

    #[cfg(windows)]
    pub fn group_cell(&self, config: &Config) -> DisplayCell {
        self.windows_metadata.group_cell(config)
    }

    #[cfg(not(any(unix, windows)))]
    pub fn group_cell(&self, config: &Config) -> DisplayCell {
        let group_style = config.theme.group_style();
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_str_with_style("-", group_style),
            None => DisplayCell::error_cell(true),
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn size_cell(&self, config: &Config) -> DisplayCell {
        match &self.size {
            Some(size) => format_size(*size, config),
            None => DisplayCell::error_cell(false),
        }
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn timestamp_cell(&self, config: &Config) -> DisplayCell {
        match &self.timestamp {
            Some(timestamp) => format_timestamp(*timestamp, config),
            None => DisplayCell::error_cell(false),
        }
    }
}
