use axum::body::Bytes;

use crate::models::ExtractionResult;

pub async fn process_extraction(
    data: &Bytes,
    name: String,
    file_name: String,
    content_type: String,
) -> Result<ExtractionResult, String> {
    let start = std::time::Instant::now();
    let text =
        textractor::extraction::extract(data).map_err(|e| format!("Extraction failed: {}", e))?;
    let elapsed = start.elapsed();

    let result = match text {
        Some(text) => ExtractionResult {
            extraction_time: elapsed.as_secs_f32(),
            success: true,
            name,
            file_name,
            content_type,
            text: Some(text),
            error: None,
        },
        None => ExtractionResult {
            extraction_time: elapsed.as_secs_f32(),
            success: false,
            name,
            file_name,
            content_type,
            text: None,
            error: Some("Unsupported file type".to_string()),
        },
    };

    Ok(result)
}
