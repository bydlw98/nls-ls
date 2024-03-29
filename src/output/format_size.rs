use nls_term_grid::{Alignment, GridCell};

use crate::config::{Config, SizeFormat};
use crate::output::GridCellExts;

pub fn format_size(size: u64, config: &Config) -> GridCell {
    let size_style = config.theme.size_style();

    match config.size_format {
        SizeFormat::Raw => GridCell::from_num_with_style(size, size_style),
        SizeFormat::HumanReadable => human_readable(size, size_style),
        SizeFormat::Si => si(size, size_style),
        SizeFormat::Iec => iec(size, size_style),
    }
}

macro_rules! internal_format_size_impl {
    (
        $size:ident,
        $size_style:ident,
        $factor:literal,
        $prefix_1:literal,
        $prefix_2:literal,
        $prefix_3:literal,
        $prefix_4:literal,
        $prefix_5:literal,
        $prefix_6:literal
    ) => {
        if $size < u64::pow($factor, 1) {
            GridCell::from_num_with_style($size, $size_style)
        } else if $size < u64::pow($factor, 2) {
            format_size_with_prefix($size, u64::pow($factor, 1), $prefix_1, $size_style)
        } else if $size < u64::pow($factor, 3) {
            format_size_with_prefix($size, u64::pow($factor, 2), $prefix_2, $size_style)
        } else if $size < u64::pow($factor, 4) {
            format_size_with_prefix($size, u64::pow($factor, 3), $prefix_3, $size_style)
        } else if $size < u64::pow($factor, 5) {
            format_size_with_prefix($size, u64::pow($factor, 4), $prefix_4, $size_style)
        } else if $size < u64::pow($factor, 6) {
            format_size_with_prefix($size, u64::pow($factor, 5), $prefix_5, $size_style)
        } else {
            format_size_with_prefix($size, u64::pow($factor, 6), $prefix_6, $size_style)
        }
    };
}

/// format size using factors of 1024 like 1.0K 200M 3.0G etc
fn human_readable(size: u64, size_style: Option<&str>) -> GridCell {
    internal_format_size_impl!(size, size_style, 1024, "K", "M", "G", "T", "P", "E")
}

/// format size using factors of 1000 like 1.0k 200M 3.0G etc
fn si(size: u64, size_style: Option<&str>) -> GridCell {
    internal_format_size_impl!(size, size_style, 1000, "k", "M", "G", "T", "P", "E")
}

/// format size using factors of 1024 like 1.0Ki 200Mi 3.0Gi etc
fn iec(size: u64, size_style: Option<&str>) -> GridCell {
    internal_format_size_impl!(size, size_style, 1024, "Ki", "Mi", "Gi", "Ti", "Pi", "Ei")
}

fn format_size_with_prefix(
    num: u64,
    factor: u64,
    prefix: &str,
    size_style: Option<&str>,
) -> GridCell {
    let num_f64 = (num as f64) / (factor as f64);

    if num_f64 >= 10.0 {
        let size_string = format!("{}{}", num_f64.ceil() as u64, prefix);

        let mut size_cell = GridCell::from_ascii_str_with_style(&size_string, size_style);
        size_cell.alignment = Alignment::Right;

        size_cell
    } else {
        // E.g 123.456
        // multiply by 10 first to move the first decimal digit in front of decimal point
        //      123.456 * 10 = 1234.56
        // get the ceil of value
        //      123.456.ceil() = 124
        // divide by 10
        //      124 / 10 = 12.4
        let size_string = format!("{:.1}{}", ((num_f64 * 10.0).ceil() / 10.0), prefix);

        let mut size_cell = GridCell::from_ascii_str_with_style(&size_string, size_style);
        size_cell.alignment = Alignment::Right;

        size_cell
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::theme::ThemeConfig;

    #[test]
    fn test_format_size_512() {
        internal_test_format_size(512, "512", "512", "512", "512")
    }

    #[test]
    fn test_format_size_1000() {
        internal_test_format_size(1000, "1000", "1000", "1.0k", "1000")
    }

    #[test]
    fn test_format_size_100_000() {
        internal_test_format_size(100_000, "100000", "98K", "100k", "98Ki")
    }

    #[test]
    fn test_format_size_2_000_000() {
        internal_test_format_size(2_000_000, "2000000", "2.0M", "2.0M", "2.0Mi")
    }

    #[test]
    fn test_format_size_200_000_000() {
        internal_test_format_size(200_000_000, "200000000", "191M", "200M", "191Mi")
    }

    #[test]
    fn test_format_size_3_000_000_000() {
        internal_test_format_size(3_000_000_000, "3000000000", "2.8G", "3.0G", "2.8Gi")
    }

    #[test]
    fn test_format_size_300_000_000_000() {
        internal_test_format_size(300_000_000_000, "300000000000", "280G", "300G", "280Gi")
    }

    #[test]
    fn test_format_size_4_000_000_000_000() {
        internal_test_format_size(4_000_000_000_000, "4000000000000", "3.7T", "4.0T", "3.7Ti")
    }

    #[test]
    fn test_format_size_400_000_000_000_000() {
        internal_test_format_size(
            400_000_000_000_000,
            "400000000000000",
            "364T",
            "400T",
            "364Ti",
        )
    }

    #[test]
    fn test_format_size_5_000_000_000_000_000() {
        internal_test_format_size(
            5_000_000_000_000_000,
            "5000000000000000",
            "4.5P",
            "5.0P",
            "4.5Pi",
        )
    }

    #[test]
    fn test_format_size_500_000_000_000_000_000() {
        internal_test_format_size(
            500_000_000_000_000_000,
            "500000000000000000",
            "445P",
            "500P",
            "445Pi",
        )
    }

    #[test]
    fn test_format_size_6000_000_000_000_000_000() {
        internal_test_format_size(
            6_000_000_000_000_000_000,
            "6000000000000000000",
            "5.3E",
            "6.0E",
            "5.3Ei",
        )
    }

    #[test]
    fn test_format_size_600_000_000_000_000_000_000() {
        internal_test_format_size(
            18_000_000_000_000_000_000,
            "18000000000000000000",
            "16E",
            "18E",
            "16Ei",
        )
    }

    #[allow(clippy::field_reassign_with_default)]
    fn internal_test_format_size(
        size: u64,
        raw_str: &str,
        human_readable_str: &str,
        si_str: &str,
        iec_str: &str,
    ) {
        let mut config = Config::default();

        config.size_format = SizeFormat::Raw;
        let correct_cell = GridCell {
            contents: String::from(raw_str),
            width: raw_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::HumanReadable;
        let correct_cell = GridCell {
            contents: String::from(human_readable_str),
            width: human_readable_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Si;
        let correct_cell = GridCell {
            contents: String::from(si_str),
            width: si_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Iec;
        let correct_cell = GridCell {
            contents: String::from(iec_str),
            width: iec_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.theme = ThemeConfig::with_default_colors();
        let size_style: &str = config.theme.size_style().unwrap();

        config.size_format = SizeFormat::Raw;
        let correct_cell = GridCell {
            contents: format!("\x1b[{}m{}\x1b[0m", size_style, raw_str),
            width: raw_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::HumanReadable;
        let correct_cell = GridCell {
            contents: format!("\x1b[{}m{}\x1b[0m", size_style, human_readable_str),
            width: human_readable_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Si;
        let correct_cell = GridCell {
            contents: format!("\x1b[{}m{}\x1b[0m", size_style, si_str),
            width: si_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Iec;
        let correct_cell = GridCell {
            contents: format!("\x1b[{}m{}\x1b[0m", size_style, iec_str),
            width: iec_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);
    }
}
