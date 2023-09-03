use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub struct LsColors {
    pub file: String,
    pub dir: String,
    pub symlink: String,
    pub block_device: String,
    pub char_device: String,
    pub fifo: String,
    pub socket: String,
    pub setuid: String,
    pub setgid: String,
    pub multiple_hard_links: String,
    pub dir_sticky_and_other_writable: String,
    pub dir_other_writeable: String,
    pub dir_sticky: String,
    pub exec: String,
    pub extension: HashMap<String, String>,
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
                        self.file = v.to_string();
                    }
                    "di" => {
                        self.dir = v.to_string();
                    }
                    "ln" => {
                        self.symlink = v.to_string();
                    }
                    "bd" => {
                        self.block_device = v.to_string();
                    }
                    "cd" => {
                        self.char_device = v.to_string();
                    }
                    "pi" => {
                        self.fifo = v.to_string();
                    }
                    "so" => {
                        self.socket = v.to_string();
                    }
                    "su" => {
                        self.setuid = v.to_string();
                    }
                    "sg" => {
                        self.setgid = v.to_string();
                    }
                    "mg" => {
                        self.multiple_hard_links = v.to_string();
                    }
                    "tw" => {
                        self.dir_sticky_and_other_writable = v.to_string();
                    }
                    "ow" => {
                        self.dir_other_writeable = v.to_string();
                    }
                    "st" => {
                        self.dir_sticky = v.to_string();
                    }
                    "ex" => {
                        self.exec = v.to_string();
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
}

impl Default for LsColors {
    fn default() -> Self {
        Self {
            file: String::from("0"),
            dir: String::from("0"),
            symlink: String::from("0"),
            block_device: String::from("0"),
            char_device: String::from("0"),
            fifo: String::from("0"),
            socket: String::from("0"),
            setuid: String::from("0"),
            setgid: String::from("0"),
            multiple_hard_links: String::from("0"),
            dir_sticky_and_other_writable: String::from("0"),
            dir_other_writeable: String::from("0"),
            dir_sticky: String::from("0"),
            exec: String::from("0"),
            extension: HashMap::default(),
        }
    }
}
