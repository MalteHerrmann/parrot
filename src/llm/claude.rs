use std::process::Command;

use async_trait::async_trait;

use crate::{
    error::LLMError,
    llm::{Model, ModelFactory, constants::names},
};

const CLAUDE_CLI_NAME: &str = "claude";

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

#[async_trait]
impl Model for Claude {
    fn get_name(&self) -> String {
        names::CLAUDE.into()
    }

    async fn prompt(&self, input: &str) -> Result<String, LLMError> {
        let out = Command::new(CLAUDE_CLI_NAME)
            .args([input, "-p"])
            .output()
            .map_err(|e| LLMError::Prompt(e.to_string()))?;

        Ok(String::from_utf8(out.stdout)?)
    }
}
