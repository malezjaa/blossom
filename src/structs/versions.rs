use node_semver::{Range, Version};

pub struct VersionParser;

impl VersionParser {
    pub fn resolve_package_name(name: &str) -> Result<(String, String), String> {
        println!("resolving package name: {}", name);
        if name.starts_with('@') {
            let split = name.split('@');
            let mut parts = split.clone().collect::<Vec<&str>>();
            parts.remove(0);
            if parts.len() == 2 {
                let name = format!("@{}", parts[0]);
                let version_raw = parts[1];

                let comparator = VersionParser::parse_version(&name, version_raw).unwrap();

                Ok((name, comparator.to_string()))
            } else {
                let name = format!("@{}", parts[0]);

                Ok((name, "latest".to_string()))
            }
        } else {
            let mut split = name.split('@');

            let name = split
                .next().unwrap().to_string();

            let version_raw = match split.next() {
                Some(version_raw) if version_raw == "latest" => return Ok((name, "latest".to_string())),
                Some(version_raw) => version_raw,
                None => return Ok((name, "latest".to_string())),
            };

            let comparator = VersionParser::parse_version(&name, version_raw).unwrap();

            Ok((name, comparator.to_string()))
        }
    }

    pub fn parse_version(name: &str, mut version: &str) -> Result<Version, Box<dyn std::error::Error>> {
        let mut version_str = version.to_string();

        if version_str.len() == 1 {
            version_str += ".0.0";
        }

        let version = Version::parse(VersionParser::sanitize_version(&version_str)).map_err(|err| {
            format!("Invalid version notation for {}@{} {}", name, version_str, err)
        })?;

        Ok(version)
    }

    pub fn sanitize_version(version: &str) -> String {
        version.to_string().replace(['^', '~', '=', '>', '<'], "")
    }

    pub fn check_range(version: &str, range: &str) -> Result<Version, Box<dyn std::error::Error>> {
        let version = Version::parse(version).map_err(|_| "Invalid version")?;
        let range = Range::parse(range).map_err(|_| "Invalid range")?;

        if version.satisfies(&range) {
            Ok(version)
        } else {
            Err("Version does not match range".into())
        }
    }

    pub fn combine_name(name: &str, version: &str) -> Result<String, Box<dyn std::error::Error>> {
        let version = VersionParser::parse_version(name, version)?;
        let name = name.to_string() + "@" + &*version.to_string();

        Ok(name)
    }
}