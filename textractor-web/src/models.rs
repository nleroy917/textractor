use serde::Serialize;

#[derive(Serialize)]
pub struct ServerInfo {
    pub version: String,
    pub name: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExtractionResult {
    pub extraction_time: f32,
    pub success: bool,
    pub name: String,
    pub file_name: String,
    pub content_type: String,
    pub text: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ExtractionResponse {
    pub results: Vec<ExtractionResult>,
}
