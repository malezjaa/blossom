use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::iter::Map;
use bytes::Bytes;
use crate::errors::BlossomError;
use crate::errors::BlossomError::{FailedResponseText, HTTPFailed, ParsingFailed};
use crate::types::VersionData;
use crate::utils::errors::BlossomError::FailedResponseBytes;
use colored;
use colored::Colorize;
use serde_json::Value;
use crate::utils::types::BinData;

pub const REGISTRY_URL: &str = "https://registry.npmjs.org";

async fn request(client: reqwest::Client, route: String) -> Result<String, BlossomError> {
    client
        .get(format!("{REGISTRY_URL}{route}"))
        .header(
            "Accept",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        )
        .send()
        .await
        .map_err(HTTPFailed)?
        .text()
        .await
        .map_err(FailedResponseText)
}

pub struct Requester {
    client: reqwest::Client,
}

impl Requester {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_package_metadata(
        &self,
        package_name: &String,
    ) -> Result<String, BlossomError> {
        request(
            self.client.clone(),
            format!("/{package_name}/latest", package_name = package_name),
        )
            .await
    }

    pub async fn get_version_metadata(
        &self,
        package_name: &String,
        version: &String,
    ) -> Result<(VersionData, Option<BinData>), BlossomError> {
        print!("🔍 {}        \r", package_name);
        io::stdout().flush().unwrap();
        let response = request(
            self.client.clone(),
            format!("/{package_name}/{version}", package_name = package_name, version = version),
        )
            .await?;

        if response == "Not Found" {
            return Err(BlossomError::PackageNotFound(format!("{} is not in the npm registry, or you have no permission to fetch it", package_name.to_string().bold())));
        }

        let json: VersionData = serde_json::from_str(&response).map_err(ParsingFailed)?;

        let bin: Option<BinData> = {
            let full_json: Value = serde_json::from_str::<Value>(&response).unwrap_or_default();
            let bin_obj = full_json["bin"].clone();

            if bin_obj.is_string() {
                Some(BinData::StringValue(bin_obj.as_str().unwrap().to_string()))
            } else if bin_obj.is_object() {
                let mut bin_map = HashMap::new();
                for (key, value) in bin_obj.as_object().unwrap().iter() {
                    bin_map.insert(key.to_string(), value.to_string());
                }
                Some(BinData::HashMapValue(bin_map))
            } else {
                None
            }
        };

        Ok((json, bin))
    }



    pub async fn get_bytes(client: reqwest::Client, url: String) -> Result<Bytes, BlossomError> {
        client
            .get(url)
            .send()
            .await
            .map_err(HTTPFailed)?
            .bytes()
            .await
            .map_err(FailedResponseBytes)
    }
}
