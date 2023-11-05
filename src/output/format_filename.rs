#[cfg(unix)]
use std::fs::FileType;
use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;

use crate::config::{Config, IndicatorStyle};
use crate::ls_colors::get_file_extension;
#[cfg(unix)]
use crate::os::unix::sys_prelude::*;
#[cfg(unix)]
use crate::utils::HasMaskSetExt;

use super::DisplayCell;

pub fn format_filename(
    path: &Path,
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    let file_type = metadata.file_type();

    if file_type.is_file() {
        internal_format_regular_file(file_name, metadata, config)
    } else if file_type.is_dir() {
        internal_format_dir(file_name, metadata, config)
    } else if file_type.is_symlink() {
        internal_format_symlink(path, file_name, config)
    } else {
        cfg_if::cfg_if! {
            if #[cfg(unix)] {
                internal_format_unix_file_type_exts(file_name, file_type, config)
            } else {
                DisplayCell::from(file_name.to_string())
            }
        }
    }
}

#[cfg(unix)]
fn internal_format_unix_file_type_exts(
    file_name: &str,
    file_type: FileType,
    config: &Config,
) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;

    if file_type.is_block_device() {
        return DisplayCell::from_str_with_style(file_name, ls_colors.block_device_style());
    } else if file_type.is_char_device() {
        return DisplayCell::from_str_with_style(file_name, ls_colors.char_device_style());
    } else if file_type.is_fifo() {
        let mut filename_cell = DisplayCell::from_str_with_style(file_name, ls_colors.fifo_style());
        if indicator_style.others() {
            filename_cell.push_char(IndicatorStyle::FIFO);
        }
        return filename_cell;
    } else if file_type.is_socket() {
        let mut filename_cell =
            DisplayCell::from_str_with_style(file_name, ls_colors.socket_style());
        if indicator_style.others() {
            filename_cell.push_char(IndicatorStyle::SOCKET);
        }
        return filename_cell;
    } else {
        return DisplayCell::from(file_name.to_string());
    }
}

#[cfg(unix)]
fn internal_format_regular_file(
    file_name: &str,
    metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    const EXEC_MASK: u32 = c::S_IXUSR | c::S_IXGRP | c::S_IXOTH;
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;
    let st_mode = metadata.mode();

    let mut filename_cell = if st_mode.has_mask_set(c::S_ISUID) {
        DisplayCell::from_str_with_style(file_name, ls_colors.setuid_style())
    } else if st_mode.has_mask_set(c::S_ISGID) {
        DisplayCell::from_str_with_style(file_name, ls_colors.setgid_style())
    } else if st_mode.has_bit_in_mask_set(EXEC_MASK) {
        DisplayCell::from_str_with_style(file_name, ls_colors.exec_style())
    } else if metadata.nlink() > 1 {
        DisplayCell::from_str_with_style(file_name, ls_colors.multiple_hard_links_style())
    } else {
        let extension = get_file_extension(file_name);
        if extension.is_empty() {
            DisplayCell::from_str_with_style(file_name, ls_colors.file_style())
        } else {
            DisplayCell::from_str_with_style(file_name, ls_colors.extension_style(extension))
        }
    };

    if indicator_style.others() && st_mode.has_bit_in_mask_set(EXEC_MASK) {
        filename_cell.push_char(IndicatorStyle::EXEC);
    }

    filename_cell
}

#[cfg(not(unix))]
fn internal_format_regular_file(
    file_name: &str,
    _metadata: &Metadata,
    config: &Config,
) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;
    let extension = get_file_extension(file_name);
    if extension.is_empty() {
        return DisplayCell::from_str_with_style(file_name, ls_colors.file_style());
    } else {
        #[cfg(windows)]
        if ["exe", "bat", "cmd"].contains(&&extension.as_str()) {
            let mut filename_cell =
                DisplayCell::from_str_with_style(file_name, ls_colors.exec_style());
            if indicator_style.others() {
                filename_cell.push_char(IndicatorStyle::EXEC);
            }
            return filename_cell;
        }

        return DisplayCell::from_str_with_style(file_name, ls_colors.extension_style(extension));
    }
}

fn internal_format_dir(file_name: &str, _metadata: &Metadata, config: &Config) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;

    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            let st_mode = _metadata.mode();
            let mut filename_cell = match (
                st_mode.has_mask_set(c::S_ISVTX),
                st_mode.has_mask_set(c::S_IWOTH),
            ) {
                (false, false) => DisplayCell::from_str_with_style(file_name, ls_colors.dir_style()),
                (true, false) => DisplayCell::from_str_with_style(file_name, ls_colors.dir_sticky_style()),
                (false, true) => {
                    DisplayCell::from_str_with_style(file_name, ls_colors.dir_other_writeable_style())
                }
                _ => DisplayCell::from_str_with_style(
                    file_name,
                    ls_colors.dir_sticky_and_other_writeable_style(),
                ),
            };
        } else {
            let mut filename_cell = DisplayCell::from_str_with_style(file_name, ls_colors.dir_style());
        }
    }

    if indicator_style.dir() {
        filename_cell.push_char(IndicatorStyle::DIR);
    }

    filename_cell
}

