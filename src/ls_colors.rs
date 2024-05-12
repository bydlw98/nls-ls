use std::collections::hash_map::HashMap;

use compact_str::{CompactString, ToCompactString};

macro_rules! ls_colors_get_style_impl {
    ($field:ident, $method:ident, $comment:literal) => {
        #[doc = concat!("Returns the style used for ", $comment)]
        #[inline]
        pub fn $method(&self) -> Option<&str> {
            self.$field.as_deref()
        }
    };
}

#[derive(Debug)]
pub struct LsColors {
    file: Option<CompactString>,
    dir: Option<CompactString>,
    symlink: Option<CompactString>,
    exec: Option<CompactString>,
    #[cfg(unix)]
    block_device: Option<CompactString>,
    #[cfg(unix)]
    char_device: Option<CompactString>,
    #[cfg(unix)]
    fifo: Option<CompactString>,
    #[cfg(unix)]
    socket: Option<CompactString>,
    #[cfg(unix)]
    setuid: Option<CompactString>,
    #[cfg(unix)]
    setgid: Option<CompactString>,
    #[cfg(unix)]
    multiple_hard_links: Option<CompactString>,
    #[cfg(unix)]
    dir_sticky_and_other_writable: Option<CompactString>,
    #[cfg(unix)]
    dir_other_writable: Option<CompactString>,
    #[cfg(unix)]
    dir_sticky: Option<CompactString>,
    extension: HashMap<CompactString, CompactString>,
}

impl LsColors {
    pub fn with_colors() -> Self {
        let mut ls_colors = Self::default();

        match std::env::var("LS_COLORS") {
            Ok(ls_colors_string) => ls_colors.parse(ls_colors_string),
            Err(err) => {
                log::debug!(
                    "unable to get value of environment variable 'LS_COLORS': {}",
                    err
                );
                log::debug!("default LS_COLORS will be used");
                ls_colors.parse(String::from(DEFAULT_LS_COLORS));
            }
        }

        ls_colors
    }

