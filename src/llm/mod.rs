/// The `llm` module contains the required logic
/// to interact with a generalized selection of
/// supported models.
use crate::error::LLMError;

use std::{env::var, process::Command};

/// Promptable defines the required functionality
/// to interact with a language model.
pub trait Promptable {
    fn get_name(&self) -> String;
    fn prompt(&self, input: str) -> Result<(), LLMError>;
}

/// Returns the available models in the current
/// system context.
pub fn get_available_models() -> Result<Vec<String>, LLMError> {
    let mut models = vec![];

    if check_for_anthropic_key() {
        models.push("anthropic".to_string())
    }

    if check_for_claude() {
        models.push("claude".to_string())
    }

    if check_for_cursor_agent() {
        models.push("cursor-agent".to_string())
    }

    if check_for_openai_key() {
        models.push("openai".to_string())
    }

    Ok(models)
}

fn check_for_anthropic_key() -> bool {
    match var("ANTHROPIC_API_KEY") {
        Ok(v) => v != "",
        Err(_) => false,
    }
}

fn check_for_claude() -> bool {
    match Command::new("claude").arg("-h").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check_for_cursor_agent() -> bool {
    match Command::new("cursor-agent").arg("-h").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check_for_openai_key() -> bool {
    match var("OPENAI_API_KEY") {
        Ok(v) => v != "",
        Err(_) => false,
    }
}
