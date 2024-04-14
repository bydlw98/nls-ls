use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::str;
use std::time::SystemTime;

use chrono::offset::{Local, TimeZone};
use chrono::{DateTime, Datelike, LocalResult, Timelike};
use nls_term_grid::{Alignment, GridCell};
use once_cell::sync::OnceCell;

use crate::config::Config;
use crate::output::GridCellExts;
use crate::utils::systemtime_to_unix_timestamp;

/// Formats a timestamp into a left aligned `GridCell`.
///
/// If timestamp is within 6 months ago from now, the
/// format will be equivalent to the output of:
/// ```sh
/// date "+%b %e %H:%M"
/// ```
///
/// Else format will be equivalent to the output of:
/// ```sh
/// date "+%b %e  %Y"
/// ```
///
/// If timestamp is invalid e.g. timestamp is out of range,
/// a left aligned error `GridCell` is returned instead.
pub fn format_timestamp(timestamp: i64, config: &Config) -> GridCell {
    match Local.timestamp_opt(timestamp, 0) {
        LocalResult::Single(datetime) => {
            static SIX_MONTHS_AGO_TIMESTAMP: OnceCell<i64> = OnceCell::new();
            let timestamp_style = config.theme.timestamp_style();
            let mut buffer = TimestampBuffer::new();

            let six_months_ago_timestamp =
                SIX_MONTHS_AGO_TIMESTAMP.get_or_init(get_six_months_ago_timestamp);

            let timestamp_str = if timestamp > *six_months_ago_timestamp {
                buffer.format_recent(datetime)
            } else {
                buffer.format_old(datetime)
            };

            GridCell::from_ascii_str_with_style(timestamp_str, timestamp_style)
        }
        _ => GridCell::error_cell(Alignment::Left),
    }
}

/// A correctly sized stack allocated array of bytes for the formatted timestamp to be written into.
///
/// Api and implementation is inspired by [`dtolnay/itoa`](https://crates.io/crates/itoa).
struct TimestampBuffer {
    bytes: [MaybeUninit<u8>; Self::CAPACITY],
}

impl TimestampBuffer {
    /// Capacity is due to:
    /// *  3 -> abmon
    /// *  1 -> space
    /// *  2 -> day
    /// *  2 -> space
    /// * 11 -> year, chrono::DateTime::year() returns a i32. The max length of a i32 is 11 bytes
    const CAPACITY: usize = 3 + 1 + 2 + 2 + 11;

    /// Creates a new [`TimestampBuffer`]
    fn new() -> Self {
        Self {
            bytes: [MaybeUninit::<u8>::uninit(); Self::CAPACITY],
        }
    }

    /// Formats timestamp into this buffer and returns a str reference to it.
    ///
    /// Formatted timestamp will be equivalent to the output of:
    /// ```sh
    /// date "+%b %e %H:%M"
    /// ```
    fn format_recent(&mut self, datetime: DateTime<Local>) -> &str {
        let bytes_ptr = self.bytes.as_mut_ptr() as *mut u8;
        let mut bytes_len: isize = 0;

        unsafe {
            let abmon_lut_ptr = ABMON_LUT.as_ptr().offset(datetime.month0() as isize * 4);
            ptr::copy_nonoverlapping(abmon_lut_ptr, bytes_ptr, 4);
            bytes_len += 4;
        }

        unsafe {
            let day_lut_ptr = DAY_LUT.as_ptr().offset(datetime.day0() as isize * 3);
            ptr::copy_nonoverlapping(day_lut_ptr, bytes_ptr.offset(bytes_len), 3);
            bytes_len += 3;
        }

        unsafe {
            let hour_lut_ptr = HOUR_LUT.as_ptr().offset(datetime.hour() as isize * 3);
            ptr::copy_nonoverlapping(hour_lut_ptr, bytes_ptr.offset(bytes_len), 3);
            bytes_len += 3;
        }

        unsafe {
            let minute_lut_ptr = MINUTE_LUT.as_ptr().offset(datetime.minute() as isize * 2);
            ptr::copy_nonoverlapping(minute_lut_ptr, bytes_ptr.offset(bytes_len), 2);
            bytes_len += 2;
        }

        let bytes = unsafe { slice::from_raw_parts(bytes_ptr, bytes_len as usize) };

        unsafe { str::from_utf8_unchecked(bytes) }
    }

