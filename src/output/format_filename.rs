use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::config::{Config, IndicatorStyle};
#[cfg(unix)]
use crate::os::unix::sys_prelude::*;
use crate::utils::HasMaskSetExt;

use super::DisplayCell;

pub fn format_filename(file_name: &str, metadata: &Metadata, config: &Config) -> DisplayCell {
    if config.color {
        internal_format_filename_with_color(file_name, metadata, config)
    } else {
        internal_format_filename(file_name, metadata, config)
    }
}

#[cfg(unix)]
pub fn internal_format_filename(
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    const EXEC_MASK: u32 = c::S_IXUSR | c::S_IXGRP | c::S_IXOTH;
    let indicator_style = config.indicator_style;
    let st_mode = metadata.mode();
    let mut filename_cell = DisplayCell::from(String::from(file_name));

    match st_mode & c::S_IFMT {
        c::S_IFREG => {
            if indicator_style.others() && st_mode.has_bit_in_mask_set(EXEC_MASK) {
                filename_cell.push_char(IndicatorStyle::EXEC);
            }
        }
        c::S_IFDIR => {
            if indicator_style.dir() {
                filename_cell.push_char(IndicatorStyle::DIR);
            }
        }

        c::S_IFLNK => {
            if indicator_style.others() && !config.output_format.is_long() {
                filename_cell.push_char(IndicatorStyle::SYMLINK);
            }
        }

        c::S_IFIFO => {
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::FIFO);
            }
        }

        c::S_IFSOCK => {
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::SOCKET);
            }
        }

        _ => (),
    }

    filename_cell
}

#[cfg(unix)]
pub fn internal_format_filename_with_color(
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;
    let st_mode = metadata.mode();
    let mut filename_cell = DisplayCell::from(String::from(file_name));

    match st_mode & c::S_IFMT {
        c::S_IFREG => {
            file_format_with_color(&mut filename_cell, st_mode, metadata.nlink(), config);
        }
        c::S_IFDIR => {
            dir_format_with_color(&mut filename_cell, st_mode, config);
        }

        c::S_IFLNK => {
            filename_cell.paint(&ls_colors.symlink);
            if indicator_style.others() && !config.output_format.is_long() {
                filename_cell.push_char(IndicatorStyle::SYMLINK);
            }
        }

        c::S_IFIFO => {
            filename_cell.paint(&ls_colors.fifo);
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::FIFO);
            }
        }

        c::S_IFSOCK => {
            filename_cell.paint(&ls_colors.socket);
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::SOCKET);
            }
        }

        _ => (),
    }

    filename_cell
}

#[cfg(unix)]
fn file_format_with_color(
    filename_cell: &mut DisplayCell,
    st_mode: u32,
    nlink: u64,
    config: &Config,
) {
    const EXEC_MASK: u32 = c::S_IXUSR | c::S_IXGRP | c::S_IXOTH;
    let indicator_style = &config.indicator_style;
    let ls_colors = &config.ls_colors;

    if st_mode.has_mask_set(c::S_ISUID) {
        filename_cell.paint(&ls_colors.setuid);
    } else if st_mode.has_mask_set(c::S_ISGID) {
        filename_cell.paint(&ls_colors.setgid);
    } else if st_mode.has_mask_set(EXEC_MASK) {
        filename_cell.paint(&ls_colors.exec);
    } else if nlink > 1 {
        filename_cell.paint(&ls_colors.multiple_hard_links);
    } else {
        let extension = get_file_extension(filename_cell.contents());
        match ls_colors.extension.get(&extension.to_string()) {
            Some(ansi_style_str) => filename_cell.paint(ansi_style_str),
            None => filename_cell.paint(&ls_colors.file),
        }
    }

    if indicator_style.others() && st_mode.has_bit_in_mask_set(EXEC_MASK) {
        filename_cell.push_char(IndicatorStyle::EXEC);
    }
}

#[cfg(unix)]
fn dir_format_with_color(filename_cell: &mut DisplayCell, st_mode: u32, config: &Config) {
    let indicator_style = &config.indicator_style;
    let ls_colors = &config.ls_colors;

    match (
        st_mode.has_mask_set(c::S_ISVTX),
        st_mode.has_mask_set(c::S_IWOTH),
    ) {
        (false, false) => filename_cell.paint(&ls_colors.dir),
        (true, false) => filename_cell.paint(&ls_colors.dir_sticky),
        (false, true) => filename_cell.paint(&ls_colors.dir_other_writeable),
        _ => filename_cell.paint(&ls_colors.dir_sticky_and_other_writable),
    }

    if indicator_style.dir() {
        filename_cell.push_char(IndicatorStyle::DIR);
    }
}

#[cfg(not(unix))]
pub fn internal_format_filename(file_name: &str, metadata: &Metadata, config: &Config) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let file_type = metadata.file_type();
    let mut filename_cell = DisplayCell::from(String::from(file_name));

    if file_type.is_dir() {
        if indicator_style.dir() {
            filename_cell.push_char(IndicatorStyle::DIR);
        }
    } else if file_type.is_symlink() {
        let mut filename_cell = DisplayCell::from(String::from(file_name));
        if indicator_style.others() && !config.output_format.is_long() {
            filename_cell.push_char(IndicatorStyle::SYMLINK);
        }
    }

    filename_cell
}

#[cfg(not(unix))]
pub fn internal_format_filename_with_color(
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    let indicator_style = &config.indicator_style;
    let file_type = metadata.file_type();
    let ls_colors = &config.ls_colors;
    let mut filename_cell = DisplayCell::from(file_name.to_string());

    if file_type.is_dir() {
        filename_cell.paint(&ls_colors.dir);
        if indicator_style.dir() {
            filename_cell.push_char(IndicatorStyle::DIR);
        }
    } else if file_type.is_symlink() {
        filename_cell.paint(&ls_colors.symlink);
        if indicator_style.others() && !config.output_format.is_long() {
            filename_cell.push_char(IndicatorStyle::SYMLINK);
        }
    } else if file_type.is_file() {
        let extension = get_file_extension(filename_cell.contents());
        match ls_colors.extension.get(&extension.to_string()) {
            Some(ansi_style_str) => filename_cell.paint(ansi_style_str),
            None => filename_cell.paint(&ls_colors.file),
        }
    }

    filename_cell
}

fn get_file_extension(file_name: &str) -> &str {
    match file_name.rsplit_once('.') {
        Some((file_name_without_extension, extension)) => {
            if file_name_without_extension.is_empty() {
                ""
            } else {
                extension
            }
        }
        None => "",
    }
}
