#[derive(Debug, Default)]
pub struct ThemeConfig {
    inode: Option<String>,
    nlink: Option<String>,
    owner: Option<String>,
    group: Option<String>,
    size: Option<String>,
    time: Option<String>,
    read: Option<String>,
    write: Option<String>,
    execute: Option<String>,
    no_permission: Option<String>,
    #[cfg(unix)]
    setuid: Option<String>,
    #[cfg(unix)]
    setgid: Option<String>,
    #[cfg(unix)]
    sticky: Option<String>,
    #[cfg(windows)]
    archive: Option<String>,
    #[cfg(windows)]
    system: Option<String>,
    hidden: Option<String>,
}

impl ThemeConfig {
    pub fn inode_style(&self) -> Option<&str> {
        self.inode.as_deref()
    }
    pub fn nlink_style(&self) -> Option<&str> {
        self.nlink.as_deref()
    }
    pub fn owner_style(&self) -> Option<&str> {
        self.owner.as_deref()
    }
    pub fn group_style(&self) -> Option<&str> {
        self.group.as_deref()
    }
    pub fn size_style(&self) -> Option<&str> {
        self.size.as_deref()
    }
    pub fn time_style(&self) -> Option<&str> {
        self.time.as_deref()
    }
    pub fn read_style(&self) -> Option<&str> {
        self.read.as_deref()
    }
    pub fn write_style(&self) -> Option<&str> {
        self.write.as_deref()
    }
    pub fn execute_style(&self) -> Option<&str> {
        self.execute.as_deref()
    }
    pub fn no_permission_style(&self) -> Option<&str> {
        self.no_permission.as_deref()
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
    pub fn sticky_style(&self) -> Option<&str> {
        self.sticky.as_deref()
    }
    #[cfg(windows)]
    pub fn archive_style(&self) -> Option<&str> {
        self.archive.as_deref()
    }
    #[cfg(windows)]
    pub fn system_style(&self) -> Option<&str> {
        self.system.as_deref()
    }
    pub fn hidden_style(&self) -> Option<&str> {
        self.hidden.as_deref()
    }
    pub fn with_default_colors() -> Self {
        Self {
            inode: Some(String::from("32;1")),
            nlink: Some(String::from("36;1")),
            owner: Some(String::from("31")),
            group: Some(String::from("35")),
            size: Some(String::from("36")),
            time: Some(String::from("33")),
            read: Some(String::from("33;1")),
            write: Some(String::from("31;1")),
            execute: Some(String::from("32;1")),
            no_permission: Some(String::from("37;1;2")),
            #[cfg(unix)]
            setuid: Some(String::from("35;1")),
            #[cfg(unix)]
            setgid: Some(String::from("35;1")),
            #[cfg(unix)]
            sticky: Some(String::from("35;1")),
            #[cfg(windows)]
            archive: Some(String::from("31")),
            #[cfg(windows)]
            system: Some(String::from("40;33;01")),
            hidden: Some(String::from("35")),
        }
    }
}

#[derive(Debug, Default)]
pub struct IconTheme {
    file: Option<char>,
    dir: Option<char>,
    symlink: Option<char>,
    #[cfg(unix)]
    block_device: Option<char>,
    #[cfg(unix)]
    char_device: Option<char>,
    #[cfg(unix)]
    fifo: Option<char>,
    #[cfg(unix)]
    socket: Option<char>,
}

impl IconTheme {
    const COMPRESSED: Option<char> = Some('\u{f410}');
    const FONT: Option<char> = Some('\u{e659}');
    const GIT: Option<char> = Some('\u{e702}');
    const IMAGE: Option<char> = Some('\u{f1c5}');
    const PYTHON: Option<char> = Some('\u{e73c}');
    const RUST: Option<char> = Some('\u{e7a8}');
    const SHELL: Option<char> = Some('\u{ebca}');
    const VIM: Option<char> = Some('\u{e7c5}');

    pub fn with_default_icons() -> Self {
        Self {
            file: Some('\u{f4a5}'),
            dir: Some('\u{f4d3}'),
            symlink: Some('\u{f481}'),
            #[cfg(unix)]
            block_device: Some('\u{f129f}'),
            #[cfg(unix)]
            char_device: Some('\u{f065c}'),
            #[cfg(unix)]
            fifo: Some('|'),
            #[cfg(unix)]
            socket: Some('='),
        }
    }

