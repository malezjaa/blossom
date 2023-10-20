#![allow(dead_code)]
#![allow(unused_imports)]

use colored::Colorize;
use crate::cli::cli;

mod utils {
    pub mod logger;
    pub mod types;
    pub mod errors;
    pub mod version;
    pub mod files;
    pub mod scripts;
}

mod commands {
    pub mod init;
    pub mod install;
    pub mod cache;
    pub mod run;
    pub mod start;
}

mod structs {
    pub mod http;
    pub mod package;
    pub mod cache;
    pub mod installer;
    pub mod lockfile;
}

use utils::{logger, types, errors, version, files, scripts};
use structs::http::Requester;
use structs::package::Package;
use structs::cache::Cache;
use structs::installer::Installer;
use structs::lockfile::LockFile;
use crate::commands::init::init_command;
use crate::commands::install::install_command;
use crate::commands::cache::cache_command;
use crate::commands::run::run_command;
use crate::commands::start::start_command;
use crate::utils::files::symlink_pkg;
use crate::utils::scripts::run_script;

mod cli;

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            init_command(sub_matches)
                .await
                .unwrap_or_else(|err| logger::error(&format!("{}", err)));
        }
        Some(("install", sub_matches)) => {
            install_command(sub_matches)
                .await
                .unwrap_or_else(|err| logger::error(&format!("{}", err)));
        }
        Some(("cache", sub_matches)) => {
            cache_command(sub_matches)
                .await
                .unwrap_or_else(|err| logger::error(&format!("{}", err)));
        }
        Some(("run", sub_matches)) => {
            run_command(sub_matches)
                .await
                .unwrap_or_else(|err| logger::error(&format!("{}", err)));
        }
        Some(("start", sub_matches)) => {
            start_command(sub_matches)
                .await
                .unwrap_or_else(|err| logger::error(&format!("{}", err)));
        }
        _ => {
            let script_name = matches.subcommand().unwrap().0;
            run_script(script_name);
        }
    }
}
