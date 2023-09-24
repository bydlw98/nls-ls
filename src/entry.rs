use std::fs::Metadata;
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
use crate::utils::get_unix_timestamp_from_systemtime;

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
}

impl EntryBuf {
    pub fn from_direntry(dent: ignore::DirEntry, config: &Config) -> Self {
        let file_name = dent.file_name().to_string_lossy().to_string();
        let metadata = match dent.metadata() {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                None
            }
        };

        #[cfg(unix)]
        let ino = dent.ino();

        let mut entrybuf = Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: dent.into_path(),
            metadata: metadata,
            #[cfg(unix)]
            ino: ino,
            ..Default::default()
        };
        entrybuf.load_metadata(config);

        entrybuf
    }

    pub fn from_path(path: &Path, config: &Config) -> Self {
        let file_name = path.display().to_string();
        let metadata = match path.symlink_metadata() {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                None
            }
        };

        let mut entrybuf = Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: path.to_path_buf(),
            metadata: metadata,
            ..Default::default()
        };
        entrybuf.load_metadata(config);

        entrybuf
    }

    pub fn from_path_with_file_name(file_name: String, path: &Path, config: &Config) -> Self {
        let metadata = match path.symlink_metadata() {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("nls: unable to get metadata of '{}': {}", file_name, err);
                None
            }
        };

        let mut entrybuf = Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: path.to_path_buf(),
            metadata: metadata,
            ..Default::default()
        };
        entrybuf.load_metadata(config);

        entrybuf
    }

    pub fn load_metadata(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.size = Some(metadata.len());
        }

        #[cfg(unix)]
        self.load_unix_metadata(config);

        #[cfg(windows)]
        self.load_windows_metadata(config);

        #[cfg(not(any(unix, windows)))]
        self.load_other_metadata(config);
    }

    #[cfg(unix)]
    pub fn load_unix_metadata(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.ino = Some(metadata.ino());
            self.allocated_size = Some(get_allocated_size(metadata, config));

            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => Some(metadata.atime()),
                TimestampUsed::Changed => Some(metadata.ctime()),
                TimestampUsed::Created => get_unix_timestamp_from_systemtime(metadata.created()),
                TimestampUsed::Modified => Some(metadata.mtime()),
            };
        }
    }

    #[cfg(windows)]
    pub fn load_windows_metadata(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => get_unix_timestamp_from_systemtime(metadata.accessed()),
                TimestampUsed::Changed => None,
                TimestampUsed::Created => get_unix_timestamp_from_systemtime(metadata.created()),
                TimestampUsed::Modified => get_unix_timestamp_from_systemtime(metadata.modified()),
            };
        }

        self.windows_metadata = WindowsMetadata::get(&self.path, config);
        self.allocated_size = self.windows_metadata.allocated_size(config);

        if let Some(size) = self.windows_metadata.size() {
            self.size = Some(size);
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn load_other_metadata(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.timestamp = match config.timestamp_used {
                TimestampUsed::Accessed => get_unix_timestamp_from_systemtime(metadata.accessed()),
                TimestampUsed::Changed => None,
                TimestampUsed::Created => get_unix_timestamp_from_systemtime(metadata.created()),
                TimestampUsed::Modified => get_unix_timestamp_from_systemtime(metadata.modified()),
            };
        }
    }

    fn get_symlink_target_cell(&self, config: &Config) -> DisplayCell {
        let mut symlink_target_cell = DisplayCell::with_capacity(128);
        symlink_target_cell.push_ascii_str(" -> ");

        match std::fs::read_link(&self.path) {
            Ok(symlink_target_name) => match self.path.metadata() {
                Ok(symlink_target_metadata) => {
                    let symlink_target_file_name_cell = format_filename(
                        &symlink_target_name.to_string_lossy(),
                        &symlink_target_metadata,
                        config,
                    );
                    symlink_target_cell.append(symlink_target_file_name_cell);
                }
                Err(err) => {
                    symlink_target_cell.push_str(&symlink_target_name.to_string_lossy());
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
    pub fn ino_cell(&self) -> DisplayCell {
        match &self.ino {
            Some(ino) => DisplayCell::from_ascii_string(ino.to_string(), false),
            None => DisplayCell::error_right_aligned(),
        }
    }

    #[cfg(windows)]
    pub fn ino_cell(&self) -> DisplayCell {
        match get_file_id_by_path(&self.path) {
            Ok(file_id) => DisplayCell::from_ascii_string(file_id.to_string(), false),
            Err(err) => {
                eprintln!(
                    "nls: unable to get inode number of '{}': {}",
                    self.path.display(),
                    err
                );

                DisplayCell::error_right_aligned()
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn ino_cell(&self) -> DisplayCell {
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_string('-'.to_string(), false),
            None => DisplayCell::error_right_aligned(),
        }
    }

    pub fn allocated_size(&self) -> Option<u64> {
        self.allocated_size
    }

    pub fn allocated_size_cell(&self, config: &Config) -> DisplayCell {
        match &self.allocated_size {
            Some(allocated_size) => format_size(*allocated_size, config),
            None => DisplayCell::error_right_aligned(),
        }
    }

    #[cfg(unix)]
    pub fn mode_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => rwx_mode_cell(metadata.mode()),
            None => DisplayCell::from_ascii_string(String::from("??????????"), true),
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
            )
        } else {
            self.windows_metadata.rwx_mode_cell(
                self.metadata
                    .as_ref()
                    .map(|metadata| Some(metadata.file_type()))
                    .unwrap_or(None),
            )
        }
    }

    #[cfg(not(any(unix, windows)))]
    pub fn mode_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => {
                let file_type = metadata.file_type();
                if file_type.is_file() {
                    DisplayCell::from_ascii_string(String::from("-"), true)
                } else if file_type.is_dir() {
                    DisplayCell::from_ascii_string(String::from("d"), true)
                } else if file_type.is_symlink() {
                    DisplayCell::from_ascii_string(String::from("l"), true)
                } else {
                    DisplayCell::from_ascii_string(String::from("?"), true)
                }
            }
            None => DisplayCell::from_ascii_string(String::from("?"), true),
        }
    }

    #[cfg(unix)]
    pub fn nlink_cell(&self) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => DisplayCell::from_ascii_string(metadata.nlink().to_string(), false),
            None => DisplayCell::error_right_aligned(),
        }
    }

    #[cfg(windows)]
    pub fn nlink_cell(&self) -> DisplayCell {
        self.windows_metadata.nlink_cell()
    }

    #[cfg(not(any(unix, windows)))]
    pub fn nlink_cell(&self) -> DisplayCell {
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_string(1.to_string(), false),
            None => DisplayCell::error_right_aligned(),
        }
    }

    #[cfg(unix)]
    pub fn owner_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => get_username_cell_by_uid(metadata.uid(), config),
            None => DisplayCell::error_left_aligned(),
        }
    }

    #[cfg(windows)]
    pub fn owner_cell(&self, _config: &Config) -> DisplayCell {
        self.windows_metadata.owner_cell()
    }

    #[cfg(not(any(unix, windows)))]
    pub fn owner_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_string(String::from("-"), true),
            None => DisplayCell::error_left_aligned(),
        }
    }

    #[cfg(unix)]
    pub fn group_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => get_groupname_cell_by_gid(metadata.gid(), config),
            None => DisplayCell::error_left_aligned(),
        }
    }

    #[cfg(windows)]
    pub fn group_cell(&self, _config: &Config) -> DisplayCell {
        self.windows_metadata.group_cell()
    }

    #[cfg(not(any(unix, windows)))]
    pub fn group_cell(&self, config: &Config) -> DisplayCell {
        match &self.metadata {
            Some(_) => DisplayCell::from_ascii_string(String::from("-"), true),
            None => DisplayCell::error_left_aligned(),
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn size_cell(&self, config: &Config) -> DisplayCell {
        match &self.size {
            Some(size) => format_size(*size, config),
            None => DisplayCell::error_right_aligned(),
        }
    }

    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    pub fn timestamp_cell(&self) -> DisplayCell {
        match &self.timestamp {
            Some(timestamp) => format_timestamp(*timestamp),
            None => DisplayCell::error_left_aligned(),
        }
    }
}
