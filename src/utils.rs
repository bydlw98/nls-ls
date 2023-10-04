use std::io;
use std::time::SystemTime;

pub fn terminal_width() -> Option<usize> {
    let (terminal_size::Width(term_width), _) = terminal_size::terminal_size()?;

    Some(term_width as usize)
}

pub trait HasMaskSetExt {
    fn has_mask_set(&self, mask: Self) -> bool;
    fn has_bit_in_mask_set(&self, mask: Self) -> bool;
}

impl HasMaskSetExt for u32 {
    fn has_mask_set(&self, mask: Self) -> bool {
        (self & mask) == mask
    }

    fn has_bit_in_mask_set(&self, mask: Self) -> bool {
        (self & mask) != 0
    }
}

pub fn systemtime_to_unix_timestamp(systemtime: Result<SystemTime, io::Error>) -> Option<i64> {
    systemtime
        .map(
            |sys_time| match sys_time.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => Some(duration.as_secs() as i64),
                Err(duration_err) => Some(-(duration_err.duration().as_secs() as i64)),
            },
        )
        .unwrap_or(None)
}
