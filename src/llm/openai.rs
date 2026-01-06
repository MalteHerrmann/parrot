use std::env::var;

use crate::{
    error::LLMError,
    llm::{constants::names, Model, ModelFactory},
};

pub struct OpenAI {
    api_key: String,
}

impl ModelFactory for OpenAI {
    fn init() -> Result<Self, LLMError> {
        let api_key = var("OPENAI_API_KEY")?;
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
