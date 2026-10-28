// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! The chat, as far as the wire is concerned.
//!
//! Four providers speak two protocols. OpenAI, DeepSeek and OpenRouter all
//! take the same `/chat/completions` body and answer with the same
//! `choices`; Anthropic takes `/v1/messages`, puts the system prompt at the
//! top level instead of in the conversation, calls a tool's schema
//! `input_schema`, and answers with content blocks. One conversation is
//! translated into either, and either answer is read back into the same
//! [`Reply`], so nothing above this module has to know which provider is
//! configured.
//!
//! Nothing here sends anything: [`encode`] makes a body and [`decode`]
//! reads one, both pure, both tested against a fixture of each protocol.
//! The sending is [`crate::net::post_json`].

use serde_json::{json, Map, Value};

/// Who answers, and how they are reached.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Provider {
    /// `api.openai.com`, the `/chat/completions` body.
    OpenAi,
    /// `api.anthropic.com`, the `/v1/messages` body.
    Anthropic,
    /// `api.deepseek.com`, the `/chat/completions` body.
    DeepSeek,
    /// `openrouter.ai`, the `/chat/completions` body for other houses' models.
    OpenRouter,
}

/// The shape of the request and the answer. Two, for four providers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    /// `/chat/completions`, `tools[].function`, `tool_calls`.
    OpenAi,
    /// `/v1/messages`, `tools[].input_schema`, `tool_use` blocks.
    Anthropic,
}

impl Provider {
    /// Every provider, in the order the settings pane lists them.
    pub const ALL: [Provider; 4] = [
        Provider::OpenAi,
        Provider::Anthropic,
        Provider::DeepSeek,
        Provider::OpenRouter,
    ];

    /// The name used in the settings file, the address of a key, and the
    /// word a person sees.
    pub fn id(self) -> &'static str {
        match self {
            Provider::OpenAi => "openai",
            Provider::Anthropic => "anthropic",
            Provider::DeepSeek => "deepseek",
            Provider::OpenRouter => "openrouter",
        }
    }

    /// The provider `id` names, if it names one.
    pub fn from_id(id: &str) -> Option<Self> {
        Provider::ALL.into_iter().find(|one| one.id() == id)
    }

    /// What the pane shows.
    pub fn label(self) -> &'static str {
        match self {
            Provider::OpenAi => "OpenAI",
            Provider::Anthropic => "Claude",
            Provider::DeepSeek => "DeepSeek",
            Provider::OpenRouter => "OpenRouter",
        }
    }

    /// Where its API answers.
    pub fn base(self) -> &'static str {
        match self {
            Provider::OpenAi => "https://api.openai.com/v1",
            Provider::Anthropic => "https://api.anthropic.com/v1",
            Provider::DeepSeek => "https://api.deepseek.com/v1",
            Provider::OpenRouter => "https://openrouter.ai/api/v1",
        }
    }

    /// The protocol it speaks.
    pub fn protocol(self) -> Protocol {
        match self {
            Provider::Anthropic => Protocol::Anthropic,
            Provider::OpenAi | Provider::DeepSeek | Provider::OpenRouter => Protocol::OpenAi,
        }
    }

    /// The model the pane starts with: the cheap one of each house, because
    /// a first message should not cost what a flagship does.
    pub fn default_model(self) -> &'static str {
        match self {
            Provider::OpenAi => "gpt-4o-mini",
            Provider::Anthropic => "claude-3-5-haiku-latest",
            Provider::DeepSeek => "deepseek-chat",
            Provider::OpenRouter => "openai/gpt-4o-mini",
        }
    }

    /// The URL a turn is posted to.
    pub fn endpoint(self) -> String {
        match self.protocol() {
            Protocol::OpenAi => format!("{}/chat/completions", self.base()),
            Protocol::Anthropic => format!("{}/messages", self.base()),
        }
    }

    /// The headers that carry `key`, and the version an Anthropic call
    /// cannot go without.
    pub fn headers(self, key: &str) -> Vec<(&'static str, String)> {
        match self.protocol() {
            Protocol::OpenAi => vec![("authorization", format!("Bearer {key}"))],
            Protocol::Anthropic => vec![
                ("x-api-key", key.to_owned()),
                ("anthropic-version", "2023-06-01".to_owned()),
            ],
        }
    }
}

/// A tool the assistant may call: what it is called, what it does, and the
/// JSON schema of its arguments.
#[derive(Clone, Debug)]
pub struct ToolSpec {
    /// What the model calls it: the tool's own name.
    pub name: String,
    /// What it does, in the words the model reads.
    pub description: String,
    /// The JSON schema of its arguments.
    pub schema: Value,
}

/// One thing the assistant asked to call.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    /// The provider's id for this call, which the result is answered with.
    pub id: String,
    /// The tool to run.
    pub name: String,
    /// Its arguments, as the model wrote them.
    pub arguments: Value,
}

/// A conversation, from the point of view of either protocol.
#[derive(Clone, Debug, PartialEq)]
pub enum Turn {
    /// Something the person said.
    User(
        /// The message, as typed.
        String,
    ),
    /// What the assistant said, and what it asked to call along the way.
    Assistant {
        /// Its words: empty when it only asked for tools.
        text: String,
        /// What it wants run before it carries on.
        calls: Vec<Call>,
    },
    /// The answer to one call, on its way back.
    ToolResult {
        /// The call being answered.
        call_id: String,
        /// What running it returned.
        content: String,
    },
}

/// A conversation ready to send.
#[derive(Clone, Debug, Default)]
pub struct Conversation {
    /// What the assistant is told it is.
    pub system: String,
    /// Everything said so far, oldest first.
    pub turns: Vec<Turn>,
}

/// What came back.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reply {
    /// What the assistant said, which may be nothing.
    pub text: String,
    /// What it wants run next, which may be nothing.
    pub calls: Vec<Call>,
}

impl Reply {
    /// Whether the assistant wants a tool run before it says anything else.
    pub fn wants_tools(&self) -> bool {
        !self.calls.is_empty()
    }
}

/// The body a provider takes for `conversation`, `model` and `tools`.
///
/// `max_tokens` is Anthropic's one required field and OpenAI's optional
/// one, so both get it: a chat that answers forever is not what an editor
/// wants in a panel.
pub fn encode(
    provider: Provider,
    model: &str,
    conversation: &Conversation,
    tools: &[ToolSpec],
    max_tokens: u32,
) -> Value {
    match provider.protocol() {
        Protocol::OpenAi => encode_openai(model, conversation, tools, max_tokens),
        Protocol::Anthropic => encode_anthropic(model, conversation, tools, max_tokens),
    }
}

fn encode_openai(
    model: &str,
    conversation: &Conversation,
    tools: &[ToolSpec],
    max_tokens: u32,
) -> Value {
    let mut messages = Vec::with_capacity(conversation.turns.len() + 1);
    if !conversation.system.is_empty() {
        messages.push(json!({ "role": "system", "content": conversation.system }));
    }
    for turn in &conversation.turns {
        match turn {
            Turn::User(text) => messages.push(json!({ "role": "user", "content": text })),
            Turn::Assistant { text, calls } => {
                let mut message = Map::new();
                message.insert("role".into(), json!("assistant"));
                message.insert("content".into(), json!(text));
                if !calls.is_empty() {
                    message.insert(
                        "tool_calls".into(),
                        Value::Array(
                            calls
                                .iter()
                                .map(|call| {
                                    json!({
                                        "id": call.id,
                                        "type": "function",
                                        "function": {
                                            "name": call.name,
                                            // The wire wants the arguments as a string
                                            // holding JSON, not as JSON.
                                            "arguments": call.arguments.to_string(),
                                        }
                                    })
                                })
                                .collect(),
                        ),
                    );
                }
                messages.push(Value::Object(message));
            }
            Turn::ToolResult { call_id, content } => messages.push(json!({
                "role": "tool",
                "tool_call_id": call_id,
                "content": content,
            })),
        }
    }

    let mut body = Map::new();
    body.insert("model".into(), json!(model));
    body.insert("messages".into(), Value::Array(messages));
    body.insert("max_tokens".into(), json!(max_tokens));
    if !tools.is_empty() {
        body.insert(
            "tools".into(),
            Value::Array(
                tools
                    .iter()
                    .map(|tool| {
                        json!({
                            "type": "function",
                            "function": {
                                "name": tool.name,
                                "description": tool.description,
                                "parameters": tool.schema,
                            }
                        })
                    })
                    .collect(),
            ),
        );
    }
    Value::Object(body)
}

fn encode_anthropic(
    model: &str,
    conversation: &Conversation,
    tools: &[ToolSpec],
    max_tokens: u32,
) -> Value {
    let mut messages: Vec<Value> = Vec::with_capacity(conversation.turns.len());
    for turn in &conversation.turns {
        match turn {
            Turn::User(text) => messages.push(json!({
                "role": "user",
                "content": [{ "type": "text", "text": text }],
            })),
            Turn::Assistant { text, calls } => {
                let mut blocks = Vec::with_capacity(calls.len() + 1);
                if !text.is_empty() {
                    blocks.push(json!({ "type": "text", "text": text }));
                }
                for call in calls {
                    blocks.push(json!({
                        "type": "tool_use",
                        "id": call.id,
                        "name": call.name,
                        "input": call.arguments,
                    }));
                }
                messages.push(json!({ "role": "assistant", "content": blocks }));
            }
            Turn::ToolResult { call_id, content } => messages.push(json!({
                "role": "user",
                "content": [{
                    "type": "tool_result",
                    "tool_use_id": call_id,
                    "content": content,
                }],
            })),
        }
    }

    let mut body = Map::new();
    body.insert("model".into(), json!(model));
    body.insert("max_tokens".into(), json!(max_tokens));
    // Anthropic takes the system prompt beside the conversation, not in it.
    if !conversation.system.is_empty() {
        body.insert("system".into(), json!(conversation.system));
    }
    body.insert("messages".into(), Value::Array(messages));
    if !tools.is_empty() {
        body.insert(
            "tools".into(),
            Value::Array(
                tools
                    .iter()
                    .map(|tool| {
                        json!({
                            "name": tool.name,
                            "description": tool.description,
                            // The same schema, under another name.
                            "input_schema": tool.schema,
                        })
                    })
                    .collect(),
            ),
        );
    }
    Value::Object(body)
}

/// What `body` says, in either protocol.
///
/// A provider that answered with an error, or with a shape this does not
/// know, is an `Err` holding what it said: a panel that shows the
/// provider's own words is easier to act on than "bad response".
pub fn decode(provider: Provider, body: &Value) -> Result<Reply, String> {
    if let Some(message) = error_message(body) {
        return Err(message);
    }
    match provider.protocol() {
        Protocol::OpenAi => decode_openai(body),
        Protocol::Anthropic => decode_anthropic(body),
    }
}

/// The provider's own complaint, when it made one.
fn error_message(body: &Value) -> Option<String> {
    let error = body.get("error")?;
    let said = error
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| error.as_str())
        .unwrap_or("the provider refused the request");
    Some(said.to_owned())
}

fn decode_openai(body: &Value) -> Result<Reply, String> {
    let message = body
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .ok_or_else(|| "the answer has no choices[0].message".to_owned())?;

    let text = message
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();

    let mut calls = Vec::new();
    for call in message
        .get("tool_calls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let function = match call.get("function") {
            Some(function) => function,
            None => continue,
        };
        let name = function
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        // `arguments` is a string holding JSON. A model that filled it with
        // something else still gets a call, with no arguments to give it.
        let arguments = function
            .get("arguments")
            .and_then(Value::as_str)
            .and_then(|text| serde_json::from_str::<Value>(text).ok())
            .unwrap_or_else(|| json!({}));
        calls.push(Call {
            id: call
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            name,
            arguments,
        });
    }
    Ok(Reply { text, calls })
}

fn decode_anthropic(body: &Value) -> Result<Reply, String> {
    let blocks = body
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(|| "the answer has no content".to_owned())?;

    let mut text = String::new();
    let mut calls = Vec::new();
    for block in blocks {
        match block.get("type").and_then(Value::as_str) {
            Some("text") => {
                if let Some(said) = block.get("text").and_then(Value::as_str) {
                    if !text.is_empty() {
                        text.push('\n');
                    }
                    text.push_str(said);
                }
            }
            Some("tool_use") => calls.push(Call {
                id: block
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                name: block
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                arguments: block.get("input").cloned().unwrap_or_else(|| json!({})),
            }),
            _ => {}
        }
    }
    Ok(Reply { text, calls })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tools() -> Vec<ToolSpec> {
        vec![ToolSpec {
            name: "timeline_read".into(),
            description: "Reads the timeline".into(),
            schema: json!({ "type": "object", "properties": { "path": { "type": "string" } } }),
        }]
    }

    fn asked() -> Conversation {
        Conversation {
            system: "You edit video.".into(),
            turns: vec![
                Turn::User("make the intro shorter".into()),
                Turn::Assistant {
                    text: String::new(),
                    calls: vec![Call {
                        id: "call_1".into(),
                        name: "timeline_read".into(),
                        arguments: json!({ "path": "/tmp/cut" }),
                    }],
                },
                Turn::ToolResult {
                    call_id: "call_1".into(),
                    content: "{\"clips\":[]}".into(),
                },
            ],
        }
    }

    #[test]
    fn every_provider_reaches_its_own_endpoint() {
        assert_eq!(
            Provider::OpenAi.endpoint(),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            Provider::DeepSeek.endpoint(),
            "https://api.deepseek.com/v1/chat/completions"
        );
        assert_eq!(
            Provider::OpenRouter.endpoint(),
            "https://openrouter.ai/api/v1/chat/completions"
        );
        assert_eq!(
            Provider::Anthropic.endpoint(),
            "https://api.anthropic.com/v1/messages"
        );
    }

    #[test]
    fn only_anthropic_asks_for_a_version() {
        let openai = Provider::OpenAi.headers("sk-x");
        assert_eq!(openai, vec![("authorization", "Bearer sk-x".to_owned())]);
        let claude = Provider::Anthropic.headers("sk-ant");
        assert!(claude.iter().any(|(name, _)| *name == "x-api-key"));
        assert!(claude
            .iter()
            .any(|(name, value)| *name == "anthropic-version" && value == "2023-06-01"));
    }

    #[test]
    fn openai_body_puts_the_system_in_the_conversation() {
        let body = encode(Provider::OpenAi, "gpt-4o-mini", &asked(), &tools(), 512);
        assert_eq!(body["model"], "gpt-4o-mini");
        assert_eq!(body["max_tokens"], 512);
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
        // A call goes out as a string holding JSON, not as JSON.
        assert_eq!(
            body["messages"][2]["tool_calls"][0]["function"]["arguments"],
            "{\"path\":\"/tmp/cut\"}"
        );
        assert_eq!(body["messages"][3]["role"], "tool");
        assert_eq!(body["messages"][3]["tool_call_id"], "call_1");
        assert_eq!(body["tools"][0]["type"], "function");
        assert_eq!(body["tools"][0]["function"]["name"], "timeline_read");
        assert!(body["tools"][0]["function"]["parameters"].is_object());
    }

    #[test]
    fn anthropic_body_lifts_the_system_out() {
        let body = encode(Provider::Anthropic, "claude-3-5-haiku-latest", &asked(), &tools(), 512);
        assert_eq!(body["system"], "You edit video.");
        assert_eq!(body["messages"][0]["role"], "user");
        // No system turn inside the conversation.
        assert!(body["messages"]
            .as_array()
            .unwrap()
            .iter()
            .all(|message| message["role"] != "system"));
        assert_eq!(body["messages"][1]["content"][0]["type"], "tool_use");
        assert_eq!(body["messages"][1]["content"][0]["input"]["path"], "/tmp/cut");
        assert_eq!(body["messages"][2]["content"][0]["type"], "tool_result");
        assert_eq!(body["messages"][2]["content"][0]["tool_use_id"], "call_1");
        assert_eq!(body["tools"][0]["input_schema"]["type"], "object");
        // Anthropic wants a schema, not a function wrapper.
        assert!(body["tools"][0].get("function").is_none());
    }

    #[test]
    fn an_openai_answer_is_read_into_text_and_calls() {
        let body = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "I will look.",
                    "tool_calls": [{
                        "id": "call_9",
                        "type": "function",
                        "function": { "name": "timeline_read", "arguments": "{\"path\":\"/tmp/x\"}" }
                    }]
                }
            }]
        });
        let reply = decode(Provider::OpenAi, &body).expect("an answer");
        assert_eq!(reply.text, "I will look.");
        assert!(reply.wants_tools());
        assert_eq!(reply.calls[0].name, "timeline_read");
        assert_eq!(reply.calls[0].arguments["path"], "/tmp/x");
    }

    #[test]
    fn an_anthropic_answer_is_read_into_text_and_calls() {
        let body = json!({
            "content": [
                { "type": "text", "text": "I will look." },
                { "type": "tool_use", "id": "toolu_1", "name": "timeline_read", "input": { "path": "/tmp/x" } }
            ]
        });
        let reply = decode(Provider::Anthropic, &body).expect("an answer");
        assert_eq!(reply.text, "I will look.");
        assert_eq!(reply.calls[0].id, "toolu_1");
        assert_eq!(reply.calls[0].arguments["path"], "/tmp/x");
    }

    #[test]
    fn a_plain_answer_says_nothing_to_call() {
        let openai = json!({ "choices": [{ "message": { "content": "Done." } }] });
        assert!(!decode(Provider::OpenAi, &openai).unwrap().wants_tools());
        let claude = json!({ "content": [{ "type": "text", "text": "Done." }] });
        let reply = decode(Provider::Anthropic, &claude).unwrap();
        assert_eq!(reply.text, "Done.");
        assert!(!reply.wants_tools());
    }

    #[test]
    fn a_refusal_is_an_error_holding_the_providers_words() {
        let body = json!({ "error": { "message": "Incorrect API key provided" } });
        let said = decode(Provider::OpenAi, &body).unwrap_err();
        assert!(said.contains("Incorrect API key"), "got {said}");
        // Anthropic's errors share the shape.
        let said = decode(Provider::Anthropic, &body).unwrap_err();
        assert!(said.contains("Incorrect API key"));
    }

    #[test]
    fn a_call_with_unreadable_arguments_still_arrives() {
        let body = json!({
            "choices": [{ "message": { "tool_calls": [
                { "id": "c", "function": { "name": "export_wait", "arguments": "not json" } }
            ]}}]
        });
        let reply = decode(Provider::OpenAi, &body).unwrap();
        assert_eq!(reply.calls[0].name, "export_wait");
        assert_eq!(reply.calls[0].arguments, json!({}));
    }

    #[test]
    fn a_provider_id_round_trips() {
        for provider in Provider::ALL {
            assert_eq!(Provider::from_id(provider.id()), Some(provider));
        }
        assert_eq!(Provider::from_id("gemini"), None);
    }
}
