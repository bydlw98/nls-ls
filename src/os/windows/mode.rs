use std::fs::FileType;

use super::sys_prelude::*;

use crate::config::Config;
use crate::output::{GridCell, GridCellExts};
use crate::utils::HasMaskSetExt;

pub fn pwsh_mode_cell(file_attributes: Option<u32>, config: &Config) -> GridCell {
    let ls_colors = &config.ls_colors;
    let theme = &config.theme;

    match file_attributes {
        Some(file_attributes) => {
            let mut cell = GridCell::with_capacity(6);

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_DIRECTORY) {
                cell.push_char_with_style('d', ls_colors.dir_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_ARCHIVE) {
                cell.push_char_with_style('a', theme.archive_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_READONLY) {
                cell.push_char_with_style('r', theme.read_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_HIDDEN) {
                cell.push_char_with_style('h', theme.hidden_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_SYSTEM) {
                cell.push_char_with_style('s', theme.system_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            if file_attributes.has_mask_set(c::FILE_ATTRIBUTE_REPARSE_POINT) {
                cell.push_char_with_style('l', ls_colors.symlink_style());
            } else {
                cell.push_char_with_style('-', theme.no_permission_style());
            }

            cell
        }
        None => GridCell::from_ascii_str_with_style("??????", None),
    }
}

pub fn rwx_mode_cell(
    file_type: Option<FileType>,
    rwx_permissions: &str,
    config: &Config,
) -> GridCell {
    let mut cell = GridCell::with_capacity(128);
    let ls_colors = &config.ls_colors;
    match file_type {
        Some(file_type) => {
            if file_type.is_file() {
                cell.push_char_with_style('-', ls_colors.file_style());
            } else if file_type.is_dir() {
                cell.push_char_with_style('d', ls_colors.dir_style());
            } else if file_type.is_symlink() {
                cell.push_char_with_style('l', ls_colors.symlink_style());
            } else {
                cell.push_char('?');
            }
        }
        None => cell.push_char('?'),
    }

    cell.push_str_with_width(rwx_permissions, 10);

    cell
}
