#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fs;
use std::io::{Error, Write};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::structs::installer::Installer;
use crate::utils::logger;

pub struct Package {
    pub dependencies: Option<HashMap<String, String>>,
    pub devDependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJsonExample {
    pub name: String,
    pub version: String,
    pub license: String,
    pub author: String,
    pub description: String,
    pub scripts: Option<HashMap<String, String>>,
    pub main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    pub dependencies: Option<HashMap<String, String>>,
    pub devDependencies: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            dependencies: None,
            devDependencies: None,
        }
    }

    pub fn save_dependency(is_dev: &bool, version: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut package = Self::read_from_file()?;
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("package.json");
        let file_contents = fs::read_to_string(path)?;
        let orignal: Value = serde_json::from_str(&file_contents)?;
        if *is_dev {
            if package.devDependencies.is_none() {
                package.devDependencies = Some(HashMap::new());
            }
            if let Some(dev_dependencies) = &mut package.devDependencies {
                dev_dependencies.insert(name.to_string(), version.to_string());
            }
        } else {
            if package.dependencies.is_none() {
                package.dependencies = Some(HashMap::new());
            }
            if let Some(dependencies) = &mut package.dependencies {
                dependencies.insert(name.to_string(), version.to_string());
            }
        }

        let mut original = orignal.as_object().unwrap().clone();
        if let Some(dependencies) = &package.dependencies {
            original.insert("dependencies".to_string(), serde_json::to_value(dependencies)?);
        }

        if let Some(dev_dependencies) = &package.devDependencies {
            original.insert("devDependencies".to_string(), serde_json::to_value(dev_dependencies)?);
        }

        let json = serde_json::to_string_pretty(&original)?;
        let mut file = fs::File::create(current_dir.join("package.json"))?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn init_package(&self, name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("package.json");
        if path.exists() {
            return Err(Box::try_from("Package.json already exists").unwrap())
        }
        let folder_name = current_dir.file_name().unwrap().to_str().unwrap();

        let package_name = name.unwrap_or(folder_name);

        let mut file = fs::File::create(path)?;

        let mut scripts = HashMap::new();
        scripts.insert("test".to_string(), "echo \"Error: no test specified\" && exit 1".to_string());

        let package_json = PackageJsonExample {
            name: String::from(package_name),
            version: String::from("1.0.0"),
            description: String::from(""),
            main: String::from("index.js"),
            scripts: Some(scripts),
            author: String::from(""),
            license: String::from("ISC"),
        };

        let json = serde_json::to_string_pretty(&package_json)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn read_from_file() -> Result<PackageJson, Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("package.json");
        let file_contents = fs::read_to_string(path)?;

        let package_json: PackageJson = serde_json::from_str(&file_contents)?;

        Ok(package_json)
    }
}