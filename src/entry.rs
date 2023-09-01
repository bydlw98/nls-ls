use std::path::{Path, PathBuf};

use term_grid::Cell;

use crate::config::Config;

#[derive(Debug)]
pub struct EntryBuf {
    file_name: String,
    file_name_key: String,
    path: PathBuf,
}

impl EntryBuf {
    pub fn from_direntry(dent: ignore::DirEntry, config: &Config) -> Self {
        let file_name = dent.file_name().to_string_lossy().to_string();

        Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: dent.into_path(),
        }
    }

    pub fn from_path(path: &Path, config: &Config) -> Self {
        let file_name = path.display().to_string();

        Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: path.to_path_buf(),
        }
    }

    pub fn from_path_with_file_name(file_name: String, path: &Path, config: &Config) -> Self {
        Self {
            file_name_key: file_name.to_ascii_lowercase(),
            file_name: file_name,
            path: path.to_path_buf(),
        }
    }

    pub fn file_name_key(&self) -> &str {
        &self.file_name_key
    }

    pub fn file_name_cell(&self) -> Cell {
        Cell::from(self.file_name.clone())
    }
}
