use std::process::Command;

use crate::{error::LLMError, llm::Model};

pub struct Claude {}

impl Claude {
    pub fn init() -> Result<Self, LLMError> {
        Command::new("claude")
            .arg("-h")
            .output()
            .map_err(|_| LLMError::CLINotFound("claude".into()))?;

        Ok(Self {})
    }
}

impl Model for Claude {
    fn get_name(&self) -> String {
        return "Claude CLI".into();
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("claude cli")
    }
}
