use crate::params::Parameters;
use crate::query_openai::query_openai;

pub fn edit_file(params: &Parameters) -> Result<(), Box<dyn std::error::Error>> {
    let results = query_openai(&params)?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!("Code: {}", results.code);
    println!(
        "Description of what was done: {}",
        results.description_of_changes
    );

    Ok(())
}
