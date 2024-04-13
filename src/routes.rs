use axum::{extract::Multipart, Json};

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
        let data = field.bytes().await?;

        let mime_type = ContentType::from(content_type.as_str());

        let text: Option<String> = match mime_type {
            ContentType::Pdf => Some(PdfExtractor::extract(&data)?),
            ContentType::MsWord => Some(DocxExtractor::extract(&data)?),
            ContentType::WordDocument => Some(DocxExtractor::extract(&data)?),
            ContentType::WordTemplate => Some(DocxExtractor::extract(&data)?),
            ContentType::WordDocumentMacroEnabled => Some(DocxExtractor::extract(&data)?),
            ContentType::WordTemplateMacroEnabled => Some(DocxExtractor::extract(&data)?),
            ContentType::MsExcel => todo!(),
            ContentType::ExcelSheet => todo!(),
            ContentType::ExcelTemplate => todo!(),
            ContentType::ExcelSheetMacroEnabled => todo!(),
            ContentType::ExcelTemplateMacroEnabled => todo!(),
            ContentType::ExcelAddInMacroEnabled => todo!(),
            ContentType::ExcelBinarySheet => todo!(),
            ContentType::MsPowerPoint => todo!(),
            ContentType::PowerPointPresentation => todo!(),
            ContentType::PowerPointTemplate => todo!(),
            ContentType::PowerPointSlideshow => todo!(),
            ContentType::PowerPointAddInMacroEnabled => todo!(),
            ContentType::PowerPointPresentationMacroEnabled => todo!(),
            ContentType::PowerPointTemplateMacroEnabled => todo!(),
            ContentType::PowerPointSlideshowMacroEnabled => todo!(),
            ContentType::MsAccess => todo!(),
            ContentType::Unknown => None,
        };

        match text {
            Some(text) => {
                extracted_text.push(ExtractionResult {
                    success: true,
                    name,
                    file_name,
                    content_type,
                    text,
                });
            }
            None => {
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
