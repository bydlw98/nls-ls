//! Grid API and Implementation is inspired by [ogham/term_grid](https://crates.io/crates/term-grid)

use super::{Alignment, DisplayCell};

use std::fmt;

#[derive(Debug, Default)]
pub struct Grid {
    cells_vec: Vec<DisplayCell>,
    num_spaces: usize,
    direction: Direction,
}

impl Grid {
    pub fn new(initial_capacity: usize, num_spaces: usize, direction: Direction) -> Self {
        Self {
            cells_vec: Vec::with_capacity(initial_capacity),
            num_spaces: num_spaces,
            direction: direction,
        }
    }

    pub fn total_cell_count(&self) -> usize {
        self.cells_vec.len()
    }

    pub fn add(&mut self, cell: DisplayCell) {
        self.cells_vec.push(cell);
    }

    pub fn fit_into_columns(&self, num_columns: usize) -> Display<'_> {
        let dimentions = self.calculate_dimentions(num_columns);

        Display {
            dimentions: dimentions,
            grid: self,
        }
    }

    pub fn fit_into_width(&self, display_width: usize) -> Option<Display<'_>> {
        if self.cells_vec.is_empty() {
            return Some(Display {
                dimentions: Dimentions::one_row(0),
                grid: self,
            });
        }
        let max_cell_width: usize = self
            .cells_vec
            .iter()
            .map(|cell| cell.width)
            .max()
            .unwrap_or(0);

        // return `None` if there is a `DisplayCell` whose width is
        // greator than or equal than display_width
        if max_cell_width >= display_width {
            None
        } else {
            let total_width: usize = (self.cells_vec.iter().map(|cell| cell.width).sum::<usize>())
                + (self.total_cell_count() - 1) * self.num_spaces;

            // if total width width is <= display_width, display all `DisplayCell` in one row
            if total_width <= display_width {
                Some(Display {
                    dimentions: Dimentions::one_row(self.total_cell_count()),
                    grid: self,
                })
            } else {
                Some(self.internal_fit_into_width(max_cell_width, display_width))
            }
        }
    }

    fn internal_fit_into_width(&self, max_cell_width: usize, display_width: usize) -> Display<'_> {
        let total_cell_count = self.total_cell_count();
        // choose the starting num_columns by using the max DisplayCell width
        // with seperator spaces
        let mut num_columns = display_width / (max_cell_width + self.num_spaces);
        let mut dimentions = self.calculate_dimentions(num_columns);

        // increase the num_columns to find the dimentions where grid is most well packed
        loop {
            num_columns += 1;
            let new_dimentions = self.calculate_dimentions(num_columns);

            // stop increasing num_columns if total width is greator than display_width
            if new_dimentions.total_width(self.num_spaces) > display_width {
                break;
            }
            // use new_dimentions as dimentions if it is well packed
            else if new_dimentions.is_well_packed(total_cell_count, dimentions.num_rows) {
                dimentions = new_dimentions;
            }
        }

        Display {
            dimentions: dimentions,
            grid: self,
        }
    }

    fn calculate_dimentions(&self, num_columns: usize) -> Dimentions {
        let num_rows = usize_div_ceil(self.total_cell_count(), num_columns);
        let mut column_widths: Vec<usize> = vec![0; num_columns];

        for (cell_index, cell) in self.cells_vec.iter().enumerate() {
            let column_index = match self.direction {
                Direction::LeftToRight => cell_index % num_columns,
                Direction::TopToBottom => cell_index / num_rows,
            };

            column_widths[column_index] = column_widths[column_index].max(cell.width);
        }

        Dimentions {
            num_rows: num_rows,
            column_widths: column_widths,
        }
    }
}

#[derive(Debug)]
pub struct Display<'a> {
    dimentions: Dimentions,
    grid: &'a Grid,
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_cell_count = self.grid.total_cell_count();
        if total_cell_count == 0 {
            return writeln!(f);
        }
        let mut cell_count: usize = 0;
        let last_cell_index = total_cell_count - 1;
        let num_columns = self.dimentions.column_widths.len();
        let last_column_index = num_columns - 1;

        for row_index in 0..self.dimentions.num_rows {
            for column_index in 0..num_columns {
                let cell_index = match self.grid.direction {
                    Direction::LeftToRight => row_index * num_columns + column_index,
                    Direction::TopToBottom => row_index + self.dimentions.num_rows * column_index,
                };

                // if the cell_index is greator than last_cell_index,
                // continue to next loop iteration
                if cell_index > last_cell_index {
                    continue;
                }

                cell_count += 1;
                let cell = &self.grid.cells_vec[cell_index];

                // if (the current column is the last column or is the last cell)
                // and the cell is left aligned, the cell does not need to be
                // written with padding and does not need be written with seperator spaces
                if ((column_index == last_column_index) || (cell_count == total_cell_count))
                    && cell.alignment == Alignment::Left
                {
                    write!(f, "{}", cell.contents)?;
                } else {
                    cell.write(f, self.dimentions.column_widths[column_index])?;
                    write!(f, "{}", " ".repeat(self.grid.num_spaces))?;
                }
            }
            // write a '\n' after the last column in row
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    TopToBottom,
}

