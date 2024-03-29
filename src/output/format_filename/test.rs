#![allow(clippy::field_reassign_with_default)]

use super::*;

#[cfg(unix)]
use std::path::Path;

use unicode_width::UnicodeWidthStr;

use crate::ls_colors::LsColors;
use crate::theme::IconTheme;

#[test]
fn test_create_filename_cell() {
    let file_name = "dir1";

    let cell = create_filename_cell(file_name, None, None);
    let correct_cell = GridCell {
        contents: String::from(file_name),
        width: 4,
        alignment: Alignment::Left,
    };

    assert_eq!(cell, correct_cell);
}

#[test]
fn test_create_filename_cell_with_color() {
    let mut config = Config::default();
    config.ls_colors = LsColors::with_colors();
    let ls_colors = &config.ls_colors;
    let file_name = "dir1";

    let cell = create_filename_cell(file_name, ls_colors.dir_style(), None);
    let correct_cell = GridCell {
        contents: format!(
            "\x1b[{}m{}\x1b[0m",
            ls_colors.dir_style().unwrap(),
            file_name
        ),
        width: 4,
        alignment: Alignment::Left,
    };

    assert_eq!(cell, correct_cell);
}

#[test]
fn test_create_filename_cell_with_icon() {
    let mut config = Config::default();
    config.icons = IconTheme::with_default_icons();
    let icons = &config.icons;
    let file_name = "dir1";

    let cell = create_filename_cell(file_name, None, icons.dir_icon(file_name));
    let correct_cell = GridCell {
        contents: format!("{} {}", icons.dir_icon(file_name).unwrap(), file_name),
        width: 6,
        alignment: Alignment::Left,
    };

    assert_eq!(cell, correct_cell);
}

#[test]
fn test_create_filename_cell_with_icon_and_color() {
    let mut config = Config::default();
    config.ls_colors = LsColors::with_colors();
    let ls_colors = &config.ls_colors;
    config.icons = IconTheme::with_default_icons();
    let icons = &config.icons;
    let file_name = "dir1";

    let cell = create_filename_cell(file_name, ls_colors.dir_style(), icons.dir_icon(file_name));
    let correct_cell = GridCell {
        contents: format!(
            "\x1b[{}m{} {}\x1b[0m",
            ls_colors.dir_style().unwrap(),
            icons.dir_icon(file_name).unwrap(),
            file_name
        ),
        width: 6,
        alignment: Alignment::Left,
    };

    assert_eq!(cell, correct_cell);
}

#[test]
fn test_format_filename_regular_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.file_style();
    let file_name = "file";

    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Classify,
        false,
        ansi_style_str,
    );
}

