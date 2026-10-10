use serde_json::{Value, json};

use super::params::AnthropicParams;

fn schema_structured_output_code_generation() -> Value {
    json!({
        "format": {
            "type": "json_schema",
            "schema": {
                "type": "object",
                "properties": {
                    "description_of_what_was_done": { "type": "string" },
                    "code": { "type": "string" }
                },
                "required": ["description_of_what_was_done", "code"],
                "additionalProperties": false
            }
        }
    })
}

fn system_prompt_code_generation(lang: &str) -> String {
    format!(
        "You are a helpful programming assistant that specializes in: {lang}

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- description_of_what_was_done: brief summary of what you did
- code: your updated code
"
    )
}

pub fn request_edit_code_block(code_block: &str, prompt: &str, params: &AnthropicParams) -> Value {
    let prompt = format!(
        "Take the instructions:
```plaintext
{prompt}
```
And apply them to the code:
```
{code_block}
```"
    );

    json!({
        "max_tokens": params.max_tokens,
        "messages": [{"content": prompt, "role": "user"}],
        "model": params.model,
        "output_config": schema_structured_output_code_generation(),
        "system": [{"text": system_prompt_code_generation(&params.programming_language), "type": "text"}],
    })
}