    /// Formats timestamp into this buffer and returns a str reference to it.
    ///
    /// Formatted timestamp will be equivalent to the output of:
    /// ```sh
    /// date "+%b %e  %Y"
    /// ```
    fn format_old(&mut self, datetime: DateTime<Local>) -> &str {
        let bytes_ptr = self.bytes.as_mut_ptr() as *mut u8;
        let mut bytes_len: isize = 0;

        unsafe {
            let abmon_lut_ptr = ABMON_LUT.as_ptr().offset(datetime.month0() as isize * 4);
            ptr::copy_nonoverlapping(abmon_lut_ptr, bytes_ptr, 4);
            bytes_len += 4;
        }

        unsafe {
            let day_lut_ptr = DAY_LUT.as_ptr().offset(datetime.day0() as isize * 3);
            ptr::copy_nonoverlapping(day_lut_ptr, bytes_ptr.offset(bytes_len), 3);
            bytes_len += 3;
        }

        self.push_formatted_year(&mut bytes_len, datetime.year());

        let bytes = unsafe { slice::from_raw_parts(bytes_ptr, bytes_len as usize) };

        unsafe { str::from_utf8_unchecked(bytes) }
    }

    /// Pushes into this buffer a space and a year, zero-padded to 4 digits.
    fn push_formatted_year(&mut self, bytes_len: &mut isize, year: i32) {
        let mut year_buffer = itoa::Buffer::new();
        let year_str = year_buffer.format(year);
        let year_ptr = year_str.as_ptr();
        let year_len = year_str.len();

        let mut padding_len: isize = 4 - year_len as isize;
        // If padding_len <= 0 then the year is >= 4 digits,
        // thus we only need one byte for the leading space.
        //
        // Else year is less than 4 digits.
        // The current padding_len only contains the number of
        // zeros to pad year to 4 digits.
        // Thus padding_len is incremented by 1 to include the leading zero.
        if padding_len <= 0 {
            padding_len = 1;
        } else {
            padding_len += 1;
        }

        let year_padding_lut_ptr = YEAR_PADDING_LUT.as_ptr();
        let bytes_ptr = self.bytes.as_mut_ptr() as *mut u8;
        unsafe {
            ptr::copy_nonoverlapping(
                year_padding_lut_ptr,
                bytes_ptr.offset(*bytes_len),
                padding_len as usize,
            );
            *bytes_len += padding_len;
        }
        unsafe {
            ptr::copy_nonoverlapping(year_ptr, bytes_ptr.offset(*bytes_len), year_len);
            *bytes_len += year_len as isize;
        }
    }
}

/// A lookup table of abmon values.
///
/// Each value consists of 4 bytes:
/// * 3 -> an abmon.
/// * 1 -> a trailing space character.
const ABMON_LUT: &[u8] = b"Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec ";

/// A lookup table of day values.
///
/// Each value consists of 3 bytes:
/// * 2 -> a right aligned day.
/// * 1 -> a trailing space character.
const DAY_LUT: &[u8] = b" 1  2  3  4  5  6  7  8  9 10 \
                         11 12 13 14 15 16 17 18 19 20 \
                         21 22 23 24 25 26 27 28 29 30 \
                         31 ";

/// A lookup table of hour values.
///
/// Each value consists of 3 bytes:
/// * 2 -> a hour.
/// * 1 -> a colon character.
const HOUR_LUT: &[u8] = b"00:01:02:03:04:05:06:07:08:09:\
                          10:11:12:13:14:15:16:17:18:19:\
                          20:21:22:23:";

/// A lookup table of minute values.
///
/// Each value consists of 2 bytes:
/// * 2 -> a minute.
const MINUTE_LUT: &[u8] = b"00010203040506070809\
                            10111213141516171819\
                            20212223242526272829\
                            30313233343536373839\
                            40414243444546474849\
                            50515253545556575859\
                            60";

/// A lookup table of year padding.
///
/// Lookup table consists of:
/// * 1 -> padding space
/// * 3 -> paddings zeros
const YEAR_PADDING_LUT: &[u8] = b" 000";

/// Returns a unix timestamp 6 months ago from now.
fn get_six_months_ago_timestamp() -> i64 {
    systemtime_to_unix_timestamp(Ok(SystemTime::now())).unwrap() - SIX_MONTHS_IN_SECS
}

