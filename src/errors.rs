// Module: errors
// This module contains the error types and Result type used throughout the project.

pub enum Errors {}
pub type Result<T> = std::result::Result<T, Errors>;
