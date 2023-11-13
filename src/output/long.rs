use crate::config::Config;
use crate::entry::EntryBuf;

pub fn long_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    use super::grid::*;

    let num_columns: usize = 5
        + (config.list_inode as usize)
        + (config.list_allocated_size as usize)
        + (config.list_owner as usize)
        + (config.list_group as usize);

    let mut grid = Grid::new(entrybuf_vec.len() * num_columns, 1, Direction::LeftToRight);

    for entrybuf in entrybuf_vec {
        if config.list_inode {
            grid.add(entrybuf.ino_cell(config));
        }
        if config.list_allocated_size {
            grid.add(entrybuf.allocated_size_cell(config));
        }
        grid.add(entrybuf.mode_cell(config));
        grid.add(entrybuf.nlink_cell(config));
        if config.list_owner {
            grid.add(entrybuf.owner_cell(config));
        }
        if config.list_group {
            grid.add(entrybuf.group_cell(config));
        }
        grid.add(entrybuf.size_cell(config));
        grid.add(entrybuf.timestamp_cell(config));
        grid.add(entrybuf.file_name_cell(config));
    }

    print!("{}", grid.fit_into_columns(num_columns));
}
