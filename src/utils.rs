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
