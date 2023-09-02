use std::cmp::Ordering;

use crate::config::{Config, SortingOrder};
use crate::entry::EntryBuf;

pub fn sort_entrybuf_vec(entrybuf_vec: &mut Vec<EntryBuf>, config: &Config) {
    if entrybuf_vec.len() < 2 {
        return;
    }

    match config.sorting_order {
        SortingOrder::FileName => {
            entrybuf_vec.sort_by(file_name_compare);
            log::debug!("Sorted by file name");
        }
        SortingOrder::Size => {
            entrybuf_vec.sort_by(size_compare);
            log::debug!("Sorted by size");
        }
        SortingOrder::Timestamp => {
            entrybuf_vec.sort_by(timestamp_compare);
            log::debug!("Sorted by time");
        }
    }

    if config.reverse {
        entrybuf_vec.reverse();
    }
}

fn file_name_compare(entrybuf_1: &EntryBuf, entrybuf_2: &EntryBuf) -> Ordering {
    entrybuf_1.file_name_key().cmp(entrybuf_2.file_name_key())
}

fn size_compare(entrybuf_1: &EntryBuf, entrybuf_2: &EntryBuf) -> Ordering {
    entrybuf_2
        .size()
        .cmp(&entrybuf_1.size())
        .then(file_name_compare(entrybuf_1, entrybuf_2))
}

fn timestamp_compare(entrybuf_1: &EntryBuf, entrybuf_2: &EntryBuf) -> Ordering {
    entrybuf_2
        .timestamp()
        .cmp(&entrybuf_1.timestamp())
        .then(file_name_compare(entrybuf_1, entrybuf_2))
}
