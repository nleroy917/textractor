use pdf_extract::extract_text_from_mem;

pub trait Extract {
    fn extract(data: &[u8]) -> Result<String, String>;
}

pub struct PdfExtractor;

impl Extract for PdfExtractor {
    fn extract(data: &[u8]) -> Result<String, String> {
        extract_text_from_mem(data).map_err(|e| e.to_string())
    }
}