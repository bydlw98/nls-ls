use nls_term_grid::Alignment;

use crate::config::{Config, SizeFormat};
use crate::output::{GridCell, GridCellExts};

pub fn format_size(size: u64, config: &Config) -> GridCell {
    let size_style = config.theme.size_style();
    let mut buffer = size_fmt::Buffer::new();

    let size_str = match config.size_format {
        SizeFormat::Raw => buffer.raw_fmt(size),
        SizeFormat::HumanReadable => buffer.human_fmt(size),
        SizeFormat::Si => buffer.si_fmt(size),
        SizeFormat::Iec => buffer.iec_fmt(size),
    };

    let mut size_cell = GridCell::from_ascii_str_with_style(size_str, size_style);
    size_cell.alignment = Alignment::Right;

    size_cell
}

#[cfg(test)]
mod tests {
    use super::*;

    use compact_str::{format_compact, CompactString};

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
            contents: CompactString::new(raw_str),
            width: raw_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::HumanReadable;
        let correct_cell = GridCell {
            contents: CompactString::new(human_readable_str),
            width: human_readable_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Si;
        let correct_cell = GridCell {
            contents: CompactString::new(si_str),
            width: si_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Iec;
        let correct_cell = GridCell {
            contents: CompactString::new(iec_str),
            width: iec_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.theme = ThemeConfig::with_default_colors();
        let size_style: &str = config.theme.size_style().unwrap();

        config.size_format = SizeFormat::Raw;
        let correct_cell = GridCell {
            contents: format_compact!("\x1b[{}m{}\x1b[0m", size_style, raw_str),
            width: raw_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::HumanReadable;
        let correct_cell = GridCell {
            contents: format_compact!("\x1b[{}m{}\x1b[0m", size_style, human_readable_str),
            width: human_readable_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Si;
        let correct_cell = GridCell {
            contents: format_compact!("\x1b[{}m{}\x1b[0m", size_style, si_str),
            width: si_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);

        config.size_format = SizeFormat::Iec;
        let correct_cell = GridCell {
            contents: format_compact!("\x1b[{}m{}\x1b[0m", size_style, iec_str),
            width: iec_str.len(),
            alignment: Alignment::Right,
        };
        assert_eq!(format_size(size, &config), correct_cell);
    }
}
