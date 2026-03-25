use serde_json::Value;

#[derive(Debug, Clone)]
pub enum CoreCommand {
    ExecuteApi(ApiRequest),
    ConnectWebSocket { url: String },
    DisconnectWebSocket,
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub label: String,
    pub method: HttpMethod,
    pub base_url: String,
    pub path: String,
    pub body: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ApiResponseEvent {
    pub label: String,
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum CoreEvent {
    ApiResponse(ApiResponseEvent),
    WebSocketConnected,
    WebSocketDisconnected { reason: String },
    WebSocketFrame(Vec<u8>),
    Log(String),
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
}
