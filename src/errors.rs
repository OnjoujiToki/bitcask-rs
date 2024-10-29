// Module: errors
// This module contains the error types and Result type used throughout the project.

use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Failed to read from data file")]
    FailedToReadFromDataFile,
    #[error("Failed to write to data file")]
    FailedToWriteToDataFile,
    #[error("Failed to sync data file")]
    FailedToSyncDataFile,
    #[error("Failed to open data file")]
    FailedToOpenDataFile,
}
pub type Result<T> = result::Result<T, Errors>;