    pub fn file_icon(&self, file_name: &str, extension: &str) -> Option<char> {
        if self.file.is_none() {
            None
        } else {
            match file_name {
                ".bash_aliases" => Self::SHELL,
                ".bash_history" => Self::SHELL,
                ".bash_login" => Self::SHELL,
                ".bash_logout" => Self::SHELL,
                ".bash_profile" => Self::SHELL,
                ".bashrc" => Self::SHELL,
                "Cargo.lock" => Self::RUST,
                "Cargo.toml" => Self::RUST,
                "Cargo.toml.orig" => Self::RUST,
                ".gitattributes" => Self::GIT,
                ".gitconfig" => Self::GIT,
                ".gitignore" => Self::GIT,
                ".gitmodules" => Self::GIT,
                ".login" => Self::SHELL,
                ".logout" => Self::SHELL,
                "profile" => Self::SHELL,
                ".profile" => Self::SHELL,
                "requirements.txt" => Self::PYTHON,
                ".vimrc" => Self::VIM,
                "_vimrc" => Self::VIM,
                ".zlogin" => Self::SHELL,
                ".zlogout" => Self::SHELL,
                ".zprofile" => Self::SHELL,
                ".zshenv" => Self::SHELL,
                ".zshrc" => Self::SHELL,
                ".zsh_history" => Self::SHELL,
                ".zsh_sessions" => Self::SHELL,
                _ => self.file_icon_by_extension(extension),
            }
        }
    }

    fn file_icon_by_extension(&self, extension: &str) -> Option<char> {
        if extension.is_empty() {
            self.file
        } else {
            match extension {
                "7z" => Self::COMPRESSED,
                "bash" => Self::SHELL,
                "bz2" => Self::COMPRESSED,
                "c" => Some('\u{e649}'),
                "cpp" => Some('\u{e646}'),
                "css" => Some('\u{e749}'),
                "gz" => Self::COMPRESSED,
                "html" => Some('\u{e736}'),
                "json" => Some('\u{e60b}'),
                "jpeg" => Self::IMAGE,
                "jpg" => Self::IMAGE,
                "js" => Some('\u{e74e}'),
                "lock" => Some('\u{e672}'),
                "lua" => Some('\u{e620}'),
                "md" => Some('\u{e73e}'),
                "mp3" => Some('\u{f001}'),
                "mp4" => Some('\u{f03d}'),
                "otf" => Self::FONT,
                "pdf" => Some('\u{f1c1}'),
                "png" => Self::IMAGE,
                "py" => Self::PYTHON,
                "rs" => Self::RUST,
                "sh" => Self::SHELL,
                "ttf" => Self::FONT,
                "vim" => Self::VIM,
                "xz" => Self::COMPRESSED,
                "zip" => Self::COMPRESSED,
                "zsh" => Self::SHELL,
                _ => self.file,
            }
        }
    }

    pub fn dir_icon(&self, file_name: &str) -> Option<char> {
        if self.dir.is_none() {
            None
        } else {
            match file_name {
                "bash-completion" => Self::SHELL,
                ".cargo" => Self::RUST,
                "fonts" => Self::FONT,
                ".git" => Some('\u{e5fb}'),
                ".github" => Some('\u{e65b}'),
                "__pycache__" => Self::PYTHON,
                ".rustup" => Self::RUST,
                ".venv" => Self::PYTHON,
                "vim" => Self::VIM,
                ".vim" => Self::VIM,
                "zsh" => Self::SHELL,
                _ => self.dir,
            }
        }
    }

    pub fn symlink_icon(&self) -> Option<char> {
        self.symlink
    }

    #[cfg(unix)]
    pub fn block_device_icon(&self) -> Option<char> {
        self.block_device
    }

    #[cfg(unix)]
    pub fn char_device_icon(&self) -> Option<char> {
        self.char_device
    }

    #[cfg(unix)]
    pub fn fifo_icon(&self) -> Option<char> {
        self.fifo
    }

    #[cfg(unix)]
    pub fn socket_icon(&self) -> Option<char> {
        self.socket
    }
}
