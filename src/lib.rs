mod error;
pub mod llm;

// Re-export constants for convenient access
pub use llm::constants;

// Re-export Model trait and get_available_models function for convenient access
pub use llm::{get_available_models, Model};
