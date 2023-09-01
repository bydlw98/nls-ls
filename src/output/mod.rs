mod column;

use column::{across_format, single_column_format, vertical_format};

use std::cmp::Ordering;

use crate::config::{Config, OutputFormat};
use crate::entry::EntryBuf;

pub fn output(entrybuf_vec: &mut Vec<EntryBuf>, config: &Config) {
    if entrybuf_vec.is_empty() {
        return;
    }

    entrybuf_vec.sort_by(file_name_compare);

    match config.output_format {
        OutputFormat::Across => across_format(entrybuf_vec, config),
        OutputFormat::SingleColumn => single_column_format(entrybuf_vec, config),
        OutputFormat::Vertical => vertical_format(entrybuf_vec, config),

    }
}

fn file_name_compare(entrybuf_1: &EntryBuf, entrybuf_2: &EntryBuf) -> Ordering {
    entrybuf_1.file_name_key().cmp(entrybuf_2.file_name_key())
}
