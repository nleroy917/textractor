use axum::{extract::Multipart, response::Html, Json};

use crate::errors::AppError;
use crate::extraction::{ContentType, DocxExtractor, Extract, PdfExtractor};
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
                let mime_type = ContentType::from(content_type.as_str());

                let start = std::time::Instant::now();
                let text: Option<String> = match mime_type {
                    ContentType::Pdf => Some(PdfExtractor::extract(&data)?),
                    ContentType::MsWord => Some(DocxExtractor::extract(&data)?),
                    ContentType::WordDocument => Some(DocxExtractor::extract(&data)?),
                    ContentType::WordTemplate => Some(DocxExtractor::extract(&data)?),
                    ContentType::WordDocumentMacroEnabled => Some(DocxExtractor::extract(&data)?),
                    ContentType::WordTemplateMacroEnabled => Some(DocxExtractor::extract(&data)?),
                    ContentType::MsExcel => None, // not yet supported
                    ContentType::ExcelSheet => None, // not yet supported
                    ContentType::ExcelTemplate => None, // not yet supported
                    ContentType::ExcelSheetMacroEnabled => None, // not yet supported
                    ContentType::ExcelTemplateMacroEnabled => None, // not yet supported
                    ContentType::ExcelAddInMacroEnabled => None, // not yet supported
                    ContentType::ExcelBinarySheet => None, // not yet supported
                    ContentType::MsPowerPoint => None, // not yet supported
                    ContentType::PowerPointPresentation => None, // not yet supported
                    ContentType::PowerPointTemplate => None, // not yet supported
                    ContentType::PowerPointSlideshow => None, // not yet supported
                    ContentType::PowerPointAddInMacroEnabled => None, // not yet supported
                    ContentType::PowerPointPresentationMacroEnabled => None, // not yet supported
                    ContentType::PowerPointTemplateMacroEnabled => None, // not yet supported
                    ContentType::PowerPointSlideshowMacroEnabled => None, // not yet supported
                    ContentType::MsAccess => None, // not yet supported
                    ContentType::Unknown => None, // not yet supported
                };
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
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/extract" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>
                    <button type="submit">
                        Run textract
                    </button>
                </form>
            </body>
        </html>
        "#,
    )
}
