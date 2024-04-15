use axum::{extract::Multipart, response::Html, Json};

use crate::errors::AppError;
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
        let data = field.bytes().await;

        match data {
            Ok(data) => {
                let start = std::time::Instant::now();
                let text = textractor::extraction::extract(&data)?;
                let elapsed = start.elapsed();

                match text {
                    Some(text) => {
                        extracted_text.push(ExtractionResult {
                            extraction_time: elapsed.as_secs_f32(),
                            success: true,
                            name,
                            file_name,
                            content_type,
                            text: Some(text),
                            error: None,
                        });
                    }
                    None => {
                        extracted_text.push(ExtractionResult {
                            extraction_time: elapsed.as_secs_f32(),
                            success: false,
                            name,
                            file_name,
                            content_type,
                            text: None,
                            error: Some("Unsupported file type".to_string()),
                        });
                    }
                }
            }
            Err(err) => {
                extracted_text.push(ExtractionResult {
                    extraction_time: 0.0,
                    success: false,
                    name,
                    file_name,
                    content_type,
                    text: None,
                    error: Some(err.to_string()),
                });
            }
        }
    }

    Ok(Json(ExtractionResponse {
        results: extracted_text,
    }))
}

pub async fn show_form() -> Html<&'static str> {
    // return an htnl file from ../assets/form.html
    Html(include_str!("../assets/form.html"))
}
