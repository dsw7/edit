#[derive(Debug)]
pub struct ValidationResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub reasoning: String,
    pub total_duration: f32,
    pub valid_instructions: bool,
}
