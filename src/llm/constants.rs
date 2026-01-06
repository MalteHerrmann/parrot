/// Model name constants that can be used to check against
/// model names in external projects.
///
/// These constants match the values returned by `Model::get_name()`.
pub mod names {
    /// The display name for the Cursor model: "Cursor CLI"
    pub const CURSOR: &str = "Cursor CLI";

    /// The display name for the Claude model: "Claude CLI"
    pub const CLAUDE: &str = "Claude CLI";

    /// The display name for the Anthropic model: "Anthropic API"
    pub const ANTHROPIC: &str = "Anthropic API";

    /// The display name for the OpenAI model: "OpenAI API"
    pub const OPENAI: &str = "OpenAI API";
}