const SIX_MONTHS_IN_SECS: i64 = 60 * 60 * 24 * 30 * 6;

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    use crate::config::Config;
    use crate::theme::ThemeConfig;

    #[test]
    fn test_format_timestamp_recent() {
        let datetime = Local::now();
        let timestamp = datetime.timestamp();
        let config = Config::default();

        let correct_timestamp_format = datetime.format("%b %e %H:%M").to_string();
        let correct_timestamp_width = correct_timestamp_format.len();
        let correct_timestamp_cell = GridCell {
            contents: correct_timestamp_format,
            width: correct_timestamp_width,
            alignment: Alignment::Left,
        };

        assert_eq!(format_timestamp(timestamp, &config), correct_timestamp_cell);
    }

    #[test]
    fn test_format_timestamp_recent_with_style() {
        let datetime = Local::now();
        let timestamp = datetime.timestamp();
        let config = Config {
            theme: ThemeConfig::with_default_colors(),
            ..Default::default()
        };

        let correct_timestamp_format = datetime.format("%b %e %H:%M").to_string();
        let correct_timestamp_width = correct_timestamp_format.len();
        let timestamp_style = config.theme.timestamp_style().unwrap();
        let correct_timestamp_cell = GridCell {
            contents: format!(
                "\x1b[{}m{}\x1b[0m",
                timestamp_style, correct_timestamp_format
            ),
            width: correct_timestamp_width,
            alignment: Alignment::Left,
        };

        assert_eq!(format_timestamp(timestamp, &config), correct_timestamp_cell);
    }

    #[test]
    fn test_format_timestamp_old() {
        let six_months = Duration::from_secs(SIX_MONTHS_IN_SECS as u64);
        let datetime: DateTime<Local> = DateTime::from(SystemTime::now() - six_months);
        let timestamp = datetime.timestamp();
        let config = Config::default();

        let correct_timestamp_format = datetime.format("%b %e  %Y").to_string();
        let correct_timestamp_width = correct_timestamp_format.len();
        let correct_timestamp_cell = GridCell {
            contents: correct_timestamp_format,
            width: correct_timestamp_width,
            alignment: Alignment::Left,
        };

        assert_eq!(format_timestamp(timestamp, &config), correct_timestamp_cell);
    }

    #[test]
    fn test_format_timestamp_old_with_style() {
        let six_months = Duration::from_secs(SIX_MONTHS_IN_SECS as u64);
        let datetime: DateTime<Local> = DateTime::from(SystemTime::now() - six_months);
        let timestamp = datetime.timestamp();
        let config = Config {
            theme: ThemeConfig::with_default_colors(),
            ..Default::default()
        };

        let correct_timestamp_format = datetime.format("%b %e  %Y").to_string();
        let correct_timestamp_width = correct_timestamp_format.len();
        let timestamp_style = config.theme.timestamp_style().unwrap();
        let correct_timestamp_cell = GridCell {
            contents: format!(
                "\x1b[{}m{}\x1b[0m",
                timestamp_style, correct_timestamp_format
            ),
            width: correct_timestamp_width,
            alignment: Alignment::Left,
        };

        assert_eq!(format_timestamp(timestamp, &config), correct_timestamp_cell);
    }

    #[test]
    fn test_format_timestamp_invalid() {
        let config = Config::default();
        let correct_timestamp_cell = GridCell {
            contents: String::from("?"),
            width: 1,
            alignment: Alignment::Left,
        };

        assert_eq!(format_timestamp(i64::MAX, &config), correct_timestamp_cell);
    }

    #[test]
    fn test_timestamp_buffer_format_recent() {
        let datetime = Local::now();
        let mut buffer = TimestampBuffer::new();

        assert_eq!(
            buffer.format_recent(datetime),
            datetime.format("%b %e %H:%M").to_string()
        );
    }

    #[test]
    fn test_timestamp_buffer_format_old() {
        let datetime = Local::now();
        let mut buffer = TimestampBuffer::new();

        assert_eq!(
            buffer.format_old(datetime),
            datetime.format("%b %e  %Y").to_string()
        );
    }

    #[test]
    fn test_timestamp_buffer_push_formatted_year_four_digits() {
        let mut buffer = TimestampBuffer::new();
        let mut bytes_len: isize = 0;
        buffer.push_formatted_year(&mut bytes_len, 2024);

        let bytes_ptr = buffer.bytes.as_ptr() as *const u8;
        let bytes = unsafe { slice::from_raw_parts(bytes_ptr, bytes_len as usize) };
        let year_str = unsafe { str::from_utf8_unchecked(bytes) };

        assert_eq!(year_str, " 2024");
    }

    #[test]
    fn test_timestamp_buffer_push_formatted_year_less_than_four_digits() {
        let mut buffer = TimestampBuffer::new();
        let mut bytes_len: isize = 0;
        buffer.push_formatted_year(&mut bytes_len, 202);

        let bytes_ptr = buffer.bytes.as_ptr() as *const u8;
        let bytes = unsafe { slice::from_raw_parts(bytes_ptr, bytes_len as usize) };
        let year_str = unsafe { str::from_utf8_unchecked(bytes) };

        assert_eq!(year_str, " 0202");
    }

    #[test]
    fn test_timestamp_buffer_push_formatted_year_more_than_four_digits() {
        let mut buffer = TimestampBuffer::new();
        let mut bytes_len: isize = 0;
        buffer.push_formatted_year(&mut bytes_len, 20245);

        let bytes_ptr = buffer.bytes.as_ptr() as *const u8;
        let bytes = unsafe { slice::from_raw_parts(bytes_ptr, bytes_len as usize) };
        let year_str = unsafe { str::from_utf8_unchecked(bytes) };

        assert_eq!(year_str, " 20245");
    }

    #[test]
    fn test_get_six_months_ago_timestamp() {
        let six_months = Duration::from_secs(SIX_MONTHS_IN_SECS as u64);
        let datetime = Local::now() - six_months;

        assert_eq!(get_six_months_ago_timestamp(), datetime.timestamp());
    }
}
