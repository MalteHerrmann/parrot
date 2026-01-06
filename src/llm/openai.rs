use std::env::var;

use crate::{
    error::LLMError,
    llm::{Model, ModelFactory, constants::names},
};

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub struct OpenAI {
    api_key: String,
}

impl ModelFactory for OpenAI {
    fn init() -> Result<Self, LLMError> {
        let api_key = var(OPENAI_API_KEY)?.trim().to_string();
        if api_key.is_empty() {
            return Err(LLMError::EmptyCredential(OPENAI_API_KEY.into()));
        }

        Ok(Self { api_key })
    }
}

impl Model for OpenAI {
    fn get_name(&self) -> String {
        names::OPENAI.into()
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("open ai api")
    }
}
