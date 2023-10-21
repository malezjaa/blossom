use std::collections::HashMap;
use std::{env, fs};
use serde::{Deserialize, Serialize};
use crate::structs::package::Package;
use crate::utils::types::{BinData, Dist, VersionData};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LockFileEntry  {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dist: Dist,
    pub is_package_dep: bool,
    pub is_dev: bool,
    pub bin: Option<BinData>
}

pub struct LockFile;

impl LockFile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_lock_file() {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("blossom.lock");

        if path.exists() {
            return;
        }

        fs::File::create(path).unwrap();
    }

    pub fn lock_file_contents() -> HashMap<String, LockFileEntry> {
        LockFile::create_lock_file();
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("blossom.lock");
        let contents = fs::read_to_string(path).unwrap_or("{}".parse().unwrap());
        let contents: HashMap<String, LockFileEntry> = serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new());

        contents
    }

    pub fn find_entry(name: &str) -> Option<LockFileEntry> {
        LockFile::create_lock_file();
        if name.is_empty() {
            return None;
        }

        let contents = LockFile::lock_file_contents();

        contents.get(name).cloned()
    }


    pub fn add_entry(entry: LockFileEntry) -> Result<(), Box<dyn std::error::Error>> {
        LockFile::create_lock_file();
        let lock_entry = LockFile::find_entry(&entry.name);
        if lock_entry.is_some() {
            return Ok(());
        }

        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join("blossom.lock");
        let mut contents = LockFile::lock_file_contents();

        contents.insert(entry.name.clone(), entry);

        let contents = serde_json::to_string_pretty(&contents)?;

        fs::write(path, contents)?;

        Ok(())
    }


    pub fn resolve_packages() -> Result<(HashMap<String, String>, HashMap<String, String>), Box<dyn std::error::Error>> {
        let package_json = Package::read_from_file()?;
        let mut packages_to_download = HashMap::new();
        let mut dev_packages_to_download = HashMap::new();

        let dependencies = &package_json.dependencies;
        let dev_dependencies = &package_json.devDependencies;

        for (target_map, dep_map) in &mut [
            (&mut packages_to_download, dependencies),
            (&mut dev_packages_to_download, dev_dependencies),
        ] {
            if let Some(deps) = dep_map {
                for (name, version) in deps {
                    let node_modules = env::current_dir()?.join("node_modules").join(name);

                    if !node_modules.exists() {
                        target_map.insert(name.clone(), version.clone());
                    }
                }
            }
        }

        Ok((packages_to_download, dev_packages_to_download))
    }
}