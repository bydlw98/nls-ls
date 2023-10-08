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
        match std::env::var("LS_COLORS") {
            Ok(ls_colors_string) => self.parse(ls_colors_string),
            Err(err) => {
                log::debug!(
                    "unable to get value of environment variable 'LS_COLORS': {}",
                    err
                );
                log::debug!("default LS_COLORS will be used");
                self.parse(String::from(DEFAULT_LS_COLORS));
            }
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
