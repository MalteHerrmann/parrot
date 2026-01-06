use std::process::Command;

use crate::{
    error::LLMError,
    llm::{constants::names, Model, ModelFactory},
};

const CLAUDE_CLI_NAME: &'static str = "claude";

pub struct Claude {}

impl ModelFactory for Claude {
    fn init() -> Result<Self, LLMError> {
        Command::new(CLAUDE_CLI_NAME)
            .arg("-h")
            .output()
            .map_err(|_| LLMError::CLINotFound(CLAUDE_CLI_NAME.into()))?;

        Ok(Self {})
    }
}

impl Model for Claude {
    fn get_name(&self) -> String {
        names::CLAUDE.into()
    }

    fn prompt(&self, input: &str) -> Result<String, LLMError> {
        let out = Command::new(CLAUDE_CLI_NAME)
            .args([input, "-p"])
            .output()
            .map_err(|e| LLMError::Prompt(e.to_string()))?;

        Ok(String::from_utf8(out.stdout)?)
    }
}
