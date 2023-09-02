use term_grid::{Direction, Filling, Grid, GridOptions};

use crate::config::Config;
use crate::entry::EntryBuf;

pub fn vertical_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    internal_multi_column_format(Direction::TopToBottom, entrybuf_vec, config);
}

pub fn across_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    internal_multi_column_format(Direction::LeftToRight, entrybuf_vec, config);
}

pub fn single_column_format(entrybuf_vec: &[EntryBuf], config: &Config) {
    for entrybuf in entrybuf_vec {
        println!("{}", entrybuf.file_name_cell(config));
    }
}

fn internal_multi_column_format(direction: Direction, entrybuf_vec: &[EntryBuf], config: &Config) {
    let mut grid = Grid::new(GridOptions {
        direction: direction,
        filling: Filling::Spaces(2),
    });
    grid.reserve(entrybuf_vec.len() + 1);

    for entrybuf in entrybuf_vec {
        grid.add(entrybuf.file_name_cell(config).into());
    }

    match grid.fit_into_width(config.width) {
        Some(display) => print!("{}", display),
        None => print!("{}", grid.fit_into_columns(1)),
    }
}
