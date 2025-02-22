use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base for objects that include optional annotations for the client.
/// The client can use annotations to inform how objects are used or displayed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotated {
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotations {
    /// Describes who the intended customer of this object or data is.
    pub audience: Option<Vec<Role>>,
    /// Describes how important this data is for operating the server.
    /// A value of 1 means "most important" and indicates that the data is effectively required,
    /// while 0 means "least important" and indicates that the data is entirely optional.
    #[serde(default)]
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobResourceContents {
    /// A base64-encoded string representing the binary data of the item.
    pub blob: String,
    /// The MIME type of this resource, if known.
    pub mime_type: Option<String>,
    /// The URI of this resource.
    pub uri: String,
}

/// Used by the client to invoke a tool provided by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolRequest {
    pub method: String, // const "tools/call"
    pub params: CallToolParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: Option<HashMap<String, serde_json::Value>>,
}

/// After receiving an initialize request from the client, the server sends this response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(rename = "_meta")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// The server's response to a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<ContentType>,
    pub is_error: Option<bool>,
    #[serde(rename = "_meta")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentType {
    Text(TextContent),
    Image(ImageContent),
    EmbeddedResource(EmbeddedResource),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub image_data: String,
    pub mime_type: String,
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedResource {
    pub resource: Resource,
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: Option<String>,
    pub mime_type: Option<String>,
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

/// Capabilities that a client may support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub roots: Option<RootsCapability>,
    pub sampling: Option<HashMap<String, serde_json::Value>>,
    pub experimental: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootsCapability {
    pub list_changed: bool,
}

/// Capabilities that a server may support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub experimental: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    pub tools: Option<Vec<Tool>>,
    pub prompts: Option<Vec<ResourceTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema object
    pub returns: Option<serde_json::Value>, // JSON Schema object
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTemplate {
    pub name: String,
    pub description: Option<String>,
    pub annotations: Option<Annotations>,
}

/// Represents a root directory or file that the server can operate on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
    pub name: Option<String>,
    pub uri: String,
    pub annotations: Option<Annotations>,
}
