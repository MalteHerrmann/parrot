/// This module contains the custom errors for this project.
use std::{env::VarError, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("conversion error: {0}")]
    BytesConversion(#[from] FromUtf8Error),
    #[error("cli not found: {0}")]
    CLINotFound(String),
    #[error("missing env variable: {0}")]
    Env(#[from] VarError),
    #[error("failed to prompt: {0}")]
    Prompt(String),
}
