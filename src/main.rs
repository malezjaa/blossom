#![allow(dead_code)]
#![allow(unused_imports)]

use crate::cli::cli;

mod utils {
    pub mod logger;
    pub mod types;
    pub mod errors;
    pub mod version;
    pub mod files;
}

mod commands {
    pub mod init;
    pub mod install;
}

mod structs {
    pub mod http;
    pub mod package;
    pub mod cache;
    pub mod installer;
}

use utils::{logger, types, errors, version, files};
use structs::http::Requester;
use structs::package::Package;
use structs::cache::Cache;
use structs::installer::Installer;
use crate::commands::init::init_command;
use crate::commands::install::install_command;

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
        _ => {
            logger::error("No subcommand provided");
        }
    }
}
