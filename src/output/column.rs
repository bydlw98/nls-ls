use term_grid::{Direction, Filling, Grid, GridOptions};

use super::long::LongFormatGrid;
use super::DisplayCell;
use crate::config::Config;
use crate::entry::EntryBuf;

pub fn vertical_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    internal_multi_column_format(Direction::TopToBottom, entrybuf_vec, config);
}

pub fn across_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    internal_multi_column_format(Direction::LeftToRight, entrybuf_vec, config);
}

pub fn single_column_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    if config.list_inode {
        let mut grid = LongFormatGrid::new(2, entrybuf_vec.len());
        for entrybuf in entrybuf_vec {
            grid.add(entrybuf.ino_cell());
            grid.add(entrybuf.file_name_cell(config));
        }
        print!("{}", grid);
    } else {
        for entrybuf in entrybuf_vec {
            println!("{}", entrybuf.file_name_cell(config));
        }
    }
}

fn internal_multi_column_format(direction: Direction, entrybuf_vec: &[EntryBuf], config: &Config) {
    let mut grid = Grid::new(GridOptions {
        direction: direction,
        filling: Filling::Spaces(2),
    });
    grid.reserve(entrybuf_vec.len() + 1);

    if config.list_inode {
        complex_multi_column_format(&mut grid, entrybuf_vec, config);
    } else {
        for entrybuf in entrybuf_vec {
            grid.add(entrybuf.file_name_cell(config).into());
        }
    }

    match grid.fit_into_width(config.width) {
        Some(display) => print!("{}", display),
        None => print!("{}", grid.fit_into_columns(1)),
    }
}

fn complex_multi_column_format(grid: &mut Grid, entrybuf_vec: &[EntryBuf], config: &Config) {
    let entrybuf_count = entrybuf_vec.len();
    let mut ino_cell_vec = Vec::with_capacity(entrybuf_count);
    let mut max_ino_cell_width: usize = 0;

    for entrybuf in entrybuf_vec {
        let ino_cell = entrybuf.ino_cell();
        max_ino_cell_width = max_ino_cell_width.max(ino_cell.width());
        ino_cell_vec.push(ino_cell);
    }

    for i in 0..entrybuf_count {
        ino_cell_vec[i].pad_to_width(max_ino_cell_width);
        let mut cell = DisplayCell::with_capacity(64);

        cell.push_str_with_width(&ino_cell_vec[i].to_string(), max_ino_cell_width);
        cell.push_char(' ');
        cell.append(entrybuf_vec[i].file_name_cell(config));
        grid.add(cell.into());
    }
}
