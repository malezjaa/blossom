use std::fs;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;
use crate::structs::http::Requester;
use crate::utils::files::{copy_dir_contents, symlink_dir};

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
        let cache_dir = self.get_cache_dir().unwrap();
        let package_dir = cache_dir.join(format!("{}@{}", name, version));
        let current_dir = std::env::current_dir()?;
        let node_path = current_dir.join("node_modules").join(name);

        if package_dir.exists() {
            fs::remove_dir_all(&node_path);
            copy_dir_contents(&package_dir, &node_path)?;

            return Ok(());
        }

        let bytes = Requester::get_bytes(Client::new(), String::from(tarball_url)).await?;

        let bytes = &bytes.to_vec()[..];
        let gz = GzDecoder::new(bytes);
        let mut archive = Archive::new(gz);

        let tmp_dir = current_dir.join("node_modules").join("tmp").join(name);
        archive.unpack(&tmp_dir)?;
        fs::create_dir_all(&package_dir)?;
        copy_dir_contents(&tmp_dir.join("package"), &package_dir)?;
        symlink_dir(&package_dir, &node_path)?;
        fs::remove_dir_all(current_dir.join("node_modules").join("tmp"))?;

        Ok(())
    }
}

