use crate::error::LLMError;

use super::Model;

use std::env::var;

pub struct Anthropic {
    api_key: String,
}

impl Anthropic {
    // TODO: also add a trait for this? e.g. `Initializable`?
    pub fn init() -> Result<Self, LLMError> {
        let api_key = var("ANTHROPIC_API_KEY")?;

        Ok(Self { api_key })
    }
}

impl Model for Anthropic {
    fn get_name(&self) -> String {
        return "Anthropic API".into();
    }

    fn prompt(&self, _: &str) -> Result<String, LLMError> {
        unimplemented!("anthropic api")
    }
}
