use node_semver::{Range, Version};

pub struct VersionParser;

impl VersionParser {
    pub fn resolve_package_name(name: &str) -> Result<(String, String), String> {
        if name.starts_with('@') {
            let split = name.split('@');
            let mut parts = split.clone().collect::<Vec<&str>>();
            parts.remove(0);
            if parts.len() == 2 {
                let name = format!("@{}", parts[0]);
                let version_raw = parts[1];

                Ok((name, version_raw.to_string()))
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

            Ok((name, version_raw.to_string()))
        }
    }

    pub fn parse_version(name: &str, version: &str) -> Result<Version, Box<dyn std::error::Error>> {
        let mut version_str =  version.to_string();

        //when version is only major, for example 2 it will be 2.0.0,
        // as npm registry doesn't support only major version
        if !version_str.contains('.') {
            version_str += ".0.0";
        }

        //if version is for example @2 || 3 || 4, get the last version
        if version_str.contains("||") {
            let split = version_str.split("||");
            let mut parts = split.clone().collect::<Vec<&str>>();
            let last_version = parts.pop().unwrap().trim();
            version_str = last_version.to_string();
        }

        let version = Version::parse(VersionParser::sanitize_version(&version_str)).map_err(|err| {
            format!("Invalid version notation for {}@{} {}", name, version_str, err)
        })?;

        Ok(version)
    }

    pub fn has_range(version: &str) -> bool {
        version.contains(['^', '~', '=', '>', '<'].as_ref())
    }

    pub fn sanitize_version(version: &str) -> String {
        version.to_string().replace(['^', '~', '=', '>', '<'], "")
    }

    pub fn test_range(version: &str, range: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let version = Version::parse(version).map_err(|_| "Invalid version")?;
        let range = Range::parse(range)?;

        Ok(version.satisfies(&range))
    }

    pub fn combine_name(name: &str, version: &str) -> Result<String, Box<dyn std::error::Error>> {
        let version = VersionParser::parse_version(name, version)?;
        let name = name.to_string() + "@" + &*version.to_string();

        Ok(name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_package_name() {
        let name = "@types/node";
        let result = VersionParser::resolve_package_name(name).unwrap();
        assert_eq!(result, (name.to_string(), "latest".to_string()));

        let name = "@types/node@14.0.0";
        let result = VersionParser::resolve_package_name(name).unwrap();
        assert_eq!(result, ("@types/node".to_string(), "14.0.0".to_string()));

        let name = "typescript";
        let result = VersionParser::resolve_package_name(name).unwrap();
        assert_eq!(result, (name.to_string(), "latest".to_string()));
    }

    #[test]
    fn combine_name() {
        let result = VersionParser::combine_name("typescript", "4.0.0").unwrap();
        assert_eq!(result, "typescript@4.0.0".to_string());

        let result = VersionParser::combine_name("@types/node", "14.0.0").unwrap();
        assert_eq!(result, "@types/node@14.0.0".to_string());
    }

    #[test]
    fn sanitize_version() {
        let result = VersionParser::sanitize_version("^4.0.0");
        assert_eq!(result, "4.0.0".to_string());

        let result = VersionParser::sanitize_version("~4.0.0");
        assert_eq!(result, "4.0.0".to_string());

        let result = VersionParser::sanitize_version("=4.0.0");
        assert_eq!(result, "4.0.0".to_string());

        let result = VersionParser::sanitize_version(">4.0.0");
        assert_eq!(result, "4.0.0".to_string());

        let result = VersionParser::sanitize_version("<4.0.0");
        assert_eq!(result, "4.0.0".to_string());
    }

    #[test]
    fn has_range() {
        let result = VersionParser::has_range("^4.0.0");
        assert_eq!(result, true);

        let result = VersionParser::has_range("~4.0.0");
        assert_eq!(result, true);

        let result = VersionParser::has_range(">4.0.0");
        assert_eq!(result, true);

        let result = VersionParser::has_range("<4.0.0");
        assert_eq!(result, true);

        let result = VersionParser::has_range("4.0.0");
        assert_eq!(result, false);
    }
}