use std::path::Path;
use std::process;

use ignore::overrides::OverrideBuilder;
use ignore::{Walk, WalkBuilder};

use crate::config::Config;
use crate::entry::EntryBuf;
use crate::output::{output, print_total};

pub fn list_dir(path: &Path, config: &Config) {
    let mut entrybuf_vec: Vec<EntryBuf> = Vec::with_capacity(16);

    for result in walk_dir(path, config) {
        match result {
            Ok(dent) => {
                if dent.depth() != 0 {
                    entrybuf_vec.push(EntryBuf::from_direntry(dent, config));
                }
            }
            Err(err) => {
                eprintln!("nls: {}", err);
                if !err.is_partial() && err.is_io() {
                    return;
                }
            }
        }
    }

    if config.list_current_and_parent_dirs {
        entrybuf_vec.push(EntryBuf::from_named_path(".", path, config));
        let parent_path = path.join("..");
        entrybuf_vec.push(EntryBuf::from_named_path("..", &parent_path, config));
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
    let mut override_builder = OverrideBuilder::new(path);
    for ignore_glob in &config.ignore_glob_vec {
        if let Err(err) = override_builder.add(ignore_glob) {
            eprintln!("nls: error with ignore-glob '{}': {}", ignore_glob, err);
            process::exit(1);
        }
    }
    match override_builder.build() {
        Ok(overrides) => WalkBuilder::new(path)
            .hidden(config.ignore_hidden)
            .parents(config.git_ignore || config.ignore_file)
            .ignore(config.ignore_file)
            .git_exclude(config.git_ignore)
            .git_global(config.git_ignore)
            .git_ignore(config.git_ignore)
            .follow_links(config.dereference)
            .max_depth(Some(1))
            .overrides(overrides)
            .build(),
        Err(err) => {
            eprintln!("nls: unable to build override builder: {}", err);
            process::exit(1);
        }
    }
}

fn recursive_walk_dir(path: &Path, config: &Config) -> Walk {
    let mut override_builder = OverrideBuilder::new(path);
    for ignore_glob in &config.ignore_glob_vec {
        if let Err(err) = override_builder.add(ignore_glob) {
            eprintln!("nls: error with ignore-glob '{}': {}", ignore_glob, err);
            process::exit(1);
        }
    }
    match override_builder.build() {
        Ok(overrides) => WalkBuilder::new(path)
            .hidden(config.ignore_hidden)
            .parents(config.git_ignore || config.ignore_file)
            .ignore(config.ignore_file)
            .git_exclude(config.git_ignore)
            .git_global(config.git_ignore)
            .git_ignore(config.git_ignore)
            .follow_links(config.dereference)
            .max_depth(config.max_depth)
            .overrides(overrides)
            .sort_by_file_path(|path1, path2| path1.cmp(path2))
            .filter_entry(|dent| {
                dent.file_type()
                    .map(|file_type| file_type.is_dir())
                    .unwrap_or(false)
            })
            .build(),
        Err(err) => {
            eprintln!("nls: unable to build override builder: {}", err);
            process::exit(1);
        }
    }
}
