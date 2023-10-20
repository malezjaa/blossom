use crate::structs::cache::Cache;
use crate::utils::logger;

pub async fn cache_command(
    sub_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let sub_command = sub_matches.subcommand();

    match sub_command {
        Some(("clear", _matches)) => {
            Cache.clear_cache()?;
        }
        Some(("path", _matches)) => {
            println!("{}", Cache.get_cache_dir()?.display());
        }
        _ => {
            logger::error("No subcommand provided");
        }
    }

    Ok(())
}