use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::config::Config;
#[cfg(unix)]
use crate::os::unix::*;
use crate::output::*;
#[cfg(not(unix))]
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

        #[cfg(not(unix))]
        self.load_other_metadata();
    }

    #[cfg(unix)]
    pub fn load_unix_metadata(&mut self, config: &Config) {
        if let Some(metadata) = &self.metadata {
            self.ino = Some(metadata.ino());
            self.allocated_size = Some(get_allocated_size(metadata, config));
            self.timestamp = Some(metadata.mtime());
        }
    }

    #[cfg(not(unix))]
    pub fn load_other_metadata(&mut self) {
        if let Some(metadata) = &self.metadata {
            match metadata.modified() {
                Ok(modified) => {
                    self.timestamp = Some(get_unix_timestamp_from_systemtime(modified));
                }
                Err(err) => {
                    eprintln!(
                        "nls: unable to get modified timestamp of '{}': {}",
                        self.path.display(),
                        err
                    );
                }
            }
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

    #[cfg(not(unix))]
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
    pub fn mode_cell(&self) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => rwx_mode_cell(metadata.mode()),
            None => DisplayCell::from_ascii_string(String::from("??????????"), true),
        }
    }

    #[cfg(not(unix))]
    pub fn mode_cell(&self) -> DisplayCell {
        match &self.metadata {
            Some(metadata) => {
                let file_type = Metadata.file_type();
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

    #[cfg(not(unix))]
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

    #[cfg(not(unix))]
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

    #[cfg(not(unix))]
    pub fn group_cell(&self) -> DisplayCell {
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
