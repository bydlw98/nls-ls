use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::config::Config;
#[cfg(unix)]
use crate::os::unix::*;
use crate::output::DisplayCell;

#[derive(Debug, Default)]
pub struct EntryBuf {
    file_name: String,
    file_name_key: String,
    path: PathBuf,
    metadata: Option<Metadata>,
    size: Option<u64>,
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

        let mut entrybuf = Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: dent.into_path(),
            metadata: metadata,
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
    }

    pub fn file_name_key(&self) -> &str {
        &self.file_name_key
    }

    pub fn file_name_cell(&self) -> DisplayCell {
        DisplayCell::from(self.file_name.clone())
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
            None => DisplayCell::from_ascii_string(String::from("?"), false),
        }
    }

    #[cfg(not(unix))]
    pub fn nlink_cell(&self) -> DisplayCell {
        DisplayCell::from_ascii_string(1.to_string(), false)
    }

    pub fn size_cell(&self) -> DisplayCell {
        match &self.size {
            Some(size) => DisplayCell::from_ascii_string(size.to_string(), false),
            None => DisplayCell::from_ascii_string(String::from("?"), false),
        }
    }
}
