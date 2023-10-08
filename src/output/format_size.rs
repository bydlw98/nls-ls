use super::DisplayCell;
use crate::config::{Config, SizeFormat};

pub fn format_size(size: u64, config: &Config) -> DisplayCell {
    let size_style = config.theme.size_style();

    match config.size_format {
        SizeFormat::Raw => DisplayCell::from_num_with_style(size, size_style),
        SizeFormat::HumanReadable => human_readable(size, size_style),
        SizeFormat::Si => si(size, size_style),
        SizeFormat::Iec => iec(size, size_style),
    }
}

const KILOBYTE: u64 = 1000u64;
const MEGABYTE: u64 = 1000u64.pow(2);
const GIGABYTE: u64 = 1000u64.pow(3);
const TERABYTE: u64 = 1000u64.pow(4);
const PETABYTE: u64 = 1000u64.pow(5);
const EXABYTE: u64 = 1000u64.pow(6);

const KIBIBYTE: u64 = 1024u64;
const MEBIBYTE: u64 = 1024u64.pow(2);
const GIBIBYTE: u64 = 1024u64.pow(3);
const TEBIBYTE: u64 = 1024u64.pow(4);
const PEBIBYTE: u64 = 1024u64.pow(5);
const EXBIBYTE: u64 = 1024u64.pow(6);

/// format size using factors of 1024 like 1.0K 200M 3.0G etc
fn human_readable(size: u64, size_style: Option<&str>) -> DisplayCell {
    if size < 1024 {
        DisplayCell::from_num_with_style(size, size_style)
    } else if size < MEBIBYTE {
        format_size_with_prefix(size, KIBIBYTE, "K", size_style)
    } else if size < GIBIBYTE {
        format_size_with_prefix(size, MEBIBYTE, "M", size_style)
    } else if size < TEBIBYTE {
        format_size_with_prefix(size, GIBIBYTE, "G", size_style)
    } else if size < PEBIBYTE {
        format_size_with_prefix(size, TEBIBYTE, "T", size_style)
    } else if size < EXBIBYTE {
        format_size_with_prefix(size, PEBIBYTE, "P", size_style)
    } else {
        format_size_with_prefix(size, EXBIBYTE, "E", size_style)
    }
}

/// format size using factors of 1000 like 1.0k 200M 3.0G etc
fn si(size: u64, size_style: Option<&str>) -> DisplayCell {
    if size < 1000 {
        DisplayCell::from_num_with_style(size, size_style)
    } else if size < MEGABYTE {
        format_size_with_prefix(size, KILOBYTE, "k", size_style)
    } else if size < GIGABYTE {
        format_size_with_prefix(size, MEGABYTE, "M", size_style)
    } else if size < TERABYTE {
        format_size_with_prefix(size, GIGABYTE, "G", size_style)
    } else if size < PETABYTE {
        format_size_with_prefix(size, TERABYTE, "T", size_style)
    } else if size < EXABYTE {
        format_size_with_prefix(size, PETABYTE, "P", size_style)
    } else {
        format_size_with_prefix(size, EXABYTE, "E", size_style)
    }
}

/// format size using factors of 1024 like 1.0Ki 200Mi 3.0Gi etc
fn iec(size: u64, size_style: Option<&str>) -> DisplayCell {
    if size < 1024 {
        DisplayCell::from_num_with_style(size, size_style)
    } else if size < MEBIBYTE {
        format_size_with_prefix(size, KIBIBYTE, "Ki", size_style)
    } else if size < GIBIBYTE {
        format_size_with_prefix(size, MEBIBYTE, "Mi", size_style)
    } else if size < TEBIBYTE {
        format_size_with_prefix(size, GIBIBYTE, "Gi", size_style)
    } else if size < PEBIBYTE {
        format_size_with_prefix(size, TEBIBYTE, "Ti", size_style)
    } else if size < EXBIBYTE {
        format_size_with_prefix(size, PEBIBYTE, "Pi", size_style)
    } else {
        format_size_with_prefix(size, EXBIBYTE, "Ei", size_style)
    }
}

fn format_size_with_prefix(
    num: u64,
    factor: u64,
    prefix: &str,
    size_style: Option<&str>,
) -> DisplayCell {
    let num_f64 = (num as f64) / (factor as f64);

    if num_f64 >= 10.0 {
        let size_string = format!("{}{}", num_f64.ceil() as u64, prefix);

        DisplayCell::from_ascii_str_with_style(&size_string, size_style).left_aligned(false)
    } else {
        // E.g 123.456
        // multiply by 10 first to move the first decimal digit in front of decimal point
        //      123.456 * 10 = 1234.56
        // get the ceil of value
        //      123.456.ceil() = 124
        // divide by 10
        //      124 / 10 = 12.4
        let size_string = format!("{:.1}{}", ((num_f64 * 10.0).ceil() / 10.0), prefix);

        DisplayCell::from_ascii_str_with_style(&size_string, size_style).left_aligned(false)
    }
}
