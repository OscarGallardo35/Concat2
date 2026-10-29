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

use std::time::Duration;

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
    ///
    /// Verified against each provider's own pricing page in October 2026:
    /// Haiku 3.5 is retired and `deepseek-chat` is a legacy name served by
    /// `deepseek-flash`, so the obvious strings are both wrong.
    pub fn default_model(self) -> &'static str {
        match self {
            Provider::OpenAi => "gpt-4o-mini",
            Provider::Anthropic => "claude-haiku-4-5",
            Provider::DeepSeek => "deepseek-flash",
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

/// How many pictures one message may carry.
///
/// Six is enough to show a shot, its neighbours and where the cut lands;
/// more explains nothing and multiplies the bill, because a picture is not
/// free: at 1,024 px it is about 1,334 tokens, and Claude charges $1 per
/// million of those.
pub const MAX_IMAGES: usize = 6;

/// A picture the person attached, already encoded: the frame a
/// `preview_frame` call returned, or a still they dropped in.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    /// `image/png` or `image/jpeg`, as the provider expects it spelled.
    pub media_type: String,
    /// The bytes, base64, without a data-URL prefix.
    pub base64: String,
}

impl Image {
    /// The picture as a data URL, which is how OpenAI takes one inline.
    pub fn data_url(&self) -> String {
        format!("data:{};base64,{}", self.media_type, self.base64)
    }

    /// The picture in a tool result: `preview_frame` answers with a block
    /// shaped `{ "type": "image", "data": ..., "mimeType": ... }`, and
    /// this is how it becomes one the model can be shown.
    pub fn from_mcp_block(block: &Value) -> Option<Image> {
        if block.get("type")?.as_str()? != "image" {
            return None;
        }
        Some(Image {
            media_type: block
                .get("mimeType")
                .and_then(Value::as_str)
                .unwrap_or("image/png")
                .to_owned(),
            base64: block.get("data")?.as_str()?.to_owned(),
        })
    }
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
    /// Something the person said, and what they showed with it.
    User {
        /// The message, as typed.
        text: String,
        /// The pictures they attached, at most [`MAX_IMAGES`] of which
        /// are sent.
        images: Vec<Image>,
    },
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
            Turn::User { text, images } => {
                if images.is_empty() {
                    messages.push(json!({ "role": "user", "content": text }));
                } else {
                    // With a picture, OpenAI wants the content as blocks;
                    // "low" detail is a fixed, small number of tokens and
                    // enough to see a frame. An empty text block is not
                    // sent: it is tokens paid for nothing.
                    let mut blocks = Vec::with_capacity(images.len() + 1);
                    if !text.is_empty() {
                        blocks.push(json!({ "type": "text", "text": text }));
                    }
                    for image in images.iter().take(MAX_IMAGES) {
                        blocks.push(json!({
                            "type": "image_url",
                            "image_url": { "url": image.data_url(), "detail": "low" },
                        }));
                    }
                    messages.push(json!({ "role": "user", "content": blocks }));
                }
            }
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
            Turn::User { text, images } => {
                let mut blocks = Vec::with_capacity(images.len() + 1);
                if !text.is_empty() {
                    blocks.push(json!({ "type": "text", "text": text }));
                }
                for image in images.iter().take(MAX_IMAGES) {
                    blocks.push(json!({
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": image.media_type,
                            "data": image.base64,
                        },
                    }));
                }
                // An empty content array is refused outright.
                if blocks.is_empty() {
                    blocks.push(json!({ "type": "text", "text": "" }));
                }
                messages.push(json!({ "role": "user", "content": blocks }));
            }
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

/// How long to wait for the provider to answer at all.
const CONNECT: Duration = Duration::from_secs(20);
/// How long to wait between reads: a model may think before the first word.
const READ: Duration = Duration::from_secs(120);

