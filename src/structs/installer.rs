use std::{env, fs, io};
use std::collections::HashMap;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use crate::structs::http::Requester;
use crate::utils::logger;
use crate::utils::types::{Dist, VersionData};
use colored;
use colored::Colorize;
use crate::structs::cache::Cache;
use crate::structs::package::{Package, PackageJson};
use crate::utils::version::VersionParser;
use async_recursion::async_recursion;
use crate::structs::lockfile::{LockFile, LockFileEntry};
use crate::utils::logger::print_packages;

pub struct Installer {
    pub installed_packages: HashMap<String, String>,
    pub dev_installed_packages: HashMap<String, String>,
}

impl Installer {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
            dev_installed_packages: HashMap::new(),
        }
    }

    pub async fn package_data(http: &Requester, name: String, version: String) -> Result<VersionData, Box<dyn std::error::Error>> {
        let package_data = http.get_version_metadata(&name.to_string(), &version.to_string()).await?;

        Ok(package_data)
    }

    pub async fn check_for_json() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("package.json");

        if !path.exists() {
            logger::error(&format!("No {} found in current directory. Please create one with {}", "package.json".bold(), "bl init".bold()));
            return Ok(())
        }

        Ok(())
    }

    pub async fn create_modules_folder() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let path = current_dir.join("node_modules");

        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn install_package(&mut self, name: &str, version: String, is_dev: &bool, from_json: &bool, is_package_dep: &bool) -> Result<(), Box<dyn std::error::Error>> {
        print!("📦 {}        \r", name);

        let http = Requester::new();
        let lock_entry = LockFile::find_entry(name);
        let cache = Cache::new();
        let package_json= Package::read_from_file()?;
        let mut dist: Dist = Dist {
            tarball: "".to_string(),
        };
        let mut deps: Option<HashMap<String, String>> = None;
        let mut resolved_name = String::new();
        let mut resolved_version = String::new();

        if let Some(w) = lock_entry.clone() {
            if version != "latest" && version == w.version {
                let entry = lock_entry.clone().unwrap();
                dist.tarball = entry.dist.tarball.clone();
                deps = entry.dependencies.clone();
                resolved_name = entry.name.clone();
                resolved_version = entry.version;
            } else {
                let package_info = Installer::package_data(&http, name.to_string(), version.to_string()).await?;

                dist.tarball = package_info.dist.tarball.clone();
                resolved_name = package_info.name.clone();
                resolved_version = package_info.version.clone();
                deps = package_info.dependencies;
            }
        } else {
            let package_info = Installer::package_data(&http, name.to_string(), version.to_string()).await?;

            dist.tarball = package_info.dist.tarball.clone();
            resolved_name = package_info.name.clone();
            resolved_version = package_info.version.clone();
            deps = package_info.dependencies;
        }

        for (name, dep_version) in deps.clone().unwrap_or_default() {
            let versions = VersionParser::parse_package_name(VersionParser::combine_name(&name, &dep_version))?;
            let semantic_version = VersionParser::resolve_full_version(versions.1).ok_or("Invalid version")?;
            let mut installer = Installer::new();
            installer.install_package(&name, semantic_version, is_dev, &false, &true).await?;
        }

        match cache.clone_from_cache(&dist.tarball, &resolved_version.clone(), &resolved_name).await {
            Ok(_) => {
                io::stdout().flush()?;
                if !is_package_dep {
                    Package::save_dependency(is_dev, &resolved_version.clone(), &resolved_name).unwrap();
                    if *is_dev {
                        self.dev_installed_packages.insert(resolved_name.to_string(), resolved_version.clone());
                    } else {
                        self.installed_packages.insert(resolved_name.to_string(), resolved_version.clone());
                    }
                }

                if lock_entry.clone().is_none() {
                    let entry = LockFileEntry {
                        name: resolved_name.to_string(),
                        version: resolved_version.clone(),
                        dependencies: deps.clone(),
                        is_package_dep: *is_package_dep,
                        is_dev: *is_dev,
                        dist,
                    };

                    LockFile::add_entry(entry).unwrap();
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        Ok(())
    }

    pub fn display_packages(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.installed_packages.is_empty() && self.dev_installed_packages.is_empty() {
            logger::info("Already up to date");
            return Ok(());
        }

        println!();

        print_packages("dependencies", &self.installed_packages);
        print_packages("devDependencies", &self.dev_installed_packages);

        Ok(())
    }
}