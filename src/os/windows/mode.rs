use std::fs::FileType;

use super::sys_prelude::*;

use crate::output::DisplayCell;
use crate::utils::HasMaskSetExt;

pub fn pwsh_mode_cell(file_attributes: Option<u32>) -> DisplayCell {
    match file_attributes {
        Some(file_attributes) => {
            let mut cell = DisplayCell::with_capacity(6);

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_DIRECTORY) {
                cell.push_char('d');
            } else {
                cell.push_char('-');
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_ARCHIVE) {
                cell.push_char('a');
            } else {
                cell.push_char('-');
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_READONLY) {
                cell.push_char('r');
            } else {
                cell.push_char('-');
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_HIDDEN) {
                cell.push_char('h');
            } else {
                cell.push_char('-');
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_SYSTEM) {
                cell.push_char('s');
            } else {
                cell.push_char('-');
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_REPARSE_POINT) {
                cell.push_char('l');
            } else {
                cell.push_char('-');
            }

            cell
        }
        None => DisplayCell::from_ascii_string(String::from("??????"), true),
    }
}

pub fn rwx_mode_cell(file_type: Option<FileType>, rwx_permissions: &str) -> DisplayCell {
    let mut cell = DisplayCell::with_capacity(10);

    match file_type {
        Some(file_type) => {
            if file_type.is_file() {
                cell.push_char('-');
            } else if file_type.is_dir() {
                cell.push_char('d');
            } else if file_type.is_symlink() {
                cell.push_char('l');
            } else {
                cell.push_char('?');
            }
        }
        None => cell.push_char('?'),
    }

    cell.push_ascii_str(rwx_permissions);

    cell
}