fn internal_format_symlink(path: &Path, file_name: &str, config: &Config) -> DisplayCell {
    let indicator_style = config.indicator_style;
    let ls_colors = &config.ls_colors;
    let mut filename_cell = DisplayCell::from_str_with_style(file_name, ls_colors.symlink_style());

    if indicator_style.others() && !config.output_format.is_long() {
        filename_cell.push_char(IndicatorStyle::SYMLINK);
    }

    if config.output_format.is_long() {
        filename_cell.push_str_with_width(" -> ", 4);

        match path.read_link() {
            Ok(target_name) => match path.metadata() {
                Ok(target_metadata) => {
                    let target_name_str: &str = &target_name.to_string_lossy();
                    filename_cell.append(format_filename(
                        path,
                        target_name_str,
                        &target_metadata,
                        config,
                    ));
                }
                Err(err) => {
                    filename_cell.push_str(&target_name.to_string_lossy());
                    eprintln!(
                        "nls: unable to get link metadata of '{}': {}",
                        path.display(),
                        err
                    );
                }
            },
            Err(err) => {
                filename_cell.push_char('?');
                eprintln!("nls: unable to readlink '{}': {}", path.display(), err);
            }
        }
    }

    filename_cell
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod test {
    use super::*;

    #[cfg(unix)]
    use std::path::Path;

    use unicode_width::UnicodeWidthStr;

    use crate::ls_colors::LsColors;

    #[test]
    fn test_format_filename_regular_file() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.dir_style();

        internal_test_format_filename_dir(None, IndicatorStyle::Never, false, ansi_style_str);
        internal_test_format_filename_dir(None, IndicatorStyle::Slash, true, ansi_style_str);
        internal_test_format_filename_dir(None, IndicatorStyle::Classify, true, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_sticky_dir() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.dir_sticky_style();
        let mode: Option<u32> = Some(0o755 | c::S_ISVTX);

        internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_other_writeable_dir() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.dir_other_writeable_style();
        let mode: Option<u32> = Some(0o755 | c::S_IWOTH);

        internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_sticky_and_other_writeable_dir() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.dir_sticky_and_other_writeable_style();
        let mode: Option<u32> = Some(0o755 | c::S_ISVTX | c::S_IWOTH);

        internal_test_format_filename_dir(mode, IndicatorStyle::Never, false, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Slash, true, ansi_style_str);
        internal_test_format_filename_dir(mode, IndicatorStyle::Classify, true, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_symlink_non_long_format() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.symlink_style();

        internal_test_format_filename_symlink_long_format(
            IndicatorStyle::Never,
            false,
            ansi_style_str,
        );
        internal_test_format_filename_symlink_long_format(
            IndicatorStyle::Slash,
            true,
            ansi_style_str,
        );
        internal_test_format_filename_symlink_long_format(
            IndicatorStyle::Classify,
            true,
            ansi_style_str,
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_block_device() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.block_device_style();

        internal_test_format_filename_block_device(IndicatorStyle::Never, ansi_style_str);
        internal_test_format_filename_block_device(IndicatorStyle::Slash, ansi_style_str);
        internal_test_format_filename_block_device(IndicatorStyle::Classify, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_char_device() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.char_device_style();

        internal_test_format_filename_char_device(IndicatorStyle::Never, ansi_style_str);
        internal_test_format_filename_char_device(IndicatorStyle::Slash, ansi_style_str);
        internal_test_format_filename_char_device(IndicatorStyle::Classify, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_fifo() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
        let ansi_style_str = ls_colors.fifo_style();

        internal_test_format_filename_fifo(IndicatorStyle::Never, false, ansi_style_str);
        internal_test_format_filename_fifo(IndicatorStyle::Slash, false, ansi_style_str);
        internal_test_format_filename_fifo(IndicatorStyle::Classify, true, ansi_style_str);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_filename_socket() {
        let mut ls_colors = LsColors::default();
        ls_colors.init();
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
        let mut correct_filename_cell = DisplayCell::from(symlink_path_string.clone());
        let target_name_cell =
            format_filename(&target_path, &target_path_string, &target_metadata, &config);
        correct_filename_cell.push_str(" -> ");
        correct_filename_cell.append(target_name_cell);

        assert_eq!(filename_cell, correct_filename_cell);

        config.ls_colors.init();
        let filename_cell_with_color = format_filename(
            &symlink_path,
            &symlink_path_string,
            &symlink_metadata,
            &config,
        );
        let mut correct_filename_cell_with_color = match &ansi_style_str {
            Some(ansi_style_str) => DisplayCell::from(format!(
                "\x1b[{}m{}\x1b[0m",
                ansi_style_str, symlink_path_string
            )),
            None => DisplayCell::from(symlink_path_string.clone()),
        };
        let target_name_cell =
            format_filename(&target_path, &target_path_string, &target_metadata, &config);
        correct_filename_cell_with_color.push_str(" -> ");
        correct_filename_cell_with_color.append(target_name_cell);

        correct_filename_cell_with_color.set_width(
            UnicodeWidthStr::width(&*symlink_path_string)
                + 4 // " -> " is 4 chars long
                + UnicodeWidthStr::width(&*target_path_string)
                + correct_filename_has_indicator as usize,
        );
        assert_eq!(filename_cell_with_color, correct_filename_cell_with_color);
    }

    #[cfg(unix)]
    fn internal_test_format_filename_block_device(
        indicator_style: IndicatorStyle,
        ansi_style_str: Option<&str>,
    ) {
        let block_device_path = Path::new("/dev/sda");
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
        let mut correct_filename_cell = DisplayCell::from(path_string.clone());
        if correct_filename_has_indicator {
            correct_filename_cell.push_char(indicator_symbol);
        }
        assert_eq!(filename_cell, correct_filename_cell);

        config.ls_colors.init();
        let filename_cell_with_color = format_filename(path, &path_string, &metadata, &config);
        let mut correct_filename_cell_with_color = match &ansi_style_str {
            Some(ansi_style_str) => {
                DisplayCell::from(format!("\x1b[{}m{}\x1b[0m", ansi_style_str, path_string))
            }
            None => DisplayCell::from(path_string.clone()),
        };

        correct_filename_cell_with_color.set_width(UnicodeWidthStr::width(&*path_string));
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
}
