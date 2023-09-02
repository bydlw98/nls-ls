mod column;
mod display_cell;
mod long;
mod format_filename;
mod format_size;
mod format_timestamp;
mod sort;

pub use display_cell::DisplayCell;
pub use format_filename::format_filename;
pub use format_size::format_size;
pub use format_timestamp::format_timestamp;

use column::{across_format, single_column_format, vertical_format};
use long::long_format;
use sort::sort_entrybuf_vec;

use crate::config::{Config, OutputFormat};
use crate::entry::EntryBuf;

pub fn output(entrybuf_vec: &mut Vec<EntryBuf>, config: &Config) {
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
