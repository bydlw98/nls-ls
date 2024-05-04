use nls_term_grid::*;

use crate::config::Config;
use crate::entry::EntryBuf;
use crate::output::{GridCell, GridCellExts};

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
        let mut cells_vec: Vec<GridCell> = Vec::with_capacity(entrybuf_vec.len() * num_columns);

        for entrybuf in entrybuf_vec {
            if config.list_inode {
                cells_vec.push(entrybuf.ino_cell(config));
            }
            if config.list_allocated_size {
                cells_vec.push(entrybuf.allocated_size_cell(config));
            }
            cells_vec.push(entrybuf.file_name_cell(config));
        }

        let grid = Grid::new(" ", Direction::LeftToRight, &cells_vec);
        print!("{}", grid.fit_into_columns(num_columns));
    }
}

fn multi_column_format(direction: Direction, entrybuf_vec: &[EntryBuf], config: &Config) {
    use crate::utils::terminal_width;

    let mut cells_vec: Vec<GridCell> = Vec::with_capacity(entrybuf_vec.len());

    if config.list_inode || config.list_allocated_size {
        complex_multi_column_grid_init(&mut cells_vec, entrybuf_vec, config);
    } else {
        for entrybuf in entrybuf_vec {
            cells_vec.push(entrybuf.file_name_cell(config));
        }
    }

    let display_width = terminal_width().unwrap_or(80);
    let grid = Grid::new("  ", direction, &cells_vec);

    match grid.fit_into_width(display_width) {
        Some(display) => print!("{}", display),
        None => single_column_format(entrybuf_vec, config),
    }
}

fn complex_multi_column_grid_init(
    cells_vec: &mut Vec<GridCell>,
    entrybuf_vec: &[EntryBuf],
    config: &Config,
) {
    let entrybuf_count = entrybuf_vec.len();
    let mut ino_cell_vec: Vec<GridCell> = Vec::with_capacity(entrybuf_count);
    let mut max_ino_cell_width: usize = 0;
    let mut allocated_size_cell_vec: Vec<GridCell> = Vec::with_capacity(entrybuf_count);
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
        let mut cell = GridCell::with_capacity(128);
        if config.list_inode {
            gridcell_append_contents_with_width_right_padded(
                &mut cell,
                &ino_cell_vec[i],
                max_ino_cell_width,
            );
            cell.push_char(' ');
        }
        if config.list_allocated_size {
            gridcell_append_contents_with_width_right_padded(
                &mut cell,
                &allocated_size_cell_vec[i],
                max_allocated_size_cell_width,
            );
            cell.push_char(' ');
        }
        cell.append(entrybuf_vec[i].file_name_cell(config));
        cells_vec.push(cell);
    }
}

fn gridcell_append_contents_with_width_right_padded(
    cell: &mut GridCell,
    other_cell: &GridCell,
    width: usize,
) {
    use std::fmt::Write;

    let pad_width: usize = if width <= other_cell.width {
        0
    } else {
        width - other_cell.width
    };

    // Check if pad width is 0
    if pad_width == 0 {
        // if pad width is 0, we do not need to do padding
        cell.contents.push_str(&other_cell.contents);
        cell.width += width;
    } else {
        let _ = write!(
            cell.contents,
            "{}{}",
            " ".repeat(pad_width),
            other_cell.contents
        );
        cell.width += width;
    }
}
