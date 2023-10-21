use std::collections::HashMap;
use std::iter::Map;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct VersionData {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dist: Dist,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum BinData {
    StringValue(String),
    HashMapValue(HashMap<String, String>),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Dist {
    pub tarball: String,
}
