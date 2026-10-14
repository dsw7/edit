use serde_json::{Value, json};

use crate::core::Configs;

fn format_system_prompt(language: &str) -> String {
    format!(
        "You are a helpful programming assistant that specializes in: {language}

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- code: your updated code

Put a comment describing what was done above the code.
"
    )
}

fn format_user_prompt(configs: &Configs) -> String {
    format!(
        "Take the instructions:
```plaintext
{}
```
And apply them to the code:
```
{}
```",
        configs.prompt, configs.code_to_edit
    )
}

pub fn request_edit_code_block(configs: &Configs, language: &str) -> Value {
    json!({
        "max_tokens": configs.max_tokens,
        "messages": [{"content": format_user_prompt(configs), "role": "user"}],
        "model": configs.model,
        "system": [{"text": format_system_prompt(language), "type": "text"}],
        "output_config": {
            "format": {
                "type": "json_schema",
                "schema": {
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" }
                    },
                    "required": ["code"],
                    "additionalProperties": false
                }
            }
        }
    })
}
