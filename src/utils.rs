pub fn terminal_width() -> Option<usize> {
    let (terminal_size::Width(term_width), _) = terminal_size::terminal_size()?;

    Some(term_width as usize)
}
