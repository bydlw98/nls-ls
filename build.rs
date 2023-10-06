use std::env;
use std::fs;
use std::path::PathBuf;

use clap::builder::PossibleValue;
use clap::*;
use clap_complete::generate_to;
use clap_complete::shells::*;

fn main() {
    let out_dir: PathBuf = env::var("OUT_DIR")
        .expect("OUT_DIR environment variable does not exist")
        .into();

    let mut cmd = build_command();

    fs::write(out_dir.join("help-page.txt"), cmd.render_help().to_string())
        .expect("Failed to generate help page");

    generate_completions();
    generate_manpage();
}

fn generate_completions() {
    let mut cmd = build_command();
    let bin_name = "nls";
    let completions_dir: PathBuf = PathBuf::from("./completions");
    if !completions_dir.exists() {
        fs::create_dir(&completions_dir).expect("Unable to create completion dir");
    }

    generate_to(Bash, &mut cmd, bin_name, &completions_dir)
        .expect("Failed to generate Bash completions");
    generate_to(Fish, &mut cmd, bin_name, &completions_dir)
        .expect("Failed to generate Fish completions");
    generate_to(Zsh, &mut cmd, bin_name, &completions_dir)
        .expect("Failed to generate Zsh completions");
}

fn generate_manpage() {
    let doc_dir: PathBuf = PathBuf::from("./doc");
    if !doc_dir.exists() {
        fs::create_dir(&doc_dir).expect("Unable to create doc dir");
    }
    let cmd = build_command();

    let man = clap_mangen::Man::new(cmd).date("2023-10-06");
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).expect("Unable to render man page to buffer");
    fs::write(doc_dir.join("nls.1"), buffer).expect("Unable to create man page nls.1");
}

fn build_command() -> Command {
    command!()
        .name("nls")
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
            Arg::new("allocated-bytes")
                .action(ArgAction::SetTrue)
                .long("allocated-bytes")
                .help("Use raw allocated size"),
        )
        .arg(
            Arg::new("changed")
                .action(ArgAction::SetTrue)
                .short('c')
                .overrides_with_all(["accessed", "time"])
                .help("Use timestamp of when entry status was last changed"),
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
                .value_parser([
                    PossibleValue::new("always").help("Always use color for output"),
                    PossibleValue::new("auto").help("Color for output only if stdout is a tty"),
                    PossibleValue::new("never").help("Never use color for output"),
                ])
                .value_name("WHEN")
                .default_missing_value("always")
                .num_args(0..=1)
                .help("Flag to control when to use color for output"),
        )
        .arg(
            Arg::new("directory")
                .action(ArgAction::SetTrue)
                .short('d')
                .long("directory")
                .help("Do not list directory contents, instead list the directory itself"),
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
            Arg::new("dereference-command-line")
                .action(ArgAction::SetTrue)
                .short('H')
                .long("dereference-command-line")
                .overrides_with("dereference")
                .help("Use target information of symlinks passed as arguments on command line")
        )
        .arg(
            Arg::new("help")
                .action(ArgAction::Help)
                .long("help")
                .help("Print help information"),
        )
        .arg(
            Arg::new("list-inode")
                .action(ArgAction::SetTrue)
                .short('i')
                .long("inode")
                .help("List entries along with their file inode number"),
        )
        .arg(
            Arg::new("ignore-glob")
                .action(ArgAction::Append)
                .short('I')
                .long("ignore-glob")
                .value_parser(value_parser!(String))
                .value_name("PATTERN")
                .help("Ignore entries matching glob pattern")
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
            Arg::new("kibibytes")
                .action(ArgAction::SetTrue)
                .short('k')
                .long("kibibytes")
                .help("Use 1024 byte blocks for allocated size"),
        )
        .arg(
            Arg::new("long")
                .action(ArgAction::SetTrue)
                .short('l')
                .overrides_with_all(["across", "single-column", "vertical"])
                .help("List entries along with their metadata in long format"),
        )
        .arg(
            Arg::new("dereference")
                .action(ArgAction::SetTrue)
                .short('L')
                .long("dereference")
                .overrides_with("dereference-command-line")
                .help("Use target information when listing symlink entries")
        )
        .arg(
            Arg::new("max-depth")
                .action(ArgAction::Set)
                .long("max-depth")
                .value_parser(value_parser!(usize))
                .value_name("NUM")
                .help("Set the max depth to recurse into"),
        )
        .arg(
            Arg::new("mode")
                .action(ArgAction::Set)
                .long("mode")
                .value_parser([
                    PossibleValue::new("native").help("Use the platform default mode format"),
                    PossibleValue::new("pwsh").help("Use powershell mode format e.g. 'darhsl'. This is the default on windows"),
                    PossibleValue::new("rwx").help("Use symbolic format e.g. 'drwxrwxrwx'. This is the default on unix like platforms"),
                ])
                .value_name("WORD")
                .help("Set the mode format to be used in long format"),
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
            Arg::new("recursive")
                .action(ArgAction::SetTrue)
                .short('R')
                .long("recursive")
                .help("Recurse into directories"),
        )
        .arg(
            Arg::new("list-allocated-size")
                .action(ArgAction::SetTrue)
                .short('s')
                .long("size")
                .help("List entries along with their allocated size"),
        )
        .arg(
            Arg::new("size-sort")
                .action(ArgAction::SetTrue)
                .short('S')
                .overrides_with("timestamp-sort")
                .help("Sort entries by largest size first"),
        )
        .arg(
            Arg::new("si")
                .action(ArgAction::SetTrue)
                .long("si")
                .overrides_with_all(["human-readable", "iec"])
                .help("Format size using factors of 1000 like 1.0K 200M 3.0G etc"),
        )
        .arg(
            Arg::new("timestamp-sort")
                .action(ArgAction::SetTrue)
                .short('t')
                .overrides_with("size-sort")
                .help("Sort entries by most recent timestamp first"),
        )
        .arg(
            Arg::new("time")
                .action(ArgAction::Set)
                .long("time")
                .value_parser([
                    PossibleValue::new("accessed")
                        .help("Use timestamp of when entry was last accessed (-u)"),
                    PossibleValue::new("changed")
                        .help("Use timestamp of when entry status was last changed (-c)"),
                    PossibleValue::new("created").help("Use timestamp of when entry was created"),
                    PossibleValue::new("modified")
                        .help("Use timestamp of when entry was last modified"),
                    PossibleValue::new("atime").help("Alias to 'accessed'"),
                    PossibleValue::new("ctime").help("Alias to 'changed'"),
                    PossibleValue::new("btime").help("Alias to 'created'"),
                    PossibleValue::new("mtime").help("Alias to 'modified'"),
                ])
                .value_name("WORD")
                .overrides_with_all(["accessed", "changed"])
                .help(
                    "Set timestamp to use for sorting by timestamp or/and listing in long format",
                ),
        )
        .arg(
            Arg::new("accessed")
                .action(ArgAction::SetTrue)
                .short('u')
                .overrides_with_all(["changed", "time"])
                .help("Use timestamp of when entry was last accessed"),
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
