use std::{env, fs, path::PathBuf};
use std::collections::HashMap;
use clap::ArgMatches;
use colored::Colorize;
use crate::structs::http::Requester;
use crate::structs::installer::Installer;
use crate::structs::lockfile::LockFile;
use crate::utils::{logger, types::VersionData, version::VersionParser};

pub async fn install_command(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    Installer::check_for_json().await?;
    Installer::create_modules_folder().await?;

    let package_names = sub_matches
        .get_many::<String>("package_name")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    let is_dev = sub_matches
        .get_one::<bool>("dev")
        .unwrap();

    let (packages_to_install, dev_packages_to_install) = LockFile::resolve_packages()?;
    let mut installer = Installer::new();

    install_packages(packages_to_install.clone(), false, &mut installer).await?;
    install_packages(dev_packages_to_install.clone(), true, &mut installer).await?;

    for package_name in &package_names {
        let versions = VersionParser::parse_package_name(package_name.to_string())?;
        let semantic_version = VersionParser::resolve_full_version(versions.1).ok_or("Invalid version")?;

        installer.install_package(&versions.0, semantic_version.clone(), is_dev, &false, &false).await?;
    }

    installer.display_packages()?;

    Ok(())
}

async fn install_packages(
    packages: HashMap<String, String>,
    is_dev: bool,
    installer: &mut Installer
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, version) in packages {
        installer.install_package(&name, version.clone(), &is_dev, &true, &false).await?;
    }
    Ok(())
}
