use serde::Serialize;

#[derive(Serialize)]
pub struct ServerInfo {
    pub version: String,
    pub name: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExtractionResult {
    pub success: bool,
    pub name: String,
    pub file_name: String,
    pub content_type: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct ExtractionResponse {
    pub results: Vec<ExtractionResult>,
}