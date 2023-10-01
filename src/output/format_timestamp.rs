use std::time::SystemTime;

use chrono::offset::{Local, TimeZone};
use chrono::{Datelike, Timelike};
use once_cell::sync::Lazy;

use crate::config::Config;
use crate::output::DisplayCell;
use crate::utils::get_unix_timestamp_from_systemtime;

pub fn format_timestamp(timestamp: i64, config: &Config) -> DisplayCell {
    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(datetime) => {
            let timestamp_style = config.theme.time_style();

            if timestamp > *SIX_MONTHS_AGO_UNIX_TIMESTAMP {
                DisplayCell::from_ascii_str_with_style(
                    &format!(
                        "{} {:>2} {:0>2}:{:0>2}",
                        MONTH_TABLE[datetime.month0() as usize],
                        datetime.day(),
                        datetime.hour(),
                        datetime.minute()
                    ),
                    timestamp_style,
                    true,
                )
            } else {
                DisplayCell::from_ascii_str_with_style(
                    &format!(
                        "{} {:>2}  {}",
                        MONTH_TABLE[datetime.month0() as usize],
                        datetime.day(),
                        datetime.year()
                    ),
                    timestamp_style,
                    true,
                )
            }
        }
        _ => DisplayCell::error_left_aligned(),
    }
}

static SIX_MONTHS_AGO_UNIX_TIMESTAMP: Lazy<i64> = Lazy::new(get_six_months_ago_unix_timestamp);

const MONTH_TABLE: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn get_six_months_ago_unix_timestamp() -> i64 {
    const SIX_MONTHS_IN_SECS: i64 = 60 * 60 * 24 * 30 * 6;

    get_unix_timestamp_from_systemtime(Ok(SystemTime::now())).unwrap() - SIX_MONTHS_IN_SECS
}
