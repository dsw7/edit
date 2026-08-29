#[derive(Debug)]
pub struct ValidationResults {
    pub total_duration: u32,
    pub reasoning: String,
    pub valid_instructions: bool,
}
