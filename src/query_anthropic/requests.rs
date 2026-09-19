use serde_json::{Value, json};

fn schema_structured_output_code_generation() -> serde_json::Value {
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

fn message_param(user_prompt: &str) -> serde_json::Value {
    // see https://platform.claude.com/docs/en/api/http/messages#message_param
    json!({"content": user_prompt, "role": "user"})
}

fn text_block_param(system_prompt: &str) -> serde_json::Value {
    // see https://platform.claude.com/docs/en/api/http/messages#text_block_param
    json!({"text": system_prompt, "type": "text"})
}

pub fn request_write_new_code(max_tokens: u16, model: &str, prompt: &str) -> Value {
    json!({
        "max_tokens": max_tokens,
        "messages": vec![message_param(prompt)],
        "model": model,
        "output_config": schema_structured_output_code_generation(),
        "system": vec![text_block_param(system_prompt_code_generation())],
    })
}

fn user_prompt_code_edit(prompt: &str, code_block: &str) -> String {
    format!(
        "Take the instructions:
```plaintext
{prompt}
```
And apply them to the code:
```
{code_block}
```"
    )
}

pub fn request_edit_code_block(
    code_block: &str,
    max_tokens: u16,
    model: &str,
    prompt: &str,
) -> Value {
    json!({
        "max_tokens": max_tokens,
        "messages": vec![message_param(&user_prompt_code_edit(prompt, code_block))],
        "model": model,
        "output_config": schema_structured_output_code_generation(),
        "system": vec![text_block_param(system_prompt_code_generation())],
    })
}
