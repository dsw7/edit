use serde_json::json;

use crate::structs::OpenAIParams;

fn get_structured_output_schema() -> serde_json::Value {
    json!({
        "format": {
            "type": "json_schema",
            "name": "updated_code",
            "strict": true,
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

fn get_system_prompt() -> &'static str {
    "You are a helpful programming assistant.

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- description_of_what_was_done: brief summary of what you did
- code: your updated code
"
}

pub fn set_up_request_body(params: &OpenAIParams) -> serde_json::Value {
    let structured_output_schema = get_structured_output_schema();
    let system_prompt = get_system_prompt();

    json!({
        "input": params.prompt,
        "instructions": system_prompt,
        "model": params.model,
        "store": false,
        "text": structured_output_schema,
    })
}
