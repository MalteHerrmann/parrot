use crate::{error::LLMError, llm::Model};

use std::process::Command;

pub struct CursorCLI {}

impl CursorCLI {
    pub fn init() -> Result<Self, LLMError> {
        Command::new("cursor-agent")
            .arg("-h")
            .output()
            .map_err(|_| LLMError::CLINotFound("cursor-agent".into()))?;

        Ok(Self {})
    }
}

impl Model for CursorCLI {
    fn get_name(&self) -> String {
        return "Cursor CLI".into();
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("cursor cli")
    }
}
