use super::grid::*;

use crate::config::Config;
use crate::entry::EntryBuf;

pub fn vertical_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    multi_column_format(Direction::TopToBottom, entrybuf_vec, config)
}

pub fn across_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    multi_column_format(Direction::LeftToRight, entrybuf_vec, config)
}

pub fn single_column_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    let num_columns: usize =
        1 + (config.list_inode as usize) + (config.list_allocated_size as usize);

    if num_columns == 1 {
        for entrybuf in entrybuf_vec {
            let file_name_cell = entrybuf.file_name_cell(config);
            println!("{}", file_name_cell.contents);
        }
    } else {
        let mut grid = Grid::new(entrybuf_vec.len(), 1, Direction::LeftToRight);

        for entrybuf in entrybuf_vec {
            if config.list_inode {
                grid.add(entrybuf.ino_cell(config));
            }
            if config.list_allocated_size {
                grid.add(entrybuf.allocated_size_cell(config));
            }
            grid.add(entrybuf.file_name_cell(config));
        }

        print!("{}", grid.fit_into_columns(num_columns));
    }
}

fn multi_column_format(direction: Direction, entrybuf_vec: &[EntryBuf], config: &Config) {
    use crate::utils::terminal_width;

    let mut grid = Grid::new(entrybuf_vec.len(), 2, direction);

    if config.list_inode || config.list_allocated_size {
        complex_multi_column_grid_init(&mut grid, entrybuf_vec, config);
    } else {
        for entrybuf in entrybuf_vec {
            grid.add(entrybuf.file_name_cell(config));
        }
    }

    let display_width = terminal_width().unwrap_or(80);
    match grid.fit_into_width(display_width) {
        Some(display) => print!("{}", display),
        None => single_column_format(entrybuf_vec, config),
    }
}

fn complex_multi_column_grid_init(grid: &mut Grid, entrybuf_vec: &[EntryBuf], config: &Config) {
    use super::DisplayCell;

    let entrybuf_count = entrybuf_vec.len();
    let mut ino_cell_vec: Vec<DisplayCell> = Vec::with_capacity(entrybuf_count);
    let mut max_ino_cell_width: usize = 0;
    let mut allocated_size_cell_vec: Vec<DisplayCell> = Vec::with_capacity(entrybuf_count);
    let mut max_allocated_size_cell_width: usize = 0;

    for entrybuf in entrybuf_vec {
        if config.list_inode {
            let ino_cell = entrybuf.ino_cell(config);
            max_ino_cell_width = max_ino_cell_width.max(ino_cell.width);
            ino_cell_vec.push(ino_cell);
        }
        if config.list_allocated_size {
            let allocated_size_cell = entrybuf.allocated_size_cell(config);
            max_allocated_size_cell_width =
                max_allocated_size_cell_width.max(allocated_size_cell.width);
            allocated_size_cell_vec.push(allocated_size_cell);
        }
    }

    for i in 0..entrybuf_count {
        let mut cell = DisplayCell::with_capacity(128);
        if config.list_inode {
            let ino_cell = &ino_cell_vec[i];
            let _ = ino_cell.write(&mut cell.contents, max_ino_cell_width);
            cell.width += max_ino_cell_width;
            cell.push_char(' ');
        }
        if config.list_allocated_size {
            let allocated_size_cell = &allocated_size_cell_vec[i];
            let _ = allocated_size_cell.write(&mut cell.contents, max_allocated_size_cell_width);
            cell.width += max_allocated_size_cell_width;
            cell.push_char(' ');
        }
        cell.append(entrybuf_vec[i].file_name_cell(config));
        grid.add(cell);
    }
}
