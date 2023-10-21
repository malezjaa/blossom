
use std::{env,fs};
use crate::utils::constants::{bash_content, cmd_content, powershell_content};

pub struct Binary;

impl Binary {
    pub fn assure_binary() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir().unwrap();
        let binary_path = current_dir.join("node_modules/.bin");

        if !binary_path.exists() {
            fs::create_dir_all(binary_path).unwrap();
            return Ok(())
        }

        Ok(())
    }

    pub fn create_package_binary(package_name: &str, bin_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Binary::assure_binary()?;
        let current_dir = env::current_dir().unwrap();
        let binary_path = current_dir.join("node_modules/.bin");
        let powershell_file = binary_path.join(format!("{}.ps1", package_name));
        let bash_file = binary_path.join(package_name);
        let cmd_file = binary_path.join(format!("{}.cmd", package_name));
        let bash_content = bash_content(package_name, bin_path);
        let powershell_content = powershell_content(package_name, bin_path);
        let cmd_content = cmd_content(package_name, bin_path);

        if !bash_file.exists() {
            fs::write(&bash_file, bash_content)?;
        }

        if !powershell_file.exists() {
            fs::write(&powershell_file, powershell_content)?;
        }

        if !cmd_file.exists() {
            fs::write(&cmd_file, cmd_content)?;
        }

        Ok(())
    }
}