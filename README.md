# nls-ls (nls)

[![CICD](https://github.com/bydlw98/nls-ls/actions/workflows/CICD.yml/badge.svg)](https://github.com/bydlw98/nls-ls/actions/workflows/CICD.yml)
[![Crates.io](https://img.shields.io/crates/v/nls-ls)](https://crates.io/crates/nls-ls)

Yet another ls(1) implementation

![screenshot](https://github.com/bydlw98/nls-ls/assets/108581910/8d37adbe-929e-4c02-95aa-fcf996059bfb)

## Features

* Color support for all columns
* Ignore files set in '.gitignore' files (through '--gitignore' flag)
* Cross-platform support (Linux and Windows)

## Command-line options
```
Usage: nls [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Paths to list. List the current directory if no paths are provided

Options:
  -a, --all
          Include hidden entries in listing
  -A, --almost-all
          Like -a, --all but do not list implied . and .. entries
      --allocated-bytes
          Use raw allocated size
  -c
          Use timestamp of when entry status was last changed
  -C
          List entries top-to-bottom in a grid
      --color [<WHEN>]
          Flag to control when to use color for output [possible values: always,
          auto, never]
  -d, --directory
          Do not list directory contents, instead list the directory itself
  -F, --classify
          Append filetype indicator (either */=@|) to entry file names
  -g
          Like -l but do not list the owner column
      --gitignore
          Ignore files set in '.gitignore' files
  -h, --human-readable
          Format size using factors of 1024 like 1.0K 200M 3.0G etc
  -H, --dereference-command-line
          Use target information of symlinks passed as arguments on command line
      --help
          Print help information
  -i, --inode
          List entries along with their file inode number
  -I, --ignore-glob <PATTERN>
          Ignore entries matching glob pattern
      --iec
          Format size using factors of 1024 like 1.0Ki 200Mi 3.0Gi etc
      --ignore-file
          Ignore files set in '.ignore' files
  -k, --kibibytes
          Use 1024 byte blocks for allocated size
  -l
          List entries along with their metadata in long format
  -L, --dereference
          Use target information when listing symlink entries
      --max-depth <NUM>
          Set the max depth to recurse into
      --mode <WORD>
          Set the mode format to be used in long format [possible values:
          native, pwsh, rwx]
  -n, --numeric-uid-gid
          Like -l but list the owner and group names as their respective uid and
          gid
  -o
          Like -l but do not list the group column
  -p
          Append filetype indicator / to directory entry file names
  -r, --reverse
          Reverse sorting order
  -R, --recursive
          Recurse into directories
  -s, --size
          List entries along with their allocated size
  -S
          Sort entries by largest size first
      --si
          Format size using factors of 1000 like 1.0K 200M 3.0G etc
  -t
          Sort entries by most recent timestamp first
      --time <WORD>
          Set timestamp to use for sorting by timestamp or/and listing in long
          format [possible values: accessed, changed, created, modified, atime,
          ctime, btime, mtime]
  -u
          Use timestamp of when entry was last accessed
      --version
          Print version information
  -x
          List entries left-to-right in a grid
  -1
          List one entry per line
```

## Installation

### From source

Using Rust's package manager cargo
```
cargo install nls-ls
```

A Makefile is also provided to build and install the binary, shell completions and man page
```
make
sudo make install
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
