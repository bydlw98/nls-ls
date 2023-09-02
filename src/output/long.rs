use super::DisplayCell;

use std::fmt;

use crate::config::Config;
use crate::entry::EntryBuf;

pub fn long_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    let num_columns: usize = 7;

    let mut grid = LongFormatGrid::new(num_columns, entrybuf_vec.len());

    for entrybuf in entrybuf_vec {
        grid.add(entrybuf.mode_cell());
        grid.add(entrybuf.nlink_cell());
        grid.add(entrybuf.owner_cell());
        grid.add(entrybuf.group_cell());
        grid.add(entrybuf.size_cell(config));
        grid.add(entrybuf.timestamp_cell());
        grid.add(entrybuf.file_name_cell(config));
    }

    print!("{}", grid);
}

#[derive(Debug)]
struct LongFormatGrid {
    display_cells_vec: Vec<DisplayCell>,
    column_widths: Vec<usize>,
    num_columns: usize,
    column_index: usize,
    num_entries: usize,
}

impl LongFormatGrid {
    pub fn new(num_columns: usize, num_entries: usize) -> Self {
        Self {
            display_cells_vec: Vec::with_capacity(num_entries),
            column_widths: vec![0; num_columns],
            num_columns: num_columns,
            column_index: 0,
            num_entries: num_entries,
        }
    }

    pub fn add(&mut self, display_cell: DisplayCell) {
        self.column_widths[self.column_index] =
            self.column_widths[self.column_index].max(display_cell.width());
        self.display_cells_vec.push(display_cell);

        if self.column_index < (self.num_columns - 1) {
            self.column_index += 1;
        } else {
            self.column_index = 0;
        }
    }
}

impl fmt::Display for LongFormatGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cell_index: usize = 0;

        for _ in 0..self.num_entries {
            for i in 0..(self.num_columns - 1) {
                let mut cell = self.display_cells_vec[cell_index].clone();
                cell.pad_to_width(self.column_widths[i]);

                write!(f, "{} ", cell)?;
                cell_index += 1;
            }
            writeln!(f, "{}", self.display_cells_vec[cell_index])?;
            cell_index += 1;
        }
        Ok(())
    }
}
