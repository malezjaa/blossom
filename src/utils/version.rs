use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};
use crate::errors::BlossomError;

const EMPTY_VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

type PackageDetails = (String, Option<Comparator>);

pub struct VersionParser;

impl VersionParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_package_name(&self, package_name: String) -> Result<PackageDetails, String> {
        let mut split = package_name.split('@');

        let name = split
            .next().unwrap().to_string();

        //TODO: add error handling

        let version_raw = match split.next() {
            Some(version_raw) if version_raw == "latest" => return Ok((name, None)),
            Some(version_raw) => version_raw,
            None => return Ok((name, None)),
        };

        let comparator = Self::parse_semantic_version(&version_raw.to_string()).unwrap();
        Ok((name, Some(comparator)))
    }

    pub fn parse_semantic_version(raw_version: &String) -> Result<Comparator, BlossomError> {
        let mut version =
            VersionReq::parse(raw_version).map_err(BlossomError::InvalidVersionNotation)?;
        Ok(version.comparators.remove(0))
    }

    pub fn resolve_full_version(semantic_version: Option<Comparator>) -> Option<String> {
        let latest = String::from("latest");

        let semantic_version = match semantic_version {
            Some(semantic_version) => semantic_version,
            None => return Some(latest),
        };

        let (minor, patch) = match (semantic_version.minor, semantic_version.patch) {
            (Some(minor), Some(patch)) => (minor, patch),
            _ => return None,
        };

        match semantic_version.op {
            Op::Greater | Op::GreaterEq | Op::Wildcard => Some(latest),
            Op::Exact | Op::LessEq | Op::Tilde | Op::Caret => {
                Some(Self::make_string(semantic_version.major, minor, patch))
            }
            _ => None,
        }
    }
    pub fn make_string(major: u64, minor: u64, patch: u64) -> String {
        format!("{}.{}.{}", major, minor, patch)
    }
}