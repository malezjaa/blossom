use std::process::Command;
use colored::Colorize;
use crate::structs::package::Package;
use crate::utils::logger;
use crate::utils::scripts::run_script;

pub async fn start_command(
    sub_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    run_script("start");

    Ok(())
}