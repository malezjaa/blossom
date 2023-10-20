use std::process::{Command, exit};
use colored::Colorize;
use crate::structs::package::Package;
use crate::utils::logger;

pub fn run(args: Vec<&str>) {
    let current_dir = std::env::current_dir().unwrap();
    let (sh, sh_flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let status = Command::new(sh)
        .arg(sh_flag)
        .arg(args.join(" "))
        .current_dir(current_dir)
        .status();

    match status {
        Ok(status) => {
            if !status.success() {
                logger::error("Failed to execute command");
                exit(1);
            }
        }
        Err(_) => {
            logger::error("Failed to execute command");
            exit(1);
        }
    }
}

pub fn run_script(script_name: &str) {
    let package_json = Package::read_from_file().unwrap();
    let scripts = package_json.scripts.unwrap();

    if scripts.get(script_name).is_none() {
        logger::error(&format!("Couldn't find {} script", script_name.bold()));
        return;
    }

    let script_contents = scripts.get(script_name).unwrap().split(' ').collect::<Vec<&str>>();

    run(script_contents);
}