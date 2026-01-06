/// The `llm` module contains the required logic
/// to interact with a generalized selection of
/// supported models.
mod anthropic;
mod claude;
mod cursor;
mod openai;

/// Model name constants that can be used to check against
/// model names in external projects.
pub mod constants;

use crate::{
    error::LLMError,
    llm::{anthropic::Anthropic, claude::Claude, cursor::CursorCLI, openai::OpenAI},
};

/// Defines the expected interface for the initialization
/// of the different supported models.
///
/// Note: It's required to separate this from the actual model
/// trait, that defines the interface for interaction with the
/// LLMs. This is because we store the actual `Model` in a boxed
/// vector.
///
/// To allow this, the instances of `Model` have to be `dyn`-compatible,
/// which requires the trait not to be `Sized` as it is described here:
/// https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility.
pub trait ModelFactory: Model + Sized {
    fn init() -> Result<Self, LLMError>;
}

/// Defines the required functionality
/// to interact with a language model.
pub trait Model: Send + Sync {
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
