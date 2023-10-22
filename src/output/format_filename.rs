use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::config::{Config, IndicatorStyle};
use crate::ls_colors::get_file_extension;
#[cfg(unix)]
use crate::os::unix::sys_prelude::*;
#[cfg(unix)]
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

    match st_mode & c::S_IFMT {
        c::S_IFREG => file_format_with_color(file_name, st_mode, metadata.nlink(), config),
        c::S_IFDIR => dir_format_with_color(file_name, st_mode, config),

        c::S_IFLNK => {
            let mut filename_cell =
                DisplayCell::from_str_with_style(file_name, ls_colors.symlink_style());
            if indicator_style.others() && !config.output_format.is_long() {
                filename_cell.push_char(IndicatorStyle::SYMLINK);
            }

            filename_cell
        }

        c::S_IFBLK => DisplayCell::from_str_with_style(file_name, ls_colors.block_device_style()),
        c::S_IFCHR => DisplayCell::from_str_with_style(file_name, ls_colors.char_device_style()),

        c::S_IFIFO => {
            let mut filename_cell =
                DisplayCell::from_str_with_style(file_name, ls_colors.fifo_style());
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::FIFO);
            }

            filename_cell
        }

        c::S_IFSOCK => {
            let mut filename_cell =
                DisplayCell::from_str_with_style(file_name, ls_colors.socket_style());
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::SOCKET);
            }

            filename_cell
        }

        _ => DisplayCell::from(file_name.to_string()),
    }
}

#[cfg(unix)]
fn file_format_with_color(
    file_name: &str,
    st_mode: u32,
    nlink: u64,
    config: &Config,
) -> DisplayCell {
    const EXEC_MASK: u32 = c::S_IXUSR | c::S_IXGRP | c::S_IXOTH;
    let indicator_style = &config.indicator_style;
    let ls_colors = &config.ls_colors;

    if st_mode.has_mask_set(c::S_ISUID) {
        DisplayCell::from_str_with_style(file_name, ls_colors.setuid_style())
    } else if st_mode.has_mask_set(c::S_ISGID) {
        DisplayCell::from_str_with_style(file_name, ls_colors.setgid_style())
    } else if st_mode.has_mask_set(EXEC_MASK) {
        let mut filename_cell = DisplayCell::from_str_with_style(file_name, ls_colors.exec_style());
        if indicator_style.others() && st_mode.has_bit_in_mask_set(EXEC_MASK) {
            filename_cell.push_char(IndicatorStyle::EXEC);
        }

        filename_cell
    } else if nlink > 1 {
        DisplayCell::from_str_with_style(file_name, ls_colors.multiple_hard_links_style())
    } else {
        let extension = get_file_extension(file_name);
        if extension.is_empty() {
            DisplayCell::from_str_with_style(file_name, ls_colors.file_style())
        } else {
            DisplayCell::from_str_with_style(file_name, ls_colors.extension_style(extension))
        }
    }
}

#[cfg(unix)]
fn dir_format_with_color(file_name: &str, st_mode: u32, config: &Config) -> DisplayCell {
    let indicator_style = &config.indicator_style;
    let ls_colors = &config.ls_colors;

    let mut filename_cell = match (
        st_mode.has_mask_set(c::S_ISVTX),
        st_mode.has_mask_set(c::S_IWOTH),
    ) {
        (false, false) => DisplayCell::from_str_with_style(file_name, ls_colors.dir_style()),
        (true, false) => DisplayCell::from_str_with_style(file_name, ls_colors.dir_sticky_style()),
        (false, true) => {
            DisplayCell::from_str_with_style(file_name, ls_colors.dir_other_writeable_style())
        }
        _ => DisplayCell::from_str_with_style(
            file_name,
            ls_colors.dir_sticky_and_other_writable_style(),
        ),
    };

    if indicator_style.dir() {
        filename_cell.push_char(IndicatorStyle::DIR);
    }

    filename_cell
}

#[cfg(not(unix))]
pub fn internal_format_filename(
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
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
    #[cfg(windows)]
    if file_type.is_file() {
        let extension = get_file_extension(filename_cell.contents());
        if ["exe", "bat", "cmd"].contains(&&extension.as_str()) {
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::EXEC);
            }
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

    if file_type.is_dir() {
        let mut filename_cell = DisplayCell::from_str_with_style(file_name, ls_colors.dir_style());
        if indicator_style.dir() {
            filename_cell.push_char(IndicatorStyle::DIR);
        }

        return filename_cell;
    } else if file_type.is_symlink() {
        let mut filename_cell =
            DisplayCell::from_str_with_style(file_name, ls_colors.symlink_style());
        if indicator_style.others() && !config.output_format.is_long() {
            filename_cell.push_char(IndicatorStyle::SYMLINK);
        }

        return filename_cell;
    } else if file_type.is_file() {
        let extension = get_file_extension(file_name);
        if extension.is_empty() {
            return DisplayCell::from_str_with_style(file_name, ls_colors.file_style());
        } else {
            #[cfg(windows)]
            if ["exe", "bat", "cmd"].contains(&&extension.as_str()) {
                let mut filename_cell =
                    DisplayCell::from_str_with_style(file_name, ls_colors.exec_style());
                if indicator_style.others() {
                    filename_cell.push_char(IndicatorStyle::EXEC);
                }
                return filename_cell;
            }

            return DisplayCell::from_str_with_style(
                file_name,
                ls_colors.extension_style(extension),
            );
        }
    } else {
        return DisplayCell::from(file_name.to_string());
    }
}