impl Default for Direction {
    fn default() -> Self {
        Self::LeftToRight
    }
}

#[derive(Debug, Default)]
struct Dimentions {
    num_rows: usize,
    column_widths: Vec<usize>,
}

impl Dimentions {
    pub fn total_width(&self, spaces: usize) -> usize {
        self.column_widths.iter().sum::<usize>() + ((self.column_widths.len() - 1) * spaces)
    }

    /// for dimentions to be well packed, the following must occur:
    /// 1. the last column must have less than or equal to the number of rows
    /// 2. there should be as few columns as possible, this is done by checking if
    ///     the current number of rows chosen to be used is the same as the previous
    ///     well packed dimentions. If it is the same, the previous well packed dimentions
    ///     is more well packed due to it having fewer columns
    pub fn is_well_packed(&self, cell_count: usize, previous_num_rows: usize) -> bool {
        let last_col_cell_count = cell_count % (self.column_widths.len() - 1);

        (last_col_cell_count <= self.num_rows) && (self.num_rows != previous_num_rows)
    }

    pub fn one_row(cell_count: usize) -> Self {
        Self {
            num_rows: 1,
            column_widths: vec![0; cell_count],
        }
    }
}

/// div_ceil implementation is taken from Rust Core 1.73.0 stable
fn usize_div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 && rhs > 0 {
        d + 1
    } else {
        d
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_cells() {
        let grid = Grid::new(1, 2, Direction::LeftToRight);
        let display = grid.fit_into_width(80).unwrap();

        assert_eq!(display.to_string(), "\n");

        let grid = Grid::new(1, 2, Direction::TopToBottom);
        let display = grid.fit_into_width(80).unwrap();

        assert_eq!(display.to_string(), "\n");
    }

    #[test]
    fn test_one_cell() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        grid.add(DisplayCell::from(String::from("file")));
        let display = grid.fit_into_width(80).unwrap();

        assert_eq!(display.to_string(), "file\n");

        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        grid.add(DisplayCell::from(String::from("file")));
        let display = grid.fit_into_width(80).unwrap();

        assert_eq!(display.to_string(), "file\n");
    }

    #[test]
    fn test_fit_into_width_cell_longer_than_display_width() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file11")));
        grid.add(DisplayCell::from(String::from("file111")));

        assert!(grid.fit_into_width(6).is_none());

        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file111")));
        grid.add(DisplayCell::from(String::from("file11")));

        assert!(grid.fit_into_width(6).is_none());
    }

    #[test]
    fn test_fit_into_width_fit_into_one_line() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file5")));
        let display = grid.fit_into_width(35).unwrap();

        assert_eq!(display.to_string(), "file1  file2  file3  file4  file5\n");

        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file5")));
        let display = grid.fit_into_width(35).unwrap();

        assert_eq!(display.to_string(), "file1  file2  file3  file4  file5\n");
    }

    #[test]
    fn test_fit_into_width_fit_into_one_line_color() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });
        let display = grid.fit_into_width(35).unwrap();

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file1  file2  file3  file4  file5\n"
        assert_eq!(display.to_string(), "\x1b[31mfile1\x1b[0m  \x1b[32mfile2\x1b[0m  \x1b[33mfile3\x1b[0m  \x1b[34mfile4\x1b[0m  \x1b[35mfile5\x1b[0m\n");

        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });
        let display = grid.fit_into_width(35).unwrap();

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file1  file2  file3  file4  file5\n"
        assert_eq!(display.to_string(), "\x1b[31mfile1\x1b[0m  \x1b[32mfile2\x1b[0m  \x1b[33mfile3\x1b[0m  \x1b[34mfile4\x1b[0m  \x1b[35mfile5\x1b[0m\n");
    }

    #[test]
    fn test_fit_into_width_more_than_one_line_lefttoright() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        grid.add(DisplayCell::from(String::from("file10")));
        grid.add(DisplayCell::from(String::from("file20")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file400")));
        grid.add(DisplayCell::from(String::from("file5")));
        grid.add(DisplayCell::from(String::from("file100")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file30")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file500")));
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file200")));
        grid.add(DisplayCell::from(String::from("file300")));
        grid.add(DisplayCell::from(String::from("file40")));
        grid.add(DisplayCell::from(String::from("file50")));
        let display = grid.fit_into_width(35).unwrap();

        assert_eq!(
            display.to_string(),
            "file10   file20   file3   file400\n\
             file5    file100  file2   file30\n\
             file4    file500  file1   file200\n\
             file300  file40   file50\n"
        );
    }

    #[test]
    fn test_fit_into_width_more_than_one_line_lefttoright_color() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left});

        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left});

        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left});
        let display = grid.fit_into_width(35).unwrap();

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file10   file20   file3   file400\n\
        //  file5    file100  file2   file30\n\
        //  file4    file500  file1   file200\n\
        //  file300  file40   file50\n"
        assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m   \x1b[32mfile20\x1b[0m   \x1b[33mfile3\x1b[0m   \x1b[34mfile400\x1b[0m\n\
             \x1b[35mfile5\x1b[0m    \x1b[31mfile100\x1b[0m  \x1b[32mfile2\x1b[0m   \x1b[33mfile30\x1b[0m\n\
             \x1b[34mfile4\x1b[0m    \x1b[35mfile500\x1b[0m  \x1b[31mfile1\x1b[0m   \x1b[32mfile200\x1b[0m\n\
             \x1b[33mfile300\x1b[0m  \x1b[34mfile40\x1b[0m   \x1b[35mfile50\x1b[0m\n"
        );
    }

    #[test]
    fn test_fit_into_width_more_than_one_line_toptobottom() {
        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        grid.add(DisplayCell::from(String::from("file10")));
        grid.add(DisplayCell::from(String::from("file20")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file400")));
        grid.add(DisplayCell::from(String::from("file5")));
        grid.add(DisplayCell::from(String::from("file100")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file30")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file500")));
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file200")));
        grid.add(DisplayCell::from(String::from("file300")));
        grid.add(DisplayCell::from(String::from("file40")));
        grid.add(DisplayCell::from(String::from("file50")));
        let display = grid.fit_into_width(35).unwrap();

        assert_eq!(
            display.to_string(),
            "file10   file5    file4    file300\n\
             file20   file100  file500  file40\n\
             file3    file2    file1    file50\n\
             file400  file30   file200\n"
        );
    }

    #[test]
    fn test_fit_into_width_more_than_one_line_toptobottom_color() {
        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left});

        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left});

        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left});
        #[rustfmt::skip]
        grid.add(DisplayCell {contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left});
        let display = grid.fit_into_width(35).unwrap();

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file10   file5    file4    file300\n\
        //  file20   file100  file500  file40\n\
        //  file3    file2    file1    file50\n\
        //  file400  file30   file200\n"
        assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m   \x1b[35mfile5\x1b[0m    \x1b[34mfile4\x1b[0m    \x1b[33mfile300\x1b[0m\n\
             \x1b[32mfile20\x1b[0m   \x1b[31mfile100\x1b[0m  \x1b[35mfile500\x1b[0m  \x1b[34mfile40\x1b[0m\n\
             \x1b[33mfile3\x1b[0m    \x1b[32mfile2\x1b[0m    \x1b[31mfile1\x1b[0m    \x1b[35mfile50\x1b[0m\n\
             \x1b[34mfile400\x1b[0m  \x1b[33mfile30\x1b[0m   \x1b[32mfile200\x1b[0m\n"
        );
    }

    #[test]
    fn test_fit_into_columns_lefttoright_same_alignment() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        grid.add(DisplayCell::from(String::from("file10")));
        grid.add(DisplayCell::from(String::from("file20")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file400")));
        grid.add(DisplayCell::from(String::from("file5")));
        grid.add(DisplayCell::from(String::from("file100")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file30")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file500")));
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file200")));
        grid.add(DisplayCell::from(String::from("file300")));
        grid.add(DisplayCell::from(String::from("file40")));
        grid.add(DisplayCell::from(String::from("file50")));
        let display = grid.fit_into_columns(5);

        assert_eq!(
            display.to_string(),
            "file10   file20   file3    file400  file5\n\
             file100  file2    file30   file4    file500\n\
             file1    file200  file300  file40   file50\n"
        );
    }

    #[test]
    fn test_fit_into_columns_toptobottom_same_alignment() {
        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        grid.add(DisplayCell::from(String::from("file10")));
        grid.add(DisplayCell::from(String::from("file20")));
        grid.add(DisplayCell::from(String::from("file3")));
        grid.add(DisplayCell::from(String::from("file400")));
        grid.add(DisplayCell::from(String::from("file5")));
        grid.add(DisplayCell::from(String::from("file100")));
        grid.add(DisplayCell::from(String::from("file2")));
        grid.add(DisplayCell::from(String::from("file30")));
        grid.add(DisplayCell::from(String::from("file4")));
        grid.add(DisplayCell::from(String::from("file500")));
        grid.add(DisplayCell::from(String::from("file1")));
        grid.add(DisplayCell::from(String::from("file200")));
        grid.add(DisplayCell::from(String::from("file300")));
        grid.add(DisplayCell::from(String::from("file40")));
        grid.add(DisplayCell::from(String::from("file50")));
        let display = grid.fit_into_columns(5);

        assert_eq!(
            display.to_string(),
            "file10  file400  file2   file500  file300\n\
             file20  file5    file30  file1    file40\n\
             file3   file100  file4   file200  file50\n"
        );
    }

    #[test]
    fn test_fit_into_columns_lefttoright_different_alignments() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file10"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file20"), width: 6, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file3"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file400"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file5"), width: 5, alignment: Alignment::Left });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file100"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file2"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file30"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file4"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file500"), width: 7, alignment: Alignment::Left });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file1"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file200"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file300"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file40"), width: 6, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file50"), width: 6, alignment: Alignment::Left });

        let display = grid.fit_into_columns(5);

        assert_eq!(
            display.to_string(),
            "file10    file20  file3    file400  file5\n\
             file100    file2  file30     file4  file500\n\
             file1    file200  file300   file40  file50\n"
        );
    }

    #[test]
    fn test_fit_into_columns_lefttoright_different_alignments_color() {
        let mut grid = Grid::new(1, 2, Direction::LeftToRight);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left });

        let display = grid.fit_into_columns(5);

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file10    file20  file3    file400  file5\n\
        //  file100    file2  file30     file4  file500\n\
        //  file1    file200  file300   file40  file50\n"
        assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m    \x1b[32mfile20\x1b[0m  \x1b[33mfile3\x1b[0m    \x1b[34mfile400\x1b[0m  \x1b[35mfile5\x1b[0m\n\
             \x1b[31mfile100\x1b[0m    \x1b[32mfile2\x1b[0m  \x1b[33mfile30\x1b[0m     \x1b[34mfile4\x1b[0m  \x1b[35mfile500\x1b[0m\n\
             \x1b[31mfile1\x1b[0m    \x1b[32mfile200\x1b[0m  \x1b[33mfile300\x1b[0m   \x1b[34mfile40\x1b[0m  \x1b[35mfile50\x1b[0m\n"
        );
    }

    #[test]
    fn test_fit_into_columns_toptobottom_different_alignments() {
        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file10"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file20"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file3"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file400"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file5"), width: 5, alignment: Alignment::Right });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file100"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file2"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file30"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file4"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file500"), width: 7, alignment: Alignment::Right });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file1"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file200"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file300"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file40"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("file50"), width: 6, alignment: Alignment::Left });

        let display = grid.fit_into_columns(5);

        assert_eq!(
            display.to_string(),
            "file10  file400  file2   file500  file300\n\
             file20    file5  file30    file1  file40\n\
             file3   file100  file4   file200  file50\n"
        );
    }

    #[test]
    fn test_fit_into_columns_toptobottom_different_alignments_color() {
        let mut grid = Grid::new(1, 2, Direction::TopToBottom);
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Right });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Right });

        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Right });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left });
        #[rustfmt::skip]
        grid.add(DisplayCell { contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left });

        let display = grid.fit_into_columns(5);

        // if evaluated in a output device which renders ansi escape sequences
        // the following will be rendered with each cell having a color:
        // "file10  file400  file2   file500  file300\n\
        //  file20    file5  file30    file1  file40\n\
        //  file3   file100  file4   file200  file50\n"
        assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m  \x1b[34mfile400\x1b[0m  \x1b[32mfile2\x1b[0m   \x1b[35mfile500\x1b[0m  \x1b[33mfile300\x1b[0m\n\
             \x1b[32mfile20\x1b[0m    \x1b[35mfile5\x1b[0m  \x1b[33mfile30\x1b[0m    \x1b[31mfile1\x1b[0m  \x1b[34mfile40\x1b[0m\n\
             \x1b[33mfile3\x1b[0m   \x1b[31mfile100\x1b[0m  \x1b[34mfile4\x1b[0m   \x1b[32mfile200\x1b[0m  \x1b[35mfile50\x1b[0m\n"
        );
    }
}
