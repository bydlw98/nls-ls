use crate::config::Config;
use crate::entry::EntryBuf;

pub fn long_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    use nls_term_grid::*;

    let num_columns: usize = 5
        + (config.list_inode as usize)
        + (config.list_allocated_size as usize)
        + (config.list_owner as usize)
        + (config.list_group as usize);

    let mut cells_vec: Vec<GridCell> = Vec::with_capacity(entrybuf_vec.len() * num_columns);
    for entrybuf in entrybuf_vec {
        if config.list_inode {
            cells_vec.push(entrybuf.ino_cell(config));
        }
        if config.list_allocated_size {
            cells_vec.push(entrybuf.allocated_size_cell(config));
        }
        cells_vec.push(entrybuf.mode_cell(config));
        cells_vec.push(entrybuf.nlink_cell(config));
        if config.list_owner {
            cells_vec.push(entrybuf.owner_cell(config));
        }
        if config.list_group {
            cells_vec.push(entrybuf.group_cell(config));
        }
        cells_vec.push(entrybuf.size_cell(config));
        cells_vec.push(entrybuf.timestamp_cell(config));
        cells_vec.push(entrybuf.file_name_cell(config));
    }

    let grid = Grid::new(" ", Direction::LeftToRight, &cells_vec);
    print!("{}", grid.fit_into_columns(num_columns));
}
