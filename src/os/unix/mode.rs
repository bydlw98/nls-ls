use super::sys_prelude::*;

use crate::output::DisplayCell;
use crate::utils::HasMaskSetExt;

pub fn rwx_mode_cell(st_mode: u32) -> DisplayCell {
    let mut cell = DisplayCell::with_capacity(10);

    match st_mode & c::S_IFMT {
        c::S_IFREG => cell.push_char('-'),
        c::S_IFDIR => cell.push_char('d'),
        c::S_IFLNK => cell.push_char('l'),
        c::S_IFBLK => cell.push_char('b'),
        c::S_IFCHR => cell.push_char('c'),
        c::S_IFIFO => cell.push_char('p'),
        c::S_IFSOCK => cell.push_char('s'),
        _ => cell.push_char('?'),
    }

    if st_mode.has_mask_set(c::S_IRUSR) {
        cell.push_char('r');
    } else {
        cell.push_char('-');
    };

    if st_mode.has_mask_set(c::S_IWUSR) {
        cell.push_char('w');
    } else {
        cell.push_char('-');
    };

    match (st_mode.has_mask_set(c::S_IXUSR), st_mode.has_mask_set(c::S_ISUID)) {
        (false, false) => cell.push_char('-'),
        (true, false) => cell.push_char('x'),
        (false, true) => cell.push_char('S'),
        _ => cell.push_char('s'),
    }

    if st_mode.has_mask_set(c::S_IRGRP) {
        cell.push_char('r');
    } else {
        cell.push_char('-');
    };

    if st_mode.has_mask_set(c::S_IWGRP) {
        cell.push_char('w');
    } else {
        cell.push_char('-');
    };

    match (st_mode.has_mask_set(c::S_IXGRP), st_mode.has_mask_set(c::S_ISGID)) {
        (false, false) => cell.push_char('-'),
        (true, false) => cell.push_char('x'),
        (false, true) => cell.push_char('S'),
        _ => cell.push_char('s'),
    }

    if st_mode.has_mask_set(c::S_IROTH) {
        cell.push_char('r');
    } else {
        cell.push_char('-');
    };

    if st_mode.has_mask_set(c::S_IWOTH) {
        cell.push_char('w');
    } else {
        cell.push_char('-');
    };

    match (st_mode.has_mask_set(c::S_IXOTH), st_mode.has_mask_set(c::S_ISVTX)) {
        (false, false) => cell.push_char('-'),
        (true, false) => cell.push_char('x'),
        (false, true) => cell.push_char('T'),
        _ => cell.push_char('t'),
    }

    cell
}