/// Everything one question needs: who answers, with which key, about what.
pub struct Ask<'a> {
    /// Who answers.
    pub provider: Provider,
    /// Which of its models.
    pub model: &'a str,
    /// The key, unsealed, held only for the length of this call.
    pub key: &'a str,
    /// What has been said so far, and what the assistant is told it is.
    pub conversation: &'a Conversation,
    /// What it may call. Empty means a plain conversation.
    pub tools: &'a [ToolSpec],
    /// How long an answer may run.
    pub max_tokens: u32,
}

/// Asks, and waits for the whole answer.
///
/// Used whenever the assistant may call a tool: a call arrives split across
/// events when streamed, and a chat that edits a timeline is worth more
/// than one that types itself out.
pub fn ask(asked: &Ask<'_>) -> Result<Reply, String> {
    let response = post(asked, false)?;
    let body = response
        .into_string()
        .map_err(|error| format!("the answer could not be read: {error}"))?;
    let value: Value = serde_json::from_str(&body)
        .map_err(|_| format!("the provider did not answer with JSON: {}", shortened(&body)))?;
    decode(asked.provider, &value)
}

/// Asks, and hands each piece of the answer to `on_text` as it arrives.
///
/// Tools are not streamed here: this is the path a plain conversation
/// takes, where the first words showing up at once is the whole point.
pub fn ask_streaming(asked: &Ask<'_>, on_text: &mut dyn FnMut(&str)) -> Result<Reply, String> {
    let response = post(asked, true)?;
    let reader = std::io::BufReader::new(response.into_reader());
    let mut text = String::new();
    for line in std::io::BufRead::lines(reader) {
        let line = line.map_err(|error| format!("the stream stopped early: {error}"))?;
        let Some(payload) = sse_payload(&line) else {
            continue;
        };
        if payload == "[DONE]" {
            break;
        }
        let Ok(value) = serde_json::from_str::<Value>(payload) else {
            continue;
        };
        if let Some(said) = delta_text(asked.provider.protocol(), &value) {
            text.push_str(&said);
            on_text(&said);
        }
    }
    Ok(Reply {
        text,
        calls: Vec::new(),
    })
}

/// The request itself: encode, authenticate, send.
fn post(asked: &Ask<'_>, stream: bool) -> Result<ureq::Response, String> {
    let mut body = encode(
        asked.provider,
        asked.model,
        asked.conversation,
        asked.tools,
        asked.max_tokens,
    );
    if stream {
        body["stream"] = json!(true);
    }
    let headers = asked.provider.headers(asked.key);
    crate::net::post_json(
        &asked.provider.endpoint(),
        CONNECT,
        READ,
        crate::net::Trust::Shipped,
        &|request| {
            let mut request = request;
            for (name, value) in &headers {
                request = request.set(name, value);
            }
            request
        },
        &body.to_string(),
    )
    .map_err(refusal)
}

/// What to say when there is no answer: the provider's own words when it
/// sent some, and the network's explanation otherwise.
fn refusal(failure: Box<crate::net::Failure>) -> String {
    let said = failure.to_string();
    let error = match *failure {
        crate::net::Failure { error, .. } => error,
    };
    match error {
        // A status carries the body, and these providers explain a 401 in
        // it: "Incorrect API key provided" beats "the server answered 401".
        ureq::Error::Status(code, response) => {
            let body = response.into_string().unwrap_or_default();
            match serde_json::from_str::<Value>(&body)
                .ok()
                .and_then(|value| error_message(&value))
            {
                Some(message) => message,
                None if body.trim().is_empty() => format!("the provider answered {code}"),
                None => format!("the provider answered {code}: {}", shortened(&body)),
            }
        }
        _ => said,
    }
}

/// The JSON of a `data:` line of a server-sent event stream, when the line
/// is one. Comments, blanks and the other fields are not.
fn sse_payload(line: &str) -> Option<&str> {
    let rest = line.trim_end().strip_prefix("data:")?;
    let rest = rest.strip_prefix(' ').unwrap_or(rest);
    if rest.is_empty() {
        None
    } else {
        Some(rest)
    }
}

/// The text a streamed event carries, in either protocol.
fn delta_text(protocol: Protocol, value: &Value) -> Option<String> {
    match protocol {
        Protocol::OpenAi => value
            .get("choices")?
            .get(0)?
            .get("delta")?
            .get("content")?
            .as_str()
            .map(str::to_owned),
        Protocol::Anthropic => {
            if value.get("type")?.as_str()? != "content_block_delta" {
                return None;
            }
            value
                .get("delta")?
                .get("text")?
                .as_str()
                .map(str::to_owned)
        }
    }
}

/// `text` cut to something that fits in an error line.
fn shortened(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= 160 {
        return flat;
    }
    let cut: String = flat.chars().take(157).collect();
    format!("{cut}...")
}

/// How many rounds a conversation may take before it is stopped: a model
/// that keeps asking for tools without ever answering is a loop, and a
/// panel that waits forever is worse than one that says so.
pub const STEPS: usize = 6;

/// Runs one exchange to the end: ask, run whatever tools come back, ask
/// again with their answers, and stop when the assistant speaks or when
/// [`STEPS`] rounds have gone by.
///
/// `run` is what actually touches the editor. It is passed in rather than
/// called here because the tools live in the MCP crate, above this one:
/// the host knows how to talk to a provider, not how to cut a timeline.
pub fn converse<R>(asked: &Ask<'_>, run: R) -> Result<String, String>
where
    R: FnMut(&Call) -> Result<String, String>,
{
    let provider = asked.provider;
    let model = asked.model;
    let key = asked.key;
    let tools = asked.tools;
    let max_tokens = asked.max_tokens;
    converse_with(
        asked,
        |conversation| {
            ask(&Ask {
                provider,
                model,
                key,
                conversation,
                tools,
                max_tokens,
            })
        },
        run,
    )
}

/// [`converse`], with the asking itself passed in, so the loop can be
/// tested without a provider.
pub fn converse_with<A, R>(
    asked: &Ask<'_>,
    mut respond: A,
    mut run: R,
) -> Result<String, String>
where
    A: FnMut(&Conversation) -> Result<Reply, String>,
    R: FnMut(&Call) -> Result<String, String>,
{
    let mut conversation = asked.conversation.clone();
    for _ in 0..STEPS {
        let reply = respond(&conversation)?;
        if !reply.wants_tools() {
            return Ok(reply.text);
        }
        conversation.turns.push(Turn::Assistant {
            text: reply.text,
            calls: reply.calls.clone(),
        });
        for call in &reply.calls {
            // A tool that fails is told to the model, not thrown away: it
            // can try another way, and the panel shows nothing red.
            let content = match run(call) {
                Ok(content) => content,
                Err(error) => format!("the tool failed: {error}"),
            };
            conversation.turns.push(Turn::ToolResult {
                call_id: call.id.clone(),
                content,
            });
        }
    }
    Err(format!(
        "the assistant asked for tools {STEPS} times without answering"
    ))
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
                Turn::User {
                    text: "make the intro shorter".into(),
                    images: Vec::new(),
                },
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

    #[test]
    fn only_data_lines_carry_an_event() {
        assert_eq!(sse_payload("data: {\"a\":1}"), Some("{\"a\":1}"));
        assert_eq!(sse_payload("data:{\"a\":1}"), Some("{\"a\":1}"));
        assert_eq!(sse_payload("data: [DONE]"), Some("[DONE]"));
        assert_eq!(sse_payload("event: ping"), None);
        assert_eq!(sse_payload(": keep-alive"), None);
        assert_eq!(sse_payload(""), None);
    }

    #[test]
    fn a_streamed_delta_is_read_in_both_protocols() {
        let openai = json!({ "choices": [{ "delta": { "content": "Hel" } }] });
        assert_eq!(delta_text(Protocol::OpenAi, &openai).as_deref(), Some("Hel"));
        // The first event of an OpenAI stream carries a role and no text.
        let role_only = json!({ "choices": [{ "delta": { "role": "assistant" } }] });
        assert_eq!(delta_text(Protocol::OpenAi, &role_only), None);

        let claude = json!({
            "type": "content_block_delta",
            "delta": { "type": "text_delta", "text": "lo" }
        });
        assert_eq!(delta_text(Protocol::Anthropic, &claude).as_deref(), Some("lo"));
        let ping = json!({ "type": "ping" });
        assert_eq!(delta_text(Protocol::Anthropic, &ping), None);
    }

    #[test]
    fn a_long_answer_is_cut_for_an_error_line() {
        let long = "x".repeat(400);
        let cut = shortened(&long);
        assert!(cut.chars().count() <= 160);
        assert!(cut.ends_with("..."));
        assert_eq!(shortened("  short   answer "), "short answer");
    }

    #[test]
    fn the_loop_runs_what_was_asked_and_answers_with_it() {
        let asked = Ask {
            provider: Provider::OpenAi,
            model: "gpt-4o-mini",
            key: "sk-x",
            conversation: &Conversation {
                system: "You edit video.".into(),
                turns: vec![Turn::User {
                    text: "how long is the intro?".into(),
                    images: Vec::new(),
                }],
            },
            tools: &tools(),
            max_tokens: 512,
        };

        let mut asked_for = Vec::new();
        let mut seen: Vec<usize> = Vec::new();
        let text = converse_with(
            &asked,
            |conversation| {
                seen.push(conversation.turns.len());
                if conversation.turns.iter().any(|turn| matches!(turn, Turn::ToolResult { .. })) {
                    Ok(Reply {
                        text: "The intro is 4 seconds.".into(),
                        calls: Vec::new(),
                    })
                } else {
                    Ok(Reply {
                        text: String::new(),
                        calls: vec![Call {
                            id: "call_1".into(),
                            name: "timeline_read".into(),
                            arguments: json!({ "path": "/tmp/cut" }),
                        }],
                    })
                }
            },
            |call| {
                asked_for.push(call.name.clone());
                Ok("{\"clips\":[]}".to_owned())
            },
        )
        .expect("an answer");

        assert_eq!(text, "The intro is 4 seconds.");
        assert_eq!(asked_for, vec!["timeline_read".to_owned()]);
        // The second round was asked with the call and its result in it.
        assert_eq!(seen, vec![1, 3]);
    }

    #[test]
    fn a_tool_that_fails_is_told_to_the_model() {
        let asked = Ask {
            provider: Provider::OpenAi,
            model: "m",
            key: "k",
            conversation: &Conversation::default(),
            tools: &tools(),
            max_tokens: 128,
        };
        let mut rounds = 0;
        let text = converse_with(
            &asked,
            |conversation| {
                rounds += 1;
                if rounds == 1 {
                    Ok(Reply {
                        text: String::new(),
                        calls: vec![Call {
                            id: "c".into(),
                            name: "timeline_read".into(),
                            arguments: json!({}),
                        }],
                    })
                } else {
                    // The failure has to have reached the model as words.
                    let said = format!("{conversation:?}");
                    assert!(said.contains("the tool failed: no project open"), "got {said}");
                    Ok(Reply { text: "I could not read it.".into(), calls: Vec::new() })
                }
            },
            |_call| Err("no project open".to_owned()),
        )
        .expect("an answer");
        assert_eq!(text, "I could not read it.");
    }

    #[test]
    fn a_model_that_never_answers_is_stopped() {
        let asked = Ask {
            provider: Provider::OpenAi,
            model: "m",
            key: "k",
            conversation: &Conversation::default(),
            tools: &tools(),
            max_tokens: 128,
        };
        let mut calls = 0;
        let stopped = converse_with(
            &asked,
            |_| {
                Ok(Reply {
                    text: String::new(),
                    calls: vec![Call {
                        id: "c".into(),
                        name: "timeline_read".into(),
                        arguments: json!({}),
                    }],
                })
            },
            |_call| {
                calls += 1;
                Ok("{}".to_owned())
            },
        );
        assert!(stopped.is_err(), "the loop has to end");
        // Six rounds, which is the limit, not one more.
        assert_eq!(calls, STEPS);
    }

    #[test]
    fn a_picture_is_a_data_url_for_openai() {
        let said = Conversation {
            system: String::new(),
            turns: vec![Turn::User {
                text: "where does this cut?".into(),
                images: vec![Image {
                    media_type: "image/png".into(),
                    base64: "QUJD".into(),
                }],
            }],
        };
        let body = encode(Provider::OpenAi, "gpt-4o-mini", &said, &[], 256);
        assert_eq!(body["messages"][0]["content"][0]["type"], "text");
        assert_eq!(body["messages"][0]["content"][1]["type"], "image_url");
        assert_eq!(
            body["messages"][0]["content"][1]["image_url"]["url"],
            "data:image/png;base64,QUJD"
        );
        assert_eq!(body["messages"][0]["content"][1]["image_url"]["detail"], "low");
    }

    #[test]
    fn a_picture_is_a_base64_source_for_anthropic() {
        let said = Conversation {
            system: String::new(),
            turns: vec![Turn::User {
                text: "where does this cut?".into(),
                images: vec![Image {
                    media_type: "image/png".into(),
                    base64: "QUJD".into(),
                }],
            }],
        };
        let body = encode(Provider::Anthropic, "claude-haiku-4-5", &said, &[], 256);
        let block = &body["messages"][0]["content"][1];
        assert_eq!(block["type"], "image");
        assert_eq!(block["source"]["type"], "base64");
        assert_eq!(block["source"]["media_type"], "image/png");
        assert_eq!(block["source"]["data"], "QUJD");
    }

    #[test]
    fn no_more_than_six_pictures_go_out() {
        let many = (0..10)
            .map(|n| Image {
                media_type: "image/png".into(),
                base64: format!("img{n}"),
            })
            .collect();
        let said = Conversation {
            system: String::new(),
            turns: vec![Turn::User {
                text: String::new(),
                images: many,
            }],
        };
        let openai = encode(Provider::OpenAi, "m", &said, &[], 64);
        assert_eq!(openai["messages"][0]["content"].as_array().unwrap().len(), MAX_IMAGES);
        // No text and pictures: just the pictures, with no empty text
        // block paid for.
        let claude = encode(Provider::Anthropic, "m", &said, &[], 64);
        let blocks = claude["messages"][0]["content"].as_array().unwrap();
        assert_eq!(blocks.len(), MAX_IMAGES);
        assert_eq!(blocks[0]["type"], "image");

        // A turn with neither still goes out as something: an empty
        // content array is refused outright.
        let empty = Conversation {
            system: String::new(),
            turns: vec![Turn::User {
                text: String::new(),
                images: Vec::new(),
            }],
        };
        let claude = encode(Provider::Anthropic, "m", &empty, &[], 64);
        assert_eq!(claude["messages"][0]["content"][0]["type"], "text");
    }

    #[test]
    fn a_frame_from_a_tool_becomes_a_picture() {
        // The shape preview_frame answers with, verbatim.
        let block = json!({ "type": "image", "data": "iVBORw0KGgo=", "mimeType": "image/png" });
        let image = Image::from_mcp_block(&block).expect("a picture");
        assert_eq!(image.media_type, "image/png");
        assert_eq!(image.base64, "iVBORw0KGgo=");
        assert_eq!(image.data_url(), "data:image/png;base64,iVBORw0KGgo=");
        // Anything else is not one.
        assert!(Image::from_mcp_block(&json!({ "type": "text", "text": "hi" })).is_none());
    }
}
