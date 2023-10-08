use std::time::SystemTime;

use chrono::offset::{Local, TimeZone};
use chrono::{Datelike, LocalResult, Timelike};
use once_cell::sync::Lazy;

use crate::config::Config;
use crate::output::DisplayCell;
use crate::utils::systemtime_to_unix_timestamp;

pub fn format_timestamp(timestamp: i64, config: &Config) -> DisplayCell {
    match Local.timestamp_opt(timestamp, 0) {
        LocalResult::Single(datetime) => {
            static SIX_MONTHS_AGO_TIMESTAMP: Lazy<i64> = Lazy::new(get_six_months_ago_timestamp);
            let timestamp_style = config.theme.time_style();

            let timestamp_string = if timestamp > *SIX_MONTHS_AGO_TIMESTAMP {
                format!(
                    "{} {:>2} {:0>2}:{:0>2}",
                    MONTH_TABLE[datetime.month0() as usize],
                    datetime.day(),
                    datetime.hour(),
                    datetime.minute()
                )
            } else {
                format!(
                    "{} {:>2}  {}",
                    MONTH_TABLE[datetime.month0() as usize],
                    datetime.day(),
                    datetime.year()
                )
            };

            DisplayCell::from_ascii_str_with_style(&timestamp_string, timestamp_style)
        }
        _ => DisplayCell::error_cell(true),
    }
}

const MONTH_TABLE: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn get_six_months_ago_timestamp() -> i64 {
    const SIX_MONTHS_IN_SECS: i64 = 60 * 60 * 24 * 30 * 6;

    systemtime_to_unix_timestamp(Ok(SystemTime::now())).unwrap() - SIX_MONTHS_IN_SECS
}
