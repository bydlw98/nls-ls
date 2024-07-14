use std::env;
use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicU8, Ordering};

use log::{Level, LevelFilter};

static LOGGER: Logger = Logger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();

    let level = match env::var_os("RUST_LOG") {
        Some(level_os) => {
            if level_os == "error" {
                LevelFilter::Error
            } else if level_os == "warn" {
                LevelFilter::Warn
            } else if level_os == "info" {
                LevelFilter::Info
            } else if level_os == "debug" {
                LevelFilter::Debug
            } else if level_os == "trace" {
                LevelFilter::Trace
            } else {
                LevelFilter::Off
            }
        }
        None => LevelFilter::Off,
    };

    log::set_max_level(level);
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        #[repr(u8)]
        enum ColorState {
            Uninit,
            No,
            Yes,
        }
        static COLOR_SAVED_STATE: AtomicU8 = AtomicU8::new(ColorState::Uninit as u8);

        if COLOR_SAVED_STATE.load(Ordering::Relaxed) == ColorState::Uninit as u8 {
            if io::stderr().is_terminal() {
                COLOR_SAVED_STATE.store(ColorState::Yes as u8, Ordering::Relaxed);
            } else {
                COLOR_SAVED_STATE.store(ColorState::No as u8, Ordering::Relaxed);
            }
        }

        let color_saved_state = COLOR_SAVED_STATE.load(Ordering::Relaxed);
        if color_saved_state == ColorState::Yes as u8 {
            match record.level() {
                Level::Error => {
                    eprintln!(
                        "\x1b[90m[\x1b[0m\x1b[31mERROR\x1b[0m {}\x1b[90m]\x1b[0m {}",
                        record.target(),
                        record.args()
                    );
                }
                Level::Warn => {
                    eprintln!(
                        "\x1b[90m[\x1b[0m\x1b[33mWARN\x1b[0m {}\x1b[90m]\x1b[0m {}",
                        record.target(),
                        record.args()
                    );
                }
                Level::Info => {
                    eprintln!(
                        "\x1b[90m[\x1b[0m\x1b[32mINFO\x1b[0m {}\x1b[90m]\x1b[0m {}",
                        record.target(),
                        record.args()
                    );
                }
                Level::Debug => {
                    eprintln!(
                        "\x1b[90m[\x1b[0m\x1b[34mDEBUG\x1b[0m {}\x1b[90m]\x1b[0m {}",
                        record.target(),
                        record.args()
                    );
                }
                Level::Trace => {
                    eprintln!(
                        "\x1b[90m[\x1b[0m\x1b[36mTRACE\x1b[0m {}\x1b[90m]\x1b[0m {}",
                        record.target(),
                        record.args()
                    );
                }
            }
        } else {
            eprintln!("[{} {}] {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}
