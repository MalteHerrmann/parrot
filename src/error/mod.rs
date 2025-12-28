/// This module contains the custom errors for this project.
use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("cli not found: {0}")]
    CLINotFound(String),
    #[error("missing env variable: {0}")]
    Env(#[from] VarError),
    #[error("failed to prompt")]
    Prompt,
}
