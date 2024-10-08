use axum::{extract::Multipart, response::Html, Json};

use crate::errors::AppError;
use crate::extraction::process_extraction;
use crate::models::{ExtractionResponse, ExtractionResult, ServerInfo};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List server info", body = [ServerInfo])
    )
)]
pub async fn root() -> Json<ServerInfo> {
    let info = ServerInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        message: "Welcome to the textractor API".to_string(),
    };
    Json(info)
}

#[utoipa::path(
    post,
    path = "/extract",
    request_body(
        content_type = "multipart/form-data",
        content = FileUpload
    ),
    responses(
        (status = 200, description = "Result of a file extraction.", body = [ExtractionResponse])
    )
)]
pub async fn extract(mut multipart: Multipart) -> Result<Json<ExtractionResponse>, AppError> {
    let mut extracted_text: Vec<ExtractionResult> = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("Unnamed field").to_string();
        let file_name = field.file_name().unwrap_or("Unnamed file").to_string();
        let content_type = field.content_type().map_or_else(
            || "application/octet-stream".to_string(),
            |ct| ct.to_string(),
        );

        match field.bytes().await {
            Ok(data) => {
                match process_extraction(
                    &data,
                    name.clone(),
                    file_name.clone(),
                    content_type.clone(),
                )
                .await
                {
                    Ok(result) => extracted_text.push(result),
                    Err(err) => {
                        // Log the error if necessary
                        extracted_text.push(ExtractionResult {
                            extraction_time: 0.0,
                            success: false,
                            name,
                            file_name,
                            content_type,
                            text: None,
                            error: Some(err),
                        });
                    }
                }
            }
            Err(err) => {
                // Log the error if necessary
                extracted_text.push(ExtractionResult {
                    extraction_time: 0.0,
                    success: false,
                    name,
                    file_name,
                    content_type,
                    text: None,
                    error: Some(format!("Failed to read file bytes: {}", err)),
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
