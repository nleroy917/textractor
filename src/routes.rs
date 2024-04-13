use axum::{extract::Multipart, Json};

use crate::errors::AppError;
use crate::extraction::{DocxExtractor, Extract, PdfExtractor};
use crate::models::{ExtractionResponse, ExtractionResult, ServerInfo};

pub async fn root() -> Json<ServerInfo> {
    let info = ServerInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        message: "Welcome to the textractor API".to_string(),
    };
    Json(info)
}

pub async fn extract(mut multipart: Multipart) -> Result<Json<ExtractionResponse>, AppError> {
    let mut extracted_text: Vec<ExtractionResult> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await?;

        match content_type.as_str() {
            "application/pdf" => {
                let text = PdfExtractor::extract(&data)?;
                extracted_text.push(ExtractionResult {
                    success: true,
                    name,
                    file_name,
                    content_type,
                    text,
                });
            },
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                let text = DocxExtractor::extract(&data)?;
                extracted_text.push(ExtractionResult {
                    success: true,
                    name,
                    file_name,
                    content_type,
                    text,
                });
            }
            _ => {
                extracted_text.push(ExtractionResult {
                    success: false,
                    name,
                    file_name,
                    content_type,
                    text: "Unsupported content type".to_string(),
                });
            }
        }
    }

    Ok(Json(ExtractionResponse {
        results: extracted_text,
    }))
}