    fn parse(&mut self, ls_colors_string: String) {
        for s in ls_colors_string.split(':') {
            if let Some((k, v)) = s.split_once('=') {
                match k {
                    "fi" => {
                        self.file = Some(v.to_compact_string());
                    }
                    "di" => {
                        self.dir = Some(v.to_compact_string());
                    }
                    "ln" => {
                        self.symlink = Some(v.to_compact_string());
                    }
                    "ex" => {
                        self.exec = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "bd" => {
                        self.block_device = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "cd" => {
                        self.char_device = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "pi" => {
                        self.fifo = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "so" => {
                        self.socket = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "su" => {
                        self.setuid = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "sg" => {
                        self.setgid = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "mh" => {
                        self.multiple_hard_links = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "tw" => {
                        self.dir_sticky_and_other_writable = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "ow" => {
                        self.dir_other_writable = Some(v.to_compact_string());
                    }
                    #[cfg(unix)]
                    "st" => {
                        self.dir_sticky = Some(v.to_compact_string());
                    }
                    _ => {
                        if k.starts_with("*.") {
                            self.extension.insert(
                                k.trim_start_matches("*.").to_compact_string(),
                                v.to_compact_string(),
                            );
                        }
                    }
                }
            }
        }
    }

    ls_colors_get_style_impl!(file, file_style, "regular files.");

    ls_colors_get_style_impl!(dir, dir_style, "directories.");

    ls_colors_get_style_impl!(symlink, symlink_style, "symbolic links.");

    ls_colors_get_style_impl!(exec, exec_style, "executables.");

    #[cfg(unix)]
    ls_colors_get_style_impl!(block_device, block_device_style, "block devices.");

    #[cfg(unix)]
    ls_colors_get_style_impl!(char_device, char_device_style, "char devices.");

    #[cfg(unix)]
    ls_colors_get_style_impl!(fifo, fifo_style, "fifos.");

    #[cfg(unix)]
    ls_colors_get_style_impl!(socket, socket_style, "socket file type.");

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        setuid,
        setuid_style,
        "regular files with setuid file permission."
    );

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        setgid,
        setgid_style,
        "regular files with setgid file permission."
    );

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        multiple_hard_links,
        multiple_hard_links_style,
        "files with multiple hard links."
    );

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        dir_sticky_and_other_writable,
        dir_sticky_and_other_writable_style,
        "directories with sticky and other writable permissions."
    );

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        dir_other_writable,
        dir_other_writable_style,
        "directories with other writable permission."
    );

    #[cfg(unix)]
    ls_colors_get_style_impl!(
        dir_sticky,
        dir_sticky_style,
        "directories with sticky permission."
    );

    pub fn extension_style(&self, extension: &str) -> Option<&str> {
        if self.extension.is_empty() {
            self.file_style()
        } else {
            self.extension
                .get(extension)
                .map(|ansi_str_style| Some(ansi_str_style.as_str()))
                .unwrap_or(self.file_style())
        }
    }
}

impl Default for LsColors {
    fn default() -> Self {
        Self {
            file: None,
            dir: None,
            symlink: None,
            exec: None,
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
            dir_other_writable: None,
            #[cfg(unix)]
            dir_sticky: None,
            extension: HashMap::with_capacity(128),
        }
    }
}

pub fn get_file_extension(file_name: &str) -> CompactString {
    match file_name.rsplit_once('.') {
        Some((file_name_without_extension, extension)) => {
            if file_name_without_extension.is_empty()
                || file_name_without_extension.ends_with('/')
                || file_name_without_extension.ends_with(std::path::MAIN_SEPARATOR_STR)
            {
                CompactString::default()
            } else {
                extension.to_compact_string()
            }
        }
        None => CompactString::default(),
    }
}

const DEFAULT_LS_COLORS: &str = "rs=0:di=01;34:ln=01;36:mh=00:pi=40;33:\
                                    so=01;35:do=01;35:bd=40;33;01:\
                                    cd=40;33;01:or=40;31;01:mi=00:su=37;41:\
                                    sg=30;43:ca=30;41:tw=30;42:ow=34;42:\
                                    st=37;44:ex=01;32:*.tar=01;31:*.tgz=01;31:\
                                    *.arc=01;31:*.arj=01;31:*.taz=01;31:\
                                    *.lha=01;31:*.lz4=01;31:*.lzh=01;31:\
                                    *.lzma=01;31:*.tlz=01;31:*.txz=01;31:\
                                    *.tzo=01;31:*.t7z=01;31:*.zip=01;31:\
                                    *.z=01;31:*.dz=01;31:*.gz=01;31:\
                                    *.lrz=01;31:*.lz=01;31:*.lzo=01;31:\
                                    *.xz=01;31:*.zst=01;31:*.tzst=01;31:\
                                    *.bz2=01;31:*.bz=01;31:*.tbz=01;31:\
                                    *.tbz2=01;31:*.tz=01;31:*.deb=01;31:\
                                    *.rpm=01;31:*.jar=01;31:*.war=01;31:\
                                    *.ear=01;31:*.sar=01;31:*.rar=01;31:\
                                    *.alz=01;31:*.ace=01;31:*.zoo=01;31:\
                                    *.cpio=01;31:*.7z=01;31:*.rz=01;31:\
                                    *.cab=01;31:*.wim=01;31:*.swm=01;31:\
                                    *.dwm=01;31:*.esd=01;31:*.jpg=01;35:\
                                    *.jpeg=01;35:*.mjpg=01;35:*.mjpeg=01;35:\
                                    *.gif=01;35:*.bmp=01;35:*.pbm=01;35:\
                                    *.pgm=01;35:*.ppm=01;35:*.tga=01;35:\
                                    *.xbm=01;35:*.xpm=01;35:*.tif=01;35:\
                                    *.tiff=01;35:*.png=01;35:*.svg=01;35:\
                                    *.svgz=01;35:*.mng=01;35:*.pcx=01;35:\
                                    *.mov=01;35:*.mpg=01;35:*.mpeg=01;35:\
                                    *.m2v=01;35:*.mkv=01;35:*.webm=01;35:\
                                    *.webp=01;35:*.ogm=01;35:*.mp4=01;35:\
                                    *.m4v=01;35:*.mp4v=01;35:*.vob=01;35:\
                                    *.qt=01;35:*.nuv=01;35:*.wmv=01;35:\
                                    *.asf=01;35:*.rm=01;35:*.rmvb=01;35:\
                                    *.flc=01;35:*.avi=01;35:*.fli=01;35:\
                                    *.flv=01;35:*.gl=01;35:*.dl=01;35:\
                                    *.xcf=01;35:*.xwd=01;35:*.yuv=01;35:\
                                    *.cgm=01;35:*.emf=01;35:*.ogv=01;35:\
                                    *.ogx=01;35:*.aac=00;36:*.au=00;36:\
                                    *.flac=00;36:*.m4a=00;36:*.mid=00;36:\
                                    *.midi=00;36:*.mka=00;36:*.mp3=00;36:\
                                    *.mpc=00;36:*.ogg=00;36:*.ra=00;36:\
                                    *.wav=00;36:*.oga=00;36:*.opus=00;36:\
                                    *.spx=00;36:*.xspf=00;36:";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lscolor_non_extension_style() {
        let mut ls_colors = LsColors::default();
        ls_colors.parse(String::from(DEFAULT_LS_COLORS));

        assert_eq!(ls_colors.file_style(), None);
        assert_eq!(ls_colors.dir_style(), Some("01;34"));
        assert_eq!(ls_colors.symlink_style(), Some("01;36"));
        assert_eq!(ls_colors.exec_style(), Some("01;32"));

        #[cfg(unix)]
        assert_eq!(ls_colors.block_device_style(), Some("40;33;01"));

        #[cfg(unix)]
        assert_eq!(ls_colors.char_device_style(), Some("40;33;01"));

        #[cfg(unix)]
        assert_eq!(ls_colors.fifo_style(), Some("40;33"));

        #[cfg(unix)]
        assert_eq!(ls_colors.socket_style(), Some("01;35"));

        #[cfg(unix)]
        assert_eq!(ls_colors.setuid_style(), Some("37;41"));

        #[cfg(unix)]
        assert_eq!(ls_colors.setgid_style(), Some("30;43"));

        #[cfg(unix)]
        assert_eq!(ls_colors.multiple_hard_links_style(), Some("00"));

        #[cfg(unix)]
        assert_eq!(
            ls_colors.dir_sticky_and_other_writable_style(),
            Some("30;42")
        );

        #[cfg(unix)]
        assert_eq!(ls_colors.dir_other_writable_style(), Some("34;42"));

        #[cfg(unix)]
        assert_eq!(ls_colors.dir_sticky_style(), Some("37;44"));
    }

    #[test]
    fn test_lscolor_extension_style() {
        let mut ls_colors = LsColors::default();
        ls_colors.parse(String::from(DEFAULT_LS_COLORS));

        assert_eq!(ls_colors.extension_style("gz"), Some("01;31"));
        assert_eq!(ls_colors.extension_style(""), None);

        // "xyz" extension is not set in DEFAULT_LS_COLORS
        assert_eq!(ls_colors.extension_style("xyz"), None);
    }

    #[test]
    fn test_get_file_extension() {
        use std::path::MAIN_SEPARATOR_STR;

        assert_eq!(get_file_extension("Makefile"), "");
        assert_eq!(get_file_extension("dir1/Makefile"), "");
        assert_eq!(get_file_extension("dir1/dir2/Makefile"), "");
        assert_eq!(
            get_file_extension(&format!(
                "dir1{}dir2{}Makefile",
                MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
            )),
            ""
        );

        assert_eq!(get_file_extension(".gitignore"), "");
        assert_eq!(get_file_extension("dir1/.gitignore"), "");
        assert_eq!(
            get_file_extension(&format!(
                "dir1{}dir2{}.gitignore",
                MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
            )),
            ""
        );

        assert_eq!(get_file_extension("main.rs"), "rs");
        assert_eq!(get_file_extension("dir1/main.rs"), "rs");
        assert_eq!(
            get_file_extension(&format!(
                "dir1{}dir2{}main.rs",
                MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
            )),
            "rs"
        );
    }
}
