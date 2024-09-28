use compact_str::CompactString;

macro_rules! theme_config_get_style_impl {
    ($field:ident, $method:ident, $comment:literal) => {
        #[doc = concat!("Returns the style used for ", stringify!($comment))]
        #[inline]
        pub fn $method(&self) -> Option<&str> {
            self.$field.as_deref()
        }
    };
}

#[derive(Debug, Default)]
pub struct ThemeConfig {
    inode: Option<CompactString>,
    nlink: Option<CompactString>,
    owner: Option<CompactString>,
    group: Option<CompactString>,
    size: Option<CompactString>,
    timestamp: Option<CompactString>,
    read: Option<CompactString>,
    write: Option<CompactString>,
    execute: Option<CompactString>,
    no_permission: Option<CompactString>,
    #[cfg(unix)]
    setuid: Option<CompactString>,
    #[cfg(unix)]
    setgid: Option<CompactString>,
    #[cfg(unix)]
    sticky: Option<CompactString>,
    #[cfg(windows)]
    archive: Option<CompactString>,
    #[cfg(windows)]
    system: Option<CompactString>,
    hidden: Option<CompactString>,
}

impl ThemeConfig {
    theme_config_get_style_impl!(inode, inode_style, "inode.");

    theme_config_get_style_impl!(nlink, nlink_style, "nlink.");

    theme_config_get_style_impl!(owner, owner_style, "owner.");

    theme_config_get_style_impl!(group, group_style, "group.");

    theme_config_get_style_impl!(size, size_style, "size.");

    theme_config_get_style_impl!(timestamp, timestamp_style, "timestamp.");

    theme_config_get_style_impl!(read, read_style, "read permission.");

    theme_config_get_style_impl!(write, write_style, "write permission.");

    theme_config_get_style_impl!(execute, execute_style, "execute permission.");

    theme_config_get_style_impl!(no_permission, no_permission_style, "no permission.");

    #[cfg(unix)]
    theme_config_get_style_impl!(setuid, setuid_style, "setuid permission.");

    #[cfg(unix)]
    theme_config_get_style_impl!(setgid, setgid_style, "setgid permission.");

    #[cfg(unix)]
    theme_config_get_style_impl!(sticky, sticky_style, "sticky permission.");

    #[cfg(windows)]
    theme_config_get_style_impl!(archive, archive_style, "archive attribute.");

    #[cfg(windows)]
    theme_config_get_style_impl!(system, system_style, "system attribute.");

    theme_config_get_style_impl!(hidden, hidden_style, "hidden attribute.");

