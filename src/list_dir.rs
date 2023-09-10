use std::path::Path;

use ignore::{Walk, WalkBuilder};

use crate::config::Config;
use crate::entry::EntryBuf;
use crate::output::{output, print_total};

pub fn list_dir(path: &Path, config: &Config) {
    let mut entrybuf_vec: Vec<EntryBuf> = Vec::with_capacity(16);
    let mut error_count: usize = 0;

    for result in walk_dir(path, config) {
        match result {
            Ok(dent) => {
                if dent.depth() == 0 {
                    continue;
                }
                entrybuf_vec.push(EntryBuf::from_direntry(dent, config));
            }
            Err(err) => {
                error_count += 1;
                eprintln!("nls: {}", err);
            }
        }
    }
    if error_count == 0 && !entrybuf_vec.is_empty() && config.show_current_and_parent_dirs {
        let current_dir_entrybuf =
            EntryBuf::from_path_with_file_name(String::from("."), path, config);
        let parent_dir_path = path.join("..");
        let parent_dir_entrybuf =
            EntryBuf::from_path_with_file_name(String::from(".."), &parent_dir_path, config);

        entrybuf_vec.insert(0, current_dir_entrybuf);
        entrybuf_vec.insert(1, parent_dir_entrybuf);
    }

    if config.output_format.is_long() || config.list_allocated_size {
        print_total(&entrybuf_vec, config);
    }

    output(&mut entrybuf_vec, config);
}

pub fn recursive_list_dir(path: &Path, config: &Config) {
    for result in recursive_walk_dir(path, config) {
        match result {
            Ok(dent) => {
                if dent.depth() != 0 {
                    println!("\n{}:", dent.path().display());
                }

                list_dir(dent.path(), config);
            }
            Err(err) => {
                eprintln!("nls: {}", err);
            }
        }
    }
}

fn walk_dir(path: &Path, config: &Config) -> Walk {
    WalkBuilder::new(path)
        .hidden(config.ignore_hidden)
        .parents(config.git_ignore)
        .ignore(config.ignore_file)
        .git_exclude(config.git_ignore)
        .git_global(config.git_ignore)
        .git_ignore(config.git_ignore)
        .max_depth(Some(1))
        .build()
}

fn recursive_walk_dir(path: &Path, config: &Config) -> Walk {
    WalkBuilder::new(path)
        .hidden(config.ignore_hidden)
        .parents(config.git_ignore)
        .ignore(config.ignore_file)
        .git_exclude(config.git_ignore)
        .git_global(config.git_ignore)
        .git_ignore(config.git_ignore)
        .max_depth(config.max_depth)
        .sort_by_file_path(|path1, path2| path1.cmp(path2))
        .filter_entry(|dent| {
            dent.file_type()
                .map(|file_type| file_type.is_dir())
                .unwrap_or(false)
        })
        .build()
}
