/// The `llm` module contains the required logic
/// to interact with a generalized selection of
/// supported models.
mod anthropic;
mod claude;
mod cursor;
mod openai;

use crate::{
    error::LLMError,
    llm::{anthropic::Anthropic, claude::Claude, cursor::CursorCLI, openai::OpenAI},
};

/// Prompt defines the required functionality
/// to interact with a language model.
pub trait Model {
    fn get_name(&self) -> String;
    fn prompt(&self, input: &str) -> Result<String, LLMError>;
}

/// Returns the available models in the current
/// system context.
pub fn get_available_models() -> Result<Vec<Box<dyn Model>>, LLMError> {
    let mut models: Vec<Box<dyn Model>> = vec![];

    if let Ok(m) = Anthropic::init() {
        models.push(Box::new(m))
    }

    if let Ok(m) = Claude::init() {
        models.push(Box::new(m))
    }

    if let Ok(m) = CursorCLI::init() {
        models.push(Box::new(m))
    }

    if let Ok(m) = OpenAI::init() {
        models.push(Box::new(m))
    }

    Ok(models)
}