#[test]
fn test_format_filename_file_with_extension() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.extension_style(String::from("jpeg"));
    let file_name = "file.jpeg";

    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Classify,
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_setuid_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.setuid_style();
    let mode: Option<u32> = Some(0o644 | c::S_ISUID);
    let file_name = "file";

    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_setgid_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.setgid_style();
    let mode: Option<u32> = Some(0o644 | c::S_ISGID);
    let file_name = "file";

    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_multiple_hard_links_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.multiple_hard_links_style();

    internal_test_format_filename_multiple_hard_links_file(
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_multiple_hard_links_file(
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_multiple_hard_links_file(
        IndicatorStyle::Classify,
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_executable_regular_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.exec_style();
    let default_mode: u32 = 0o644;
    let file_name = "file";

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_executable_setuid_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.setuid_style();
    let default_mode: u32 = 0o644 | c::S_ISUID;
    let file_name = "file";

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_executable_setgid_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.setgid_style();
    let default_mode: u32 = 0o644 | c::S_ISGID;
    let file_name = "file";

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let mode: Option<u32> = Some(default_mode | c::S_IXUSR | c::S_IXGRP | c::S_IXOTH);
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        mode,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[cfg(windows)]
#[test]
fn test_format_filename_executable_regular_file() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.exec_style();

    let file_name = "file.exe";
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let file_name = "file.bat";
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );

    let file_name = "file.cmd";
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_file(
        file_name,
        None,
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[test]
fn test_format_filename_regular_dir() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.dir_style();

    internal_test_format_filename_dir(None, IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_dir(None, IndicatorStyle::Slash, true, ansi_style_str);
    internal_test_format_filename_dir(None, IndicatorStyle::Classify, true, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_sticky_dir() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.dir_sticky_style();
    let mode: Option<u32> = Some(0o755 | c::S_ISVTX);

    internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_other_writeable_dir() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.dir_other_writeable_style();
    let mode: Option<u32> = Some(0o755 | c::S_IWOTH);

    internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_sticky_and_other_writeable_dir() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.dir_sticky_and_other_writeable_style();
    let mode: Option<u32> = Some(0o755 | c::S_ISVTX | c::S_IWOTH);

    internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
    internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_symlink_non_long_format() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.symlink_style();

    internal_test_format_filename_symlink_non_long_format(
        IndicatorStyle::Never,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_symlink_non_long_format(
        IndicatorStyle::Slash,
        false,
        ansi_style_str,
    );
    internal_test_format_filename_symlink_non_long_format(
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_symlink_long_format() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.symlink_style();

    internal_test_format_filename_symlink_long_format(IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_symlink_long_format(IndicatorStyle::Slash, true, ansi_style_str);
    internal_test_format_filename_symlink_long_format(
        IndicatorStyle::Classify,
        true,
        ansi_style_str,
    );
}

#[cfg(unix)]
#[test]
fn test_format_filename_block_device() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.block_device_style();

    internal_test_format_filename_block_device(IndicatorStyle::Never, ansi_style_str);
    internal_test_format_filename_block_device(IndicatorStyle::Slash, ansi_style_str);
    internal_test_format_filename_block_device(IndicatorStyle::Classify, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_char_device() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.char_device_style();

    internal_test_format_filename_char_device(IndicatorStyle::Never, ansi_style_str);
    internal_test_format_filename_char_device(IndicatorStyle::Slash, ansi_style_str);
    internal_test_format_filename_char_device(IndicatorStyle::Classify, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_fifo() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.fifo_style();

    internal_test_format_filename_fifo(IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_fifo(IndicatorStyle::Slash, false, ansi_style_str);
    internal_test_format_filename_fifo(IndicatorStyle::Classify, true, ansi_style_str);
}

#[cfg(unix)]
#[test]
fn test_format_filename_socket() {
    let ls_colors = LsColors::with_colors();
    let ansi_style_str = ls_colors.socket_style();

    internal_test_format_filename_socket(IndicatorStyle::Never, false, ansi_style_str);
    internal_test_format_filename_socket(IndicatorStyle::Slash, false, ansi_style_str);
    internal_test_format_filename_socket(IndicatorStyle::Classify, true, ansi_style_str);
}

fn internal_test_format_filename_file(
    file_name: &str,
    mode: Option<u32>,
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let file_path = tmpdir.join(file_name);
    std::fs::File::create(&file_path).unwrap();

    internal_test_format_filename_common(
        &file_path,
        mode,
        indicator_style,
        IndicatorStyle::EXEC,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_multiple_hard_links_file(
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    let file_name = "file";
    let file_name2 = "file2";
    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let file_path = tmpdir.join(file_name);
    let file_path2 = tmpdir.join(file_name2);
    std::fs::File::create(&file_path).unwrap();
    std::fs::hard_link(&file_path, file_path2)
        .unwrap_or_else(|_| panic!("unable to create hard link"));

    internal_test_format_filename_common(
        &file_path,
        None,
        indicator_style,
        IndicatorStyle::EXEC,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

fn internal_test_format_filename_dir(
    mode: Option<u32>,
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    let dir_path = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();

    internal_test_format_filename_common(
        &dir_path,
        mode,
        indicator_style,
        IndicatorStyle::DIR,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_symlink_non_long_format(
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    let target_path = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let symlink_path = tmpdir.join("symlink_file");
    std::os::unix::fs::symlink(target_path, &symlink_path)
        .unwrap_or_else(|_| panic!("unable to create symlink"));

    internal_test_format_filename_common(
        &symlink_path,
        None,
        indicator_style,
        IndicatorStyle::SYMLINK,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_symlink_long_format(
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    use crate::config::OutputFormat;

    let target_path = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let symlink_path = tmpdir.join("symlink_file");
    std::os::unix::fs::symlink(&target_path, &symlink_path)
        .unwrap_or_else(|_| panic!("unable to create symlink"));

    let symlink_path_string = symlink_path.display().to_string();
    let symlink_metadata = symlink_path
        .symlink_metadata()
        .unwrap_or_else(|_| panic!("unable get metadata of '{}'", symlink_path.display()));
    let target_path_string = target_path.display().to_string();
    let target_metadata = target_path
        .symlink_metadata()
        .unwrap_or_else(|_| panic!("unable get metadata of '{}'", target_path.display()));
    let mut config = Config::default();
    config.indicator_style = indicator_style;
    config.output_format = OutputFormat::Long;

    let filename_cell = format_filename(
        &symlink_path,
        &symlink_path_string,
        &symlink_metadata,
        &config,
    );
    let mut correct_filename_cell = GridCell::from(symlink_path_string.clone());
    let target_name_cell =
        format_filename(&target_path, &target_path_string, &target_metadata, &config);
    correct_filename_cell.push_str(" -> ");
    correct_filename_cell.append(target_name_cell);

    assert_eq!(filename_cell, correct_filename_cell);

    config.ls_colors = LsColors::with_colors();
    let filename_cell_with_color = format_filename(
        &symlink_path,
        &symlink_path_string,
        &symlink_metadata,
        &config,
    );
    let mut correct_filename_cell_with_color = match &ansi_style_str {
        Some(ansi_style_str) => GridCell::from(format!(
            "\x1b[{}m{}\x1b[0m",
            ansi_style_str, symlink_path_string
        )),
        None => GridCell::from(symlink_path_string.clone()),
    };
    let target_name_cell =
        format_filename(&target_path, &target_path_string, &target_metadata, &config);
    correct_filename_cell_with_color.push_str(" -> ");
    correct_filename_cell_with_color.append(target_name_cell);

    correct_filename_cell_with_color.width = UnicodeWidthStr::width(&*symlink_path_string)
                + 4 // " -> " is 4 chars long
                + UnicodeWidthStr::width(&*target_path_string)
                + correct_filename_has_indicator as usize;
    assert_eq!(filename_cell_with_color, correct_filename_cell_with_color);
}

#[cfg(unix)]
fn internal_test_format_filename_block_device(
    indicator_style: IndicatorStyle,
    ansi_style_str: Option<&str>,
) {
    #[cfg(target_os = "linux")]
    let block_device_path = Path::new("/dev/sda");

    #[cfg(target_os = "macos")]
    let block_device_path = Path::new("/dev/disk0");

    internal_test_format_filename_common(
        block_device_path,
        None,
        indicator_style,
        '\0',
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_char_device(
    indicator_style: IndicatorStyle,
    ansi_style_str: Option<&str>,
) {
    let char_device_path = Path::new("/dev/null");
    internal_test_format_filename_common(
        char_device_path,
        None,
        indicator_style,
        '\0',
        false,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_fifo(
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let file_path = tmpdir.join("fifo_file");
    mkfifo(&file_path);

    internal_test_format_filename_common(
        &file_path,
        None,
        indicator_style,
        IndicatorStyle::FIFO,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

#[cfg(unix)]
fn internal_test_format_filename_socket(
    indicator_style: IndicatorStyle,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    use std::os::unix::net::UnixListener;

    let tmpdir = tempfile::tempdir()
        .expect("unable to create temp dir")
        .into_path();
    let file_path = tmpdir.join("socket_file");
    UnixListener::bind(&file_path).unwrap();

    internal_test_format_filename_common(
        &file_path,
        None,
        indicator_style,
        IndicatorStyle::SOCKET,
        correct_filename_has_indicator,
        ansi_style_str,
    );
}

fn internal_test_format_filename_common(
    path: &Path,
    _mode: Option<u32>,
    indicator_style: IndicatorStyle,
    indicator_symbol: char,
    correct_filename_has_indicator: bool,
    ansi_style_str: Option<&str>,
) {
    #[cfg(unix)]
    if let Some(mode) = &_mode {
        chmod(path, *mode);
    }

    let path_string = path.display().to_string();
    let metadata = path
        .symlink_metadata()
        .unwrap_or_else(|_| panic!("unable get metadata of '{}'", path.display()));
    let mut config = Config::default();
    config.indicator_style = indicator_style;

    let filename_cell = format_filename(path, &path_string, &metadata, &config);
    let mut correct_filename_cell = GridCell::from(path_string.clone());
    if correct_filename_has_indicator {
        correct_filename_cell.push_char(indicator_symbol);
    }
    assert_eq!(filename_cell, correct_filename_cell);

    config.ls_colors = LsColors::with_colors();
    let filename_cell_with_color = format_filename(path, &path_string, &metadata, &config);
    let mut correct_filename_cell_with_color = match &ansi_style_str {
        Some(ansi_style_str) => {
            GridCell::from(format!("\x1b[{}m{}\x1b[0m", ansi_style_str, path_string))
        }
        None => GridCell::from(path_string.clone()),
    };

    correct_filename_cell_with_color.width = UnicodeWidthStr::width(&*path_string);
    if correct_filename_has_indicator {
        correct_filename_cell_with_color.push_char(indicator_symbol);
    }
    assert_eq!(filename_cell_with_color, correct_filename_cell_with_color);
}

#[cfg(unix)]
fn chmod(path: &Path, mode: u32) {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let path_cstring = CString::new(path.as_os_str().as_bytes()).unwrap();
    unsafe { libc::chmod(path_cstring.as_ptr(), mode as libc::mode_t) };
}

#[cfg(unix)]
fn mkfifo(path: &Path) {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let path_cstring = CString::new(path.as_os_str().as_bytes()).unwrap();
    unsafe { libc::mkfifo(path_cstring.as_ptr(), 0o644) };
}
