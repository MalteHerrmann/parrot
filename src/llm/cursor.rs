use crate::{
    error::LLMError,
    llm::{Model, ModelFactory},
};

use std::process::Command;

const CURSOR_CLI_NAME: &str = "cursor-agent";

pub struct CursorCLI {}

impl ModelFactory for CursorCLI {
    fn init() -> Result<Self, LLMError> {
        Command::new(CURSOR_CLI_NAME)
            .arg("-h")
            .output()
            .map_err(|_| LLMError::CLINotFound(CURSOR_CLI_NAME.into()))?;

        Ok(Self {})
    }
}

impl Model for CursorCLI {
    fn get_name(&self) -> String {
        "Cursor CLI".into()
    }

    fn prompt(&self, input: &str) -> Result<String, LLMError> {
        let out = Command::new(CURSOR_CLI_NAME)
            .args([input, "-p"])
            .output()
            .map_err(|e| LLMError::Prompt(e.to_string()))?;

        Ok(String::from_utf8(out.stdout)?)
    }
}
