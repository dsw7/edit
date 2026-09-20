use serde_json::{Value, json};

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

fn system_prompt_code_generation() -> &'static str {
    "You are a helpful programming assistant.

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- description_of_what_was_done: brief summary of what you did
- code: your updated code
"
}

pub fn request_write_new_code(max_tokens: u16, model: &str, prompt: &str) -> Value {
    json!({
        "max_tokens": max_tokens,
        "messages": [{"content": prompt, "role": "user"}],
        "model": model,
        "output_config": schema_structured_output_code_generation(),
        "system": [{"text": system_prompt_code_generation(), "type": "text"}],
    })
}

pub fn request_edit_code_block(
    code_block: &str,
    max_tokens: u16,
    model: &str,
    prompt: &str,
) -> Value {
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
        "max_tokens": max_tokens,
        "messages": [{"content": prompt, "role": "user"}],
        "model": model,
        "output_config": schema_structured_output_code_generation(),
        "system": [{"text": system_prompt_code_generation(), "type": "text"}],
    })
}
