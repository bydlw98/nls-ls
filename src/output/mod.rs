mod column;
mod display_cell;
mod format_filename;
mod format_size;
mod format_timestamp;
mod grid;
mod long;
mod sort;

pub use display_cell::{Alignment, DisplayCell};
pub use format_filename::format_filename;
pub use format_size::format_size;
pub use format_timestamp::format_timestamp;

use crate::config::{Config, OutputFormat};
use crate::entry::EntryBuf;

pub fn output(entrybuf_vec: &mut Vec<EntryBuf>, config: &Config) {
    use column::{across_format, single_column_format, vertical_format};
    use long::long_format;
    use sort::sort_entrybuf_vec;

    if entrybuf_vec.is_empty() {
        return;
    }

    sort_entrybuf_vec(entrybuf_vec, config);

    match config.output_format {
        OutputFormat::SingleColumn => single_column_format(entrybuf_vec, config),
        OutputFormat::Vertical => vertical_format(entrybuf_vec, config),
        OutputFormat::Across => across_format(entrybuf_vec, config),
        OutputFormat::Long => long_format(entrybuf_vec, config),
    }
}

pub fn print_total(entrybuf_vec: &[EntryBuf], config: &Config) {
    let total: u64 = entrybuf_vec
        .iter()
        .map(|entrybuf| entrybuf.allocated_size().unwrap_or(0))
        .sum();

    println!("total {}", format_size(total, config).contents);
}
