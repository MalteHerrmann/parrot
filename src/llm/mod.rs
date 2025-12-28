/// The `llm` module contains the required logic
/// to interact with a generalized selection of
/// supported models.
use crate::error::LLMError;

use std::env::var;

/// Promptable defines the required functionality
/// to interact with a language model.
pub trait Promptable {
    fn prompt(&self, input: str) -> Result<(), LLMError>;
}

/// Returns the available models in the current
/// system context.
pub fn get_available_models() -> Result<Vec<String>, LLMError> {
    let mut models = vec![];

    if check_for_anthropic_key() {
        models.push("anthropic".to_string())
    }

    Ok(models)
}

fn check_for_anthropic_key() -> bool {
    match var("ANTHROPIC_API_KEY") {
        Ok(v) => v.is_empty(),
        Err(_) => false,
    }
}
