use std::io::Error;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum BlossomError {
    #[error("failed to execute http request ({0})")]
    HTTPFailed(reqwest::Error),

    #[error("failed to get http response text ({0})")]
    FailedResponseText(reqwest::Error),

    #[error("invalid version notation ({0})")]
    InvalidVersionNotation(node_semver::SemverError),

    #[error("failed to parse http data to struct via json ({0})")]
    ParsingFailed(serde_json::Error),

    #[error("failed to get http response bytes ({0})")]
    FailedResponseBytes(reqwest::Error),

    #[error("failed to extract tar file ({0})")]
    ExtractionFailed(Error),

    #[error("{0}")]
    PackageNotFound(String),
}