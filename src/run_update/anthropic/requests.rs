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

fn system_prompt(language: &str) -> String {
    match language {
        "Markdown" => system_prompt_markup(language),
        _ => system_prompt_programming(language),
    }
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

fn user_prompt(configs: &Configs, language: &str) -> String {
    match language {
        "Markdown" => user_prompt_markup(configs),
        _ => user_prompt_programming(configs),
    }
}

pub fn request_edit_code_block(configs: &Configs, language: &str) -> Value {
    json!({
        "max_tokens": configs.max_tokens,
        "messages": [{"content": user_prompt(configs, language), "role": "user"}],
        "model": configs.model,
        "system": [{"text": system_prompt(language), "type": "text"}],
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
