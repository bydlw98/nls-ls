mod accounts;
mod mode;
pub mod sys_prelude;

pub use accounts::{get_groupname_cell_by_gid, get_username_cell_by_uid};
pub use mode::{pwsh_mode_cell, rwx_mode_cell};

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

use sys_prelude::*;

use crate::config::{AllocatedSizeBlocks, Config};
use crate::output::DisplayCell;

pub fn get_allocated_size(metadata: &Metadata, config: &Config) -> u64 {
    match config.allocated_size_blocks {
        AllocatedSizeBlocks::Raw => metadata.blocks() * 512,
        AllocatedSizeBlocks::Posix => metadata.blocks(),
        AllocatedSizeBlocks::Kibibytes => {
            ((metadata.blocks() as f64 * 512.0) / 1024.0).ceil() as u64
        }
    }
}

pub fn format_rdev(rdev: u64, config: &Config) -> DisplayCell {
    let major = unsafe { c::major(rdev) };
    let minor = unsafe { c::minor(rdev) };
    let major_minor_string = format!("{},{:>4}", major, minor);
    let rdev_style = config.theme.size_style();

    DisplayCell::from_ascii_str_with_style(&major_minor_string, rdev_style).left_aligned(false)
}
