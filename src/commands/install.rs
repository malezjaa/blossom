use std::fmt::format;
use colored::Colorize;
use crate::structs::http::Requester;
use crate::structs::installer::Installer;
use crate::utils::version::VersionParser;

pub async fn install_command(
    sub_matches: &clap::ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let package_name = sub_matches
        .get_one::<String>("package_name")
        .map(|s| s.as_str());

    let is_dev = sub_matches
        .get_one::<bool>("dev").unwrap();

    let parser = VersionParser::new();

    let versions = parser.parse_package_name(package_name.unwrap().to_string())?;
    let semantic_version = VersionParser::resolve_full_version(versions.1).unwrap();
    let http = Requester::new();

    let package_data = http.get_version_metadata(&versions.0.to_string(), &semantic_version).await?;

    println!("{}", "dependencies".bold().truecolor(204, 255, 102));

    if package_data.dependencies.is_some() {
        for (name, version) in package_data.dependencies.unwrap() {
            let dep_versions = parser.parse_package_name(name + "@" + &version)?;
            let semantic_version = VersionParser::resolve_full_version(dep_versions.1).unwrap();
            let installer = Installer::new(dep_versions.0, semantic_version);
            installer.install(is_dev, &true).await?;
        }
    }

    let installer = Installer::new(versions.0, semantic_version);

    installer.install(is_dev, &false).await?;

    Ok(())
}