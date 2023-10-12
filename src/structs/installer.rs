use std::fs;
use crate::structs::http::Requester;
use crate::utils::logger;
use crate::utils::types::VersionData;
use colored;
use colored::Colorize;
use crate::structs::cache::Cache;
use crate::structs::package::{Package, PackageJson};

pub struct Installer {
    pub package_name: String,
    pub version: String,
    requester: Requester,
}

impl Installer {
    pub fn new(package_name: String, version: String) -> Self {
        let http = Requester::new();

        Self {
            package_name,
            version,
            requester: http,
        }
    }

    pub async fn package_data(&self, name: String, version: String) -> Result<VersionData, Box<dyn std::error::Error>> {
        let package_data = self.requester.get_version_metadata(&name.to_string(), &version.to_string()).await?;

        Ok(package_data)
    }

    pub async fn check_for_json() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir().unwrap();
        let path = current_dir.join("package.json");

        if !path.exists() {
            logger::error(&format!("No {} found in current directory. Please create one with {}", "package.json".bold(), "bl init".bold()));
            return Ok(())
        }

        Ok(())
    }

    async fn create_modules_folder(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("node_modules");

        if !path.exists() {
            fs::create_dir(path)?;
        }

        Ok(())
    }

    //TODO: add support for lockfile and dependencies in package.json
    pub async fn install(&self, is_dev: &bool, is_package_dep: &bool) -> Result<(), Box<dyn std::error::Error>> {
        Self::check_for_json().await?;
        self.create_modules_folder().await?;
        let package_info = self.package_data(self.package_name.clone(), self.version.clone()).await?;

        let cache = Cache::new();
        let tarball_url = package_info.dist.tarball.clone();

        match cache.clone_from_cache(&tarball_url, &package_info.version.to_string(), &package_info.name.to_string()).await {
            Ok(_) => {
                if !is_package_dep {
                    Package::save_dependency(is_dev, &package_info.version.to_string(), &package_info.name.to_string()).unwrap();
                    println!("{}", format!("{} {} {}", "+".bold().cyan(), package_info.name.bold(), package_info.version.bold().truecolor(106, 106, 106)).as_str())
                }
            }
            Err(err) => {
                logger::error(&format!("{}", err));
            }
        }

        Ok(())
    }
}