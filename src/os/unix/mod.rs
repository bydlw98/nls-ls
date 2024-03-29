mod accounts;
mod mode;
pub mod sys_prelude;

pub use accounts::{get_groupname_cell_by_gid, get_username_cell_by_uid};
pub use mode::{pwsh_mode_cell, rwx_mode_cell};

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

use nls_term_grid::{Alignment, GridCell};

use sys_prelude::*;

use crate::config::{AllocatedSizeBlocks, Config};
use crate::output::GridCellExts;

pub fn get_allocated_size(metadata: &Metadata, config: &Config) -> u64 {
    match config.allocated_size_blocks {
        AllocatedSizeBlocks::Raw => metadata.blocks() * 512,
        AllocatedSizeBlocks::Posix => metadata.blocks(),
        AllocatedSizeBlocks::Kibibytes => {
            ((metadata.blocks() as f64 * 512.0) / 1024.0).ceil() as u64
        }
    }
}

pub fn format_rdev(rdev: u64, config: &Config) -> GridCell {
    let raw_rdev = rdev as libc::dev_t;
    let major = unsafe { c::major(raw_rdev) };
    let minor = unsafe { c::minor(raw_rdev) };
    let major_minor_string = format!("{},{:>4}", major, minor);
    let rdev_style = config.theme.size_style();

    let mut rdev_cell = GridCell::from_ascii_str_with_style(&major_minor_string, rdev_style);
    rdev_cell.alignment = Alignment::Right;

    rdev_cell
}
