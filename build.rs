use std::fs;
use std::path::PathBuf;

use clap::builder::PossibleValue;
use clap::*;

fn main() {
    let out_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("OUT_DIR environment variable does not exist")
        .into();

    let mut cmd = build_command();

    fs::write(out_dir.join("help-page.txt"), cmd.render_help().to_string())
        .expect("Failed to generate help page");
}

fn build_command() -> Command {
    command!()
        .bin_name("nls")
        .term_width(80)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(
            Arg::new("file")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf))
                .value_name("FILE")
                .help("Paths to list. List the current directory if no paths are provided"),
        )
        .arg(
            Arg::new("all")
                .action(ArgAction::SetTrue)
                .short('a')
                .long("all")
                .overrides_with("almost-all")
                .help("Include hidden entries in listing"),
        )
        .arg(
            Arg::new("almost-all")
                .action(ArgAction::SetTrue)
                .short('A')
                .long("almost-all")
                .overrides_with("all")
                .help("Like -a, --all but do not list implied . and .. entries"),
        )
        .arg(
            Arg::new("vertical")
                .action(ArgAction::SetTrue)
                .short('C')
                .overrides_with_all(["across", "long", "single-column"])
                .help("List entries top-to-bottom in a grid"),
        )
        .arg(
            Arg::new("color")
                .action(ArgAction::Set)
                .long("color")
                .value_parser(value_parser!(ColorWhen))
                .value_name("WHEN")
                .default_value("auto")
                .default_missing_value("always")
                .num_args(0..=1)
                .help("Flag to control when to use color for output"),
        )
        .arg(
            Arg::new("classify")
                .action(ArgAction::SetTrue)
                .short('F')
                .long("classify")
                .overrides_with("slash")
                .help("Append filetype indicator (either */=@|) to entry file names"),
        )
        .arg(
            Arg::new("list-owner")
                .action(ArgAction::SetFalse)
                .short('g')
                .help("Like -l but do not list the owner column"),
        )
        .arg(
            Arg::new("gitignore")
                .action(ArgAction::SetTrue)
                .long("gitignore")
                .help("Ignore files set in '.gitignore' files"),
        )
        .arg(
            Arg::new("human-readable")
                .action(ArgAction::SetTrue)
                .short('h')
                .long("human-readable")
                .overrides_with_all(["iec", "si"])
                .help("Format size using factors of 1024 like 1.0K 200M 3.0G etc"),
        )
        .arg(
            Arg::new("help")
                .action(ArgAction::Help)
                .long("help")
                .help("Print help information"),
        )
        .arg(
            Arg::new("iec")
                .action(ArgAction::SetTrue)
                .long("iec")
                .overrides_with_all(["iec", "si"])
                .help("Format size using factors of 1024 like 1.0Ki 200Mi 3.0Gi etc"),
        )
        .arg(
            Arg::new("ignore-file")
                .action(ArgAction::SetTrue)
                .long("ignore-file")
                .help("Ignore files set in '.ignore' files"),
        )
        .arg(
            Arg::new("long")
                .action(ArgAction::SetTrue)
                .short('l')
                .overrides_with_all(["across", "single-column", "vertical"])
                .help("List entries along with their metadata in long format"),
        )
        .arg(
            Arg::new("numeric-uid-gid")
                .action(ArgAction::SetTrue)
                .short('n')
                .long("numeric-uid-gid")
                .help("Like -l but list the owner and group names as their respective uid and gid"),
        )
        .arg(
            Arg::new("list-group")
                .action(ArgAction::SetFalse)
                .short('o')
                .help("Like -l but do not list the group column"),
        )
        .arg(
            Arg::new("slash")
                .action(ArgAction::SetTrue)
                .short('p')
                .overrides_with("classify")
                .help("Append filetype indicator / to directory entry file names"),
        )
        .arg(
            Arg::new("reverse")
                .action(ArgAction::SetTrue)
                .short('r')
                .long("reverse")
                .help("Reverse sorting order"),
        )
        .arg(
            Arg::new("si")
                .action(ArgAction::SetTrue)
                .long("si")
                .overrides_with_all(["human-readable", "iec"])
                .help("Format size using factors of 1000 like 1.0K 200M 3.0G etc"),
        )
        .arg(
            Arg::new("size-sort")
                .action(ArgAction::SetTrue)
                .short('S')
                .overrides_with("timestamp-sort")
                .help("Sort entries by largest size first"),
        )
        .arg(
            Arg::new("timestamp-sort")
                .action(ArgAction::SetTrue)
                .short('t')
                .overrides_with("size-sort")
                .help("Sort entries by most recent timestamp first"),
        )
        .arg(
            Arg::new("version")
                .action(ArgAction::Version)
                .long("version")
                .help("Print version information"),
        )
        .arg(
            Arg::new("across")
                .action(ArgAction::SetTrue)
                .short('x')
                .overrides_with_all(["long", "single-column", "vertical"])
                .help("List entries left-to-right in a grid"),
        )
        .arg(
            Arg::new("single-column")
                .action(ArgAction::SetTrue)
                .short('1')
                .overrides_with_all(["across", "long", "vertical"])
                .help("List one entry per line"),
        )
}

#[derive(Debug, Clone, Copy)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl ValueEnum for ColorWhen {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Always, Self::Auto, Self::Never]
    }
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Always => PossibleValue::new("always").help("Always use color for output"),
            Self::Auto => {
                PossibleValue::new("auto").help("Color for output only if stdout is a tty")
            }
            Self::Never => PossibleValue::new("never").help("Never use color for output"),
        })
    }
}
