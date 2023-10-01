use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub struct LsColors {
    file: Option<String>,
    dir: Option<String>,
    symlink: Option<String>,
    #[cfg(unix)]
    block_device: Option<String>,
    #[cfg(unix)]
    char_device: Option<String>,
    #[cfg(unix)]
    fifo: Option<String>,
    #[cfg(unix)]
    socket: Option<String>,
    #[cfg(unix)]
    setuid: Option<String>,
    #[cfg(unix)]
    setgid: Option<String>,
    #[cfg(unix)]
    multiple_hard_links: Option<String>,
    #[cfg(unix)]
    dir_sticky_and_other_writable: Option<String>,
    #[cfg(unix)]
    dir_other_writeable: Option<String>,
    #[cfg(unix)]
    dir_sticky: Option<String>,
    exec: Option<String>,
    extension: HashMap<String, String>,
}

impl LsColors {
    pub fn init(&mut self) {
        if let Ok(ls_colors_string) = std::env::var("LS_COLORS") {
            self.parse(ls_colors_string);
        }
    }

    fn parse(&mut self, ls_colors_string: String) {
        for s in ls_colors_string.split(':') {
            if let Some((k, v)) = s.split_once('=') {
                match k {
                    "fi" => {
                        self.file = Some(v.to_string());
                    }
                    "di" => {
                        self.dir = Some(v.to_string());
                    }
                    "ln" => {
                        self.symlink = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "bd" => {
                        self.block_device = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "cd" => {
                        self.char_device = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "pi" => {
                        self.fifo = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "so" => {
                        self.socket = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "su" => {
                        self.setuid = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "sg" => {
                        self.setgid = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "mg" => {
                        self.multiple_hard_links = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "tw" => {
                        self.dir_sticky_and_other_writable = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "ow" => {
                        self.dir_other_writeable = Some(v.to_string());
                    }
                    #[cfg(unix)]
                    "st" => {
                        self.dir_sticky = Some(v.to_string());
                    }
                    "ex" => {
                        self.exec = Some(v.to_string());
                    }
                    _ => {
                        if k.starts_with("*.") {
                            self.extension
                                .insert(k.trim_start_matches("*.").to_string(), v.to_string());
                        }
                    }
                }
            }
        }
    }

    pub fn file_style(&self) -> Option<&str> {
        self.file.as_deref()
    }
    pub fn dir_style(&self) -> Option<&str> {
        self.dir.as_deref()
    }
    pub fn symlink_style(&self) -> Option<&str> {
        self.symlink.as_deref()
    }
    #[cfg(unix)]
    pub fn block_device_style(&self) -> Option<&str> {
        self.block_device.as_deref()
    }
    #[cfg(unix)]
    pub fn char_device_style(&self) -> Option<&str> {
        self.char_device.as_deref()
    }
    #[cfg(unix)]
    pub fn fifo_style(&self) -> Option<&str> {
        self.fifo.as_deref()
    }
    #[cfg(unix)]
    pub fn socket_style(&self) -> Option<&str> {
        self.socket.as_deref()
    }
    #[cfg(unix)]
    pub fn setuid_style(&self) -> Option<&str> {
        self.setuid.as_deref()
    }
    #[cfg(unix)]
    pub fn setgid_style(&self) -> Option<&str> {
        self.setgid.as_deref()
    }
    #[cfg(unix)]
    pub fn multiple_hard_links_style(&self) -> Option<&str> {
        self.multiple_hard_links.as_deref()
    }
    #[cfg(unix)]
    pub fn dir_sticky_and_other_writable_style(&self) -> Option<&str> {
        self.dir_sticky_and_other_writable.as_deref()
    }
    #[cfg(unix)]
    pub fn dir_other_writeable_style(&self) -> Option<&str> {
        self.dir_other_writeable.as_deref()
    }
    #[cfg(unix)]
    pub fn dir_sticky_style(&self) -> Option<&str> {
        self.dir_sticky.as_deref()
    }
    pub fn exec_style(&self) -> Option<&str> {
        self.exec.as_deref()
    }
    pub fn extension_style(&self, extension: String) -> Option<&str> {
        if self.extension.is_empty() {
            self.file_style()
        } else {
            self.extension
                .get(&extension)
                .map(|ansi_str_style| Some(ansi_str_style.as_str()))
                .unwrap_or(self.file_style())
        }
    }
}

pub fn get_file_extension(file_name: &str) -> String {
    match file_name.rsplit_once('.') {
        Some((file_name_without_extension, extension)) => {
            if file_name_without_extension.is_empty() {
                String::default()
            } else {
                extension.to_string()
            }
        }
        None => String::default(),
    }
}

impl Default for LsColors {
    fn default() -> Self {
        Self {
            file: None,
            dir: None,
            symlink: None,
            #[cfg(unix)]
            block_device: None,
            #[cfg(unix)]
            char_device: None,
            #[cfg(unix)]
            fifo: None,
            #[cfg(unix)]
            socket: None,
            #[cfg(unix)]
            setuid: None,
            #[cfg(unix)]
            setgid: None,
            #[cfg(unix)]
            multiple_hard_links: None,
            #[cfg(unix)]
            dir_sticky_and_other_writable: None,
            #[cfg(unix)]
            dir_other_writeable: None,
            #[cfg(unix)]
            dir_sticky: None,
            exec: None,
            extension: HashMap::with_capacity(128),
        }
    }
}
