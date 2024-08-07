#![allow(clippy::redundant_field_names)]

mod config;
mod entry;
mod list_dir;
mod logger;
mod ls_colors;
mod os;
mod output;
mod theme;
mod utils;

use config::Config;
use entry::EntryBuf;
use std::path::{Path, PathBuf};

fn main() {
    logger::init();

    let (config, path_args_vec) = Config::init();
    log::debug!("{:?}", config);
    log::debug!("{:?}", path_args_vec);

    let num_path_args = path_args_vec.len();

    if num_path_args == 0 {
        zero_path_args(&config);
    } else if num_path_args == 1 {
        one_path_arg(&path_args_vec[0], &config);
    } else {
        multiple_path_args(path_args_vec, &config);
    }
}

fn zero_path_args(config: &Config) {
    if !config.list_dir {
        let entrybuf = EntryBuf::from_cmdline_path(Path::new("."), config);
        let mut entrybuf_vec = vec![entrybuf];

        if config.list_allocated_size {
            output::print_total(&entrybuf_vec, config);
        }
        output::output(&mut entrybuf_vec, config);
    } else if config.recursive {
        list_dir::recursive_list_dir(Path::new("."), config);
    } else {
        let _ = list_dir::list_dir(Path::new("."), config);
    }
}

fn one_path_arg(path: &Path, config: &Config) {
    let metadata_result = if config.dereference_cmdline_symlink_dir {
        path.metadata()
    } else {
        path.symlink_metadata()
    };

    match metadata_result {
        Ok(metadata) => {
            if metadata.is_dir() && config.list_dir {
                if config.recursive {
                    list_dir::recursive_list_dir(path, config);
                } else {
                    let _ = list_dir::list_dir(path, config);
                }
            } else {
                let entrybuf = EntryBuf::from_cmdline_path(path, config);
                let mut entrybuf_vec = vec![entrybuf];

                if config.list_allocated_size {
                    output::print_total(&entrybuf_vec, config);
                }
                output::output(&mut entrybuf_vec, config);
            }
        }
        Err(err) => {
            eprintln!("nls: unable to access '{}': {}", path.display(), err);
        }
    }
}

fn multiple_path_args(path_args_vec: Vec<PathBuf>, config: &Config) {
    let (list_non_dir_paths_vec, list_dir_paths_vec) = split_path_args_vec(path_args_vec, config);
    let list_non_dir_paths_vec_is_empty = list_non_dir_paths_vec.is_empty();

    if !list_non_dir_paths_vec.is_empty() {
        let mut entrybuf_vec: Vec<EntryBuf> = Vec::with_capacity(list_non_dir_paths_vec.len());
        for path in list_non_dir_paths_vec {
            entrybuf_vec.push(EntryBuf::from_cmdline_path(&path, config));
        }

        if config.list_allocated_size {
            output::print_total(&entrybuf_vec, config);
        }
        output::output(&mut entrybuf_vec, config);
    }

    if !list_dir_paths_vec.is_empty() {
        if list_non_dir_paths_vec_is_empty {
            println!("{}:", &list_dir_paths_vec[0].display());
        } else {
            println!("\n{}:", &list_dir_paths_vec[0].display());
        }

        if config.recursive {
            list_dir::recursive_list_dir(&list_dir_paths_vec[0], config);

            let remainding_dir_paths_vec = &list_dir_paths_vec[1..];
            for path in remainding_dir_paths_vec {
                println!("\n{}:", path.display());
                list_dir::recursive_list_dir(path, config);
            }
        } else {
            let _ = list_dir::list_dir(&list_dir_paths_vec[0], config);

            let remainding_dir_paths_vec = &list_dir_paths_vec[1..];
            for path in remainding_dir_paths_vec {
                println!("\n{}:", path.display());
                let _ = list_dir::list_dir(path, config);
            }
        }
    }
}

fn split_path_args_vec(
    path_args_vec: Vec<PathBuf>,
    config: &Config,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let num_path_args = path_args_vec.len();
    let mut list_non_dir_paths_vec: Vec<PathBuf> = Vec::with_capacity(num_path_args);
    let mut list_dir_paths_vec: Vec<PathBuf> = Vec::with_capacity(num_path_args);

    for path in &path_args_vec {
        inner_split_path_args_vec(
            path,
            &mut list_dir_paths_vec,
            &mut list_non_dir_paths_vec,
            config,
        );
    }

    (list_non_dir_paths_vec, list_dir_paths_vec)
}

fn inner_split_path_args_vec(
    path: &Path,
    list_dir_paths_vec: &mut Vec<PathBuf>,
    list_non_dir_paths_vec: &mut Vec<PathBuf>,
    config: &Config,
) {
    let metadata_result = if config.dereference_cmdline_symlink_dir {
        path.metadata()
    } else {
        path.symlink_metadata()
    };

    match metadata_result {
        Ok(metadata) => {
            if metadata.is_dir() && config.list_dir {
                list_dir_paths_vec.push(path.to_path_buf());
            } else {
                list_non_dir_paths_vec.push(path.to_path_buf());
            }
        }
        Err(err) => {
            eprintln!("nls: unable to access '{}': {}", path.display(), err);
        }
    }
}
