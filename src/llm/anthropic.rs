use crate::error::LLMError;

use super::{Model, ModelFactory, constants::names};

use std::env::var;

const ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";

pub struct Anthropic {
    api_key: String,
}

impl ModelFactory for Anthropic {
    fn init() -> Result<Self, LLMError> {
        let api_key = var(ANTHROPIC_API_KEY)?.trim().to_string();
        if api_key.is_empty() {
            return Err(LLMError::EmptyCredential(ANTHROPIC_API_KEY.into()));
        }

        Ok(Self { api_key })
    }
}

impl Model for Anthropic {
    fn get_name(&self) -> String {
        names::ANTHROPIC.into()
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("anthropic api")
    }
}
