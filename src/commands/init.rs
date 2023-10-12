use crate::structs::package::Package;
use crate::utils::logger;

pub async fn init_command(
    sub_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let package = sub_matches
        .get_one::<String>("package_name")
        .map(|s| s.as_str());

    match Package::new().init_package(package) {
        Ok(_) => {
            logger::success("Successfully initialized package");
        }
        Err(err) => {
            logger::error(&format!("{}", err));
        }
    }

    Ok(())
}