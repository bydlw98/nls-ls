mod accounts;
mod mode;
pub mod sys_prelude;

pub use accounts::{get_groupname_cell_by_gid, get_username_cell_by_uid};
pub use mode::{pwsh_mode_cell, rwx_mode_cell};

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

use crate::config::{AllocatedSizeBlocks, Config};

pub fn get_allocated_size(metadata: &Metadata, config: &Config) -> u64 {
    match config.allocated_size_blocks {
        AllocatedSizeBlocks::Raw => metadata.blocks() * 512,
        AllocatedSizeBlocks::Posix => metadata.blocks(),
        AllocatedSizeBlocks::Kibibytes => {
            ((metadata.blocks() as f64 * 512.0) / 1024.0).ceil() as u64
        }
    }
}
