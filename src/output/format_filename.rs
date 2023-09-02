use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use crate::config::{Config, IndicatorStyle};
use crate::os::unix::sys_prelude::*;
use crate::utils::HasMaskSetExt;

use super::DisplayCell;

#[cfg(unix)]
pub fn format_filename(file_name: &str, metadata: &Metadata, config: &Config) -> DisplayCell {
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

#[cfg(not(unix))]
pub fn format_filename(file_name: &str, metadata: &Metadata, config: &Config) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let file_type = metadata.file_type();
    let mut filename_cell = DisplayCell::from(String::from(file_name));

    if file_type.is_dir() {
        if indicator_style.dir() {
            filename_cell.push_char(IndicatorStyle::DIR);
        }
    } else if file_type.is_symlink() {
        let mut filename_cell = DisplayCell::from(String::from(file_name));
        if indicator_style.others() {
            filename_cell.push_char(IndicatorStyle::SYMLINK);
        }
    }

    filename_cell
}
