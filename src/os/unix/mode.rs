use std::path::Path;

use super::sys_prelude::*;

use crate::config::Config;
use crate::output::{GridCell, GridCellExts};
use crate::utils::HasMaskSetExt;

pub fn rwx_mode_cell(st_mode: u32, config: &Config) -> GridCell {
    let ls_colors = &config.ls_colors;
    let theme = &config.theme;
    let mut cell = GridCell::with_capacity(10);

    match st_mode & c::S_IFMT {
        c::S_IFREG => cell.push_char_with_style('-', ls_colors.file_style()),
        c::S_IFDIR => cell.push_char_with_style('d', ls_colors.dir_style()),
        c::S_IFLNK => cell.push_char_with_style('l', ls_colors.symlink_style()),
        c::S_IFBLK => cell.push_char_with_style('b', ls_colors.block_device_style()),
        c::S_IFCHR => cell.push_char_with_style('c', ls_colors.char_device_style()),
        c::S_IFIFO => cell.push_char_with_style('p', ls_colors.file_style()),
        c::S_IFSOCK => cell.push_char_with_style('s', ls_colors.socket_style()),
        _ => cell.push_char('?'),
    }

    if st_mode.has_mask_set(c::S_IRUSR) {
        cell.push_char_with_style('r', theme.read_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    if st_mode.has_mask_set(c::S_IWUSR) {
        cell.push_char_with_style('w', theme.write_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    match (
        st_mode.has_mask_set(c::S_IXUSR),
        st_mode.has_mask_set(c::S_ISUID),
    ) {
        (false, false) => cell.push_char_with_style('-', theme.no_permission_style()),
        (true, false) => cell.push_char_with_style('x', theme.execute_style()),
        (false, true) => cell.push_char_with_style('S', theme.setuid_style()),
        _ => cell.push_char_with_style('s', theme.setuid_style()),
    }

    if st_mode.has_mask_set(c::S_IRGRP) {
        cell.push_char_with_style('r', theme.read_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    if st_mode.has_mask_set(c::S_IWGRP) {
        cell.push_char_with_style('w', theme.write_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    match (
        st_mode.has_mask_set(c::S_IXGRP),
        st_mode.has_mask_set(c::S_ISGID),
    ) {
        (false, false) => cell.push_char_with_style('-', theme.no_permission_style()),
        (true, false) => cell.push_char_with_style('x', theme.execute_style()),
        (false, true) => cell.push_char_with_style('S', theme.setgid_style()),
        _ => cell.push_char_with_style('s', theme.setgid_style()),
    }

    if st_mode.has_mask_set(c::S_IROTH) {
        cell.push_char_with_style('r', theme.read_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    if st_mode.has_mask_set(c::S_IWOTH) {
        cell.push_char_with_style('w', theme.write_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    };

    match (
        st_mode.has_mask_set(c::S_IXOTH),
        st_mode.has_mask_set(c::S_ISVTX),
    ) {
        (false, false) => cell.push_char_with_style('-', theme.no_permission_style()),
        (true, false) => cell.push_char_with_style('x', theme.execute_style()),
        (false, true) => cell.push_char_with_style('T', theme.sticky_style()),
        _ => cell.push_char_with_style('t', theme.sticky_style()),
    }

    cell
}

pub fn pwsh_mode_cell(st_mode: u32, file_name: &str, path: &Path, config: &Config) -> GridCell {
    let mut cell = GridCell::with_capacity(6);
    let file_type_mask = st_mode & c::S_IFMT;
    let is_symlink = file_type_mask == c::S_IFLNK;
    let ls_colors = &config.ls_colors;
    let theme = &config.theme;

    // directory or symlink directory attribute
    if file_type_mask == c::S_IFDIR {
        cell.push_char_with_style('d', ls_colors.dir_style());
    } else if file_type_mask == c::S_IFLNK {
        if path
            .metadata()
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
        {
            cell.push_char_with_style('d', ls_colors.dir_style());
        } else {
            cell.push_char_with_style('-', theme.no_permission_style());
        }
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    }

    // archive attribute
    cell.push_char_with_style('-', theme.no_permission_style());

    // readonly attribute
    if !st_mode.has_bit_in_mask_set(c::S_IWUSR | c::S_IWGRP | c::S_IWOTH) {
        cell.push_char_with_style('r', theme.read_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    }

    // hidden attribute
    if file_name.starts_with('.') {
        cell.push_char_with_style('h', theme.hidden_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    }

    // system attribute
    cell.push_char_with_style('-', theme.no_permission_style());

    // symlink attribute
    if is_symlink {
        cell.push_char_with_style('l', ls_colors.symlink_style());
    } else {
        cell.push_char_with_style('-', theme.no_permission_style());
    }

    cell
}
