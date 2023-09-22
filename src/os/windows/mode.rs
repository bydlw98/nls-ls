use std::fs::FileType;

use crate::output::DisplayCell;

pub fn rwx_mode_cell(file_type: Option<FileType>, rwx_permissions: &str) -> DisplayCell {
    let mut cell = DisplayCell::with_capacity(10);

    match file_type {
        Some(file_type) => {
            if file_type.is_file() {
                cell.push_char('-');
            } else if file_type.is_dir() {
                cell.push_char('d');
            } else if file_type.is_symlink() {
                cell.push_char('-');
            } else {
                cell.push_char('?');
            }
        }
        None => cell.push_char('?'),
    }

    cell.push_ascii_str(rwx_permissions);

    cell
}
