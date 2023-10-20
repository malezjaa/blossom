use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct VersionData {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dist: Dist,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Dist {
    pub tarball: String,
}