    pub fn with_default_colors() -> Self {
        Self {
            inode: Some(CompactString::new_inline("32;1")),
            nlink: Some(CompactString::new_inline("36;1")),
            owner: Some(CompactString::new_inline("31")),
            group: Some(CompactString::new_inline("35")),
            size: Some(CompactString::new_inline("36")),
            timestamp: Some(CompactString::new_inline("33")),
            read: Some(CompactString::new_inline("33;1")),
            write: Some(CompactString::new_inline("31;1")),
            execute: Some(CompactString::new_inline("32;1")),
            no_permission: Some(CompactString::new_inline("37;1;2")),
            #[cfg(unix)]
            setuid: Some(CompactString::new_inline("35;1")),
            #[cfg(unix)]
            setgid: Some(CompactString::new_inline("35;1")),
            #[cfg(unix)]
            sticky: Some(CompactString::new_inline("35;1")),
            #[cfg(windows)]
            archive: Some(CompactString::new_inline("31")),
            #[cfg(windows)]
            system: Some(CompactString::new_inline("40;33;01")),
            hidden: Some(CompactString::new_inline("35")),
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
    const DEFAULT_FILE: Option<char> = Some('\u{f4a5}');
    const DEFAULT_DIR: Option<char> = Some('\u{f4d3}');
    const DEFAULT_SYMLINK: Option<char> = Some('\u{f481}');
    #[cfg(unix)]
    const DEFAULT_BLOCK_DEVICE: Option<char> = Some('\u{f129f}');
    #[cfg(unix)]
    const DEFAULT_CHAR_DEVICE: Option<char> = Some('\u{f065c}');
    #[cfg(unix)]
    const DEFAULT_FIFO: Option<char> = Some('|');
    #[cfg(unix)]
    const DEFAULT_SOCKET: Option<char> = Some('=');

    const BINARY: Option<char> = Some('\u{eae8}');
    const COMPRESSED: Option<char> = Some('\u{f410}');
    const CONFIG: Option<char> = Some('\u{e615}');
    const CPP: Option<char> = Some('\u{e646}');
    const CSHARP: Option<char> = Some('\u{e648}');
    const CSS: Option<char> = Some('\u{e749}');
    const EMACS: Option<char> = Some('\u{e632}');
    const FONT: Option<char> = Some('\u{e659}');
    const GIT: Option<char> = Some('\u{e702}');
    const GO: Option<char> = Some('\u{e627}');
    const HEADER: Option<char> = Some('\u{f0fd}');
    const HTML: Option<char> = Some('\u{e736}');
    const IMAGE: Option<char> = Some('\u{f1c5}');
    const MAKE: Option<char> = Some('\u{e673}');
    const JAVA: Option<char> = Some('\u{e738}');
    const JAVASCRIPT: Option<char> = Some('\u{e74e}');
    const LUA: Option<char> = Some('\u{e620}');
    const PYTHON: Option<char> = Some('\u{e73c}');
    const REACT: Option<char> = Some('\u{e7ba}');
    const RUST: Option<char> = Some('\u{e7a8}');
    const SHELL: Option<char> = Some('\u{ebca}');
    const TYPESCRIPT: Option<char> = Some('\u{e628}');
    const VIDEO: Option<char> = Some('\u{f03d}');
    const VIM: Option<char> = Some('\u{e7c5}');

    pub fn with_default_icons() -> Self {
        Self {
            file: Self::DEFAULT_FILE,
            dir: Self::DEFAULT_DIR,
            symlink: Self::DEFAULT_SYMLINK,
            #[cfg(unix)]
            block_device: Self::DEFAULT_BLOCK_DEVICE,
            #[cfg(unix)]
            char_device: Self::DEFAULT_CHAR_DEVICE,
            #[cfg(unix)]
            fifo: Self::DEFAULT_FIFO,
            #[cfg(unix)]
            socket: Self::DEFAULT_SOCKET,
        }
    }

    pub fn file_icon(&self, file_name: &str, extension: &str) -> Option<char> {
        if self.file.is_none() {
            None
        } else {
            match file_name {
                "a.out" => Self::BINARY,
                "bash.bashrc" => Self::SHELL,
                ".bash_aliases" => Self::SHELL,
                ".bash_history" => Self::SHELL,
                ".bash_login" => Self::SHELL,
                ".bash_logout" => Self::SHELL,
                ".bash_profile" => Self::SHELL,
                ".bashrc" => Self::SHELL,
                "BSDmakefile" => Self::SHELL,
                "Cargo.lock" => Self::RUST,
                "Cargo.toml" => Self::RUST,
                "Cargo.toml.orig" => Self::RUST,
                "config" => Self::CONFIG,
                ".csproj" => Self::CSHARP,
                ".gitattributes" => Self::GIT,
                ".gitconfig" => Self::GIT,
                ".gitignore" => Self::GIT,
                ".gitmodules" => Self::GIT,
                "GNUmakefile" => Self::MAKE,
                "go.mod" => Self::GO,
                "go.sum" => Self::GO,
                "go.work" => Self::GO,
                ".login" => Self::SHELL,
                ".logout" => Self::SHELL,
                "makefile" => Self::MAKE,
                "Makefile" => Self::MAKE,
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
                "a" => Self::BINARY,
                "avi" => Self::VIDEO,
                "bash" => Self::SHELL,
                "bin" => Self::BINARY,
                "bz2" => Self::COMPRESSED,
                "c" => Some('\u{e649}'),
                "cc" => Self::CPP,
                "cfg" => Self::CONFIG,
                "cjs" => Self::JAVASCRIPT,
                "cts" => Self::TYPESCRIPT,
                "class" => Self::JAVA,
                "conf" => Self::CONFIG,
                "cpp" => Self::CPP,
                "cs" => Self::CSHARP,
                "css" => Self::CSS,
                "csx" => Self::CSHARP,
                "cxx" => Self::CPP,
                "db" => Some('\u{e706}'),
                "diff" => Some('\u{e728}'),
                "dll" => Self::BINARY,
                "dylib" => Self::BINARY,
                "el" => Self::EMACS,
                "elc" => Self::EMACS,
                "flv" => Self::VIDEO,
                "go" => Self::GO,
                "gz" => Self::COMPRESSED,
                "h" => Self::HEADER,
                "h264" => Self::VIDEO,
                "hh" => Self::HEADER,
                "hpp" => Self::HEADER,
                "hxx" => Self::HEADER,
                "htm" => Self::HTML,
                "html" => Self::HTML,
                "ini" => Self::CONFIG,
                "jar" => Self::JAVA,
                "java" => Self::JAVA,
                "json" => Some('\u{e60b}'),
                "jpeg" => Self::IMAGE,
                "jpg" => Self::IMAGE,
                "js" => Self::JAVASCRIPT,
                "jsx" => Self::REACT,
                "lib" => Self::BINARY,
                "lock" => Some('\u{e672}'),
                "lua" => Self::LUA,
                "m2ts" => Self::VIDEO,
                "m2v" => Self::VIDEO,
                "m4v" => Self::VIDEO,
                "md" => Some('\u{e73e}'),
                "mjs" => Self::JAVASCRIPT,
                "mk" => Self::MAKE,
                "mkv" => Self::VIDEO,
                "mov" => Self::VIDEO,
                "mp3" => Some('\u{f001}'),
                "mp4" => Self::VIDEO,
                "mpeg" => Self::VIDEO,
                "mpg" => Self::VIDEO,
                "mts" => Self::TYPESCRIPT,
                "o" => Self::BINARY,
                "ogm" => Self::VIDEO,
                "ogv" => Self::VIDEO,
                "otf" => Self::FONT,
                "pdf" => Some('\u{f1c1}'),
                "png" => Self::IMAGE,
                "py" => Self::PYTHON,
                "pyc" => Self::PYTHON,
                "pyd" => Self::PYTHON,
                "pyi" => Self::PYTHON,
                "pyo" => Self::PYTHON,
                "pyw" => Self::PYTHON,
                "pyx" => Self::PYTHON,
                "pyz" => Self::PYTHON,
                "rs" => Self::RUST,
                "sh" => Self::SHELL,
                "shtml" => Self::HTML,
                "so" => Self::BINARY,
                "svelte" => Some('\u{e697}'),
                "sym" => Self::BINARY,
                "toml" => Self::CONFIG,
                "ts" => Self::TYPESCRIPT,
                "tsx" => Self::REACT,
                "ttf" => Self::FONT,
                "vim" => Self::VIM,
                "vob" => Self::VIDEO,
                "vue" => Some('\u{f0844}'),
                "webm" => Self::VIDEO,
                "whl" => Self::PYTHON,
                "wmv" => Self::VIDEO,
                "xhtml" => Self::HTML,
                "xz" => Self::COMPRESSED,
                "yml" => Self::CONFIG,
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
                "emacs" => Self::EMACS,
                ".emacs.d" => Self::EMACS,
                "fonts" => Self::FONT,
                ".git" => Some('\u{e5fb}'),
                ".github" => Some('\u{e65b}'),
                "__pycache__" => Self::PYTHON,
                ".rustup" => Self::RUST,
                ".venv" => Self::PYTHON,
                "vim" => Self::VIM,
                ".vim" => Self::VIM,
                "vimfiles" => Self::VIM,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_icontheme_file_icon() {
        let icons = IconTheme::default();
        assert!(icons.file_icon("Cargo.toml", "toml").is_none());
        assert!(icons.file_icon("init.lua", "lua").is_none());
        assert!(icons.file_icon("file_name", "").is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.file_icon("Cargo.toml", "toml"), IconTheme::RUST);
        assert_eq!(icons.file_icon("init.lua", "lua"), IconTheme::LUA);
        assert_eq!(icons.file_icon("file_name", ""), IconTheme::DEFAULT_FILE);
    }

    #[test]
    fn test_icontheme_dir_icon() {
        let icons = IconTheme::default();
        assert!(icons.dir_icon(".cargo").is_none());
        assert!(icons.dir_icon("dir1").is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.dir_icon(".cargo"), IconTheme::RUST);
        assert_eq!(icons.dir_icon("dir1"), IconTheme::DEFAULT_DIR);
    }

    #[test]
    fn test_icontheme_symlink_icon() {
        let icons = IconTheme::default();
        assert!(icons.symlink_icon().is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.symlink_icon(), IconTheme::DEFAULT_SYMLINK);
    }

    #[cfg(unix)]
    #[test]
    fn test_icontheme_block_device_icon() {
        let icons = IconTheme::default();
        assert!(icons.block_device_icon().is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.block_device_icon(), IconTheme::DEFAULT_BLOCK_DEVICE);
    }

    #[cfg(unix)]
    #[test]
    fn test_icontheme_char_device_icon() {
        let icons = IconTheme::default();
        assert!(icons.char_device_icon().is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.char_device_icon(), IconTheme::DEFAULT_CHAR_DEVICE);
    }

    #[cfg(unix)]
    #[test]
    fn test_icontheme_fifo_icon() {
        let icons = IconTheme::default();
        assert!(icons.fifo_icon().is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.fifo_icon(), IconTheme::DEFAULT_FIFO);
    }

    #[cfg(unix)]
    #[test]
    fn test_icontheme_socket_icon() {
        let icons = IconTheme::default();
        assert!(icons.socket_icon().is_none());

        let icons = IconTheme::with_default_icons();
        assert_eq!(icons.socket_icon(), IconTheme::DEFAULT_SOCKET);
    }
}
