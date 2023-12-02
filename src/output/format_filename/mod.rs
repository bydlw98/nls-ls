#[cfg(test)]
mod test;

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
    let icon = &config.icons;

    if file_type.is_block_device() {
        return create_filename_cell(
            file_name,
            ls_colors.block_device_style(),
            icon.block_device,
        );
    } else if file_type.is_char_device() {
        return create_filename_cell(
            file_name,
            ls_colors.char_device_style(),
            icon.char_device,
        );
    } else if file_type.is_fifo() {
        let mut filename_cell =
            create_filename_cell(file_name, ls_colors.fifo_style(), icon.fifo);
        if indicator_style.others() {
            filename_cell.push_char(IndicatorStyle::FIFO);
        }
        return filename_cell;
    } else if file_type.is_socket() {
        let mut filename_cell =
            create_filename_cell(file_name, ls_colors.socket_style(), icon.socket);
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
    let icon = config.icons.file;

    let mut filename_cell = if st_mode.has_mask_set(c::S_ISUID) {
        create_filename_cell(file_name, ls_colors.setuid_style(), icon)
    } else if st_mode.has_mask_set(c::S_ISGID) {
        create_filename_cell(file_name, ls_colors.setgid_style(), icon)
    } else if st_mode.has_bit_in_mask_set(EXEC_MASK) {
        create_filename_cell(file_name, ls_colors.exec_style(), icon)
    } else if metadata.nlink() > 1 {
        create_filename_cell(file_name, ls_colors.multiple_hard_links_style(), icon)
    } else {
        let extension = get_file_extension(file_name);
        if extension.is_empty() {
            create_filename_cell(file_name, ls_colors.file_style(), icon)
        } else {
            create_filename_cell(file_name, ls_colors.extension_style(extension), icon)
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
    let icon = config.icons.dir;

    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            let st_mode = _metadata.mode();
            let mut filename_cell = match (
                st_mode.has_mask_set(c::S_ISVTX),
                st_mode.has_mask_set(c::S_IWOTH),
            ) {
                (false, false) => create_filename_cell(file_name, ls_colors.dir_style(), icon),
                (true, false) => create_filename_cell(file_name, ls_colors.dir_sticky_style(), icon),
                (false, true) => create_filename_cell(file_name, ls_colors.dir_other_writeable_style(), icon),
                _ => create_filename_cell(
                    file_name,
                    ls_colors.dir_sticky_and_other_writeable_style(), icon
                ),
            };
        } else {
            let mut filename_cell = DisplayCell::from_str_with_style(&name, ls_colors.dir_style());
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
    let icon = config.icons.symlink;

    let mut filename_cell = create_filename_cell(file_name, ls_colors.symlink_style(), icon);

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

fn create_filename_cell(
    file_name: &str,
    ansi_style_str: Option<&str>,
    icon: Option<char>,
) -> DisplayCell {
    use crate::output::Alignment;
    use unicode_width::UnicodeWidthStr;

    let mut contents = String::with_capacity(64);
    let mut width: usize = 0;

    if let Some(ansi_style_str) = ansi_style_str {
        contents.push_str("\x1b[");
        contents.push_str(ansi_style_str);
        contents.push('m');
    }

    if let Some(icon) = icon {
        contents.push(icon);
        contents.push(' ');
        width += 2;
    }

    contents.push_str(file_name);
    width += UnicodeWidthStr::width(file_name);

    if ansi_style_str.is_some() {
        contents.push_str("\x1b[0m");
    }

    DisplayCell {
        contents: contents,
        width: width,
        alignment: Alignment::Left,
    }
}
