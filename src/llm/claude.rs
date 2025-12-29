use std::process::Command;

use crate::{error::LLMError, llm::Model};

const CLAUDE_CLI_NAME: &str = "claude";

pub struct Claude {}

impl Claude {
    pub fn init() -> Result<Self, LLMError> {
        Command::new(CLAUDE_CLI_NAME)
            .arg("-h")
            .output()
            .map_err(|_| LLMError::CLINotFound(CLAUDE_CLI_NAME.into()))?;

        Ok(Self {})
    }
}

impl Model for Claude {
    fn get_name(&self) -> String {
        return "Claude CLI".into();
    }

    fn prompt(&self, input: &str) -> Result<String, LLMError> {
        let out = Command::new(CLAUDE_CLI_NAME)
            .args([input, "-p"])
            .output()
            .map_err(|e| LLMError::Prompt(e.to_string()))?;

        Ok(String::from_utf8(out.stdout)?)
    }
}
