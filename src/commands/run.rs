use std::process::Command;
use colored::Colorize;
use crate::structs::package::Package;
use crate::utils::logger;
use crate::utils::scripts::run_script;

pub async fn run_command(
    sub_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let script_name = sub_matches
        .get_one::<String>("script_name")
        .map(|s| s.as_str()).unwrap();
    run_script(script_name);

    Ok(())
}