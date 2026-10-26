use serde_json::{Value, json};

use crate::run_update::configs::Configs;

fn system_prompt_markup(language: &str) -> String {
    format!(
        "You are a helpful assistant that specializes in: {language}

IMPORTANT: Do not wrap your response in backticks (```). Output the response
directly without markdown code fences.

Output:
- code: your updated markup
"
    )
}

fn system_prompt_programming(language: &str) -> String {
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

fn user_prompt_markup(configs: &Configs) -> String {
    format!(
        "Take the instructions:
```plaintext
{}
```
And apply them to the markup:
```
{}
```",
        configs.prompt, configs.code_to_edit
    )
}

fn user_prompt_programming(configs: &Configs) -> String {
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
    let (user_prompt, system_prompt) = match language {
        "Markdown" => (user_prompt_markup(configs), system_prompt_markup(language)),
        _ => (
            user_prompt_programming(configs),
            system_prompt_programming(language),
        ),
    };

    json!({
        "max_tokens": configs.max_tokens,
        "messages": [{"content": user_prompt, "role": "user"}],
        "model": configs.model,
        "system": [{"text": system_prompt, "type": "text"}],
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
