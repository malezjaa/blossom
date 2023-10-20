use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;
use tokio::fs::create_dir_all;
use crate::structs::http::Requester;
use crate::utils::files::{copy_dir_contents, symlink_dir, symlink_pkg};
use crate::utils::logger;

pub struct Cache;

impl Cache {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_cache_dir(&self) -> Result<
        PathBuf,
        Box<dyn std::error::Error>
    > {
        let home_dir = dirs::cache_dir().unwrap();
        let cache_dir = home_dir.join(".blossom");
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        Ok(cache_dir)
    }

    pub fn package_exists(&self, package_name: &str, package_version: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let cache_dir = self.get_cache_dir().unwrap();
        let package_dir = cache_dir.join(format!("{}@{}", package_name, package_version));

        if !package_dir.exists() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn clone_from_cache(&self, tarball_url: &str, version: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let normalized_name = &name.replace('/', "\\");
        let cache_dir = self.get_cache_dir().unwrap();
        let package_dir = cache_dir.join(format!("{}@{}", normalized_name, version));
        let current_dir = std::env::current_dir()?;
        let node_modules = current_dir.join("node_modules").join(normalized_name);

        if !node_modules.parent().unwrap().exists() {
            create_dir_all(node_modules.parent().unwrap()).await?;
        }

        if !package_dir.exists() {
            let bytes = Requester::get_bytes(Client::new(), String::from(tarball_url)).await?;
            let bytes = &bytes.to_vec()[..];
            let gz = GzDecoder::new(bytes);
            let mut archive = Archive::new(gz);

            let tmp_dir = current_dir.join("node_modules").join("tmp").join(normalized_name);
            archive.unpack(&tmp_dir)?;
            create_dir_all(&package_dir).await?;
            copy_dir_contents(&tmp_dir.join("package"), &package_dir)?;

            fs::remove_dir_all(current_dir.join("node_modules").join("tmp"))?;

            symlink_pkg(
                &package_dir,
                &node_modules,
            );
        } else {
            symlink_pkg(
                &package_dir,
                &node_modules,
            );
        };

        Ok(())
    }

    pub fn clear_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = self.get_cache_dir().unwrap();
        fs::remove_dir_all(cache_dir)?;

        logger::success("Successfully cleared cache");

        Ok(())
    }
}

