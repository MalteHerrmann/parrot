use std::env::var;

use crate::{error::LLMError, llm::Model};

pub struct OpenAI {
    api_key: String,
}

impl OpenAI {
    pub fn init() -> Result<Self, LLMError> {
        let api_key = var("OPENAI_API_KEY")?.into();
        Ok(Self { api_key })
    }
}

impl Model for OpenAI {
    fn get_name(&self) -> String {
        return "OpenAI API".into();
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("open ai api")
    }
}
