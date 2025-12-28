/// This module contains the custom errors for this project.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("failed to prompt")]
    PromptError,
}
