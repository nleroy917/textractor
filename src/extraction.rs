use docx_rs::read_docx;
use pdf_extract::extract_text_from_mem;

pub trait Extract {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error>;
}
pub enum ContentType {
    Pdf,
    MsWord,
    WordDocument,
    WordTemplate,
    WordDocumentMacroEnabled,
    WordTemplateMacroEnabled,
    MsExcel,
    ExcelSheet,
    ExcelTemplate,
    ExcelSheetMacroEnabled,
    ExcelTemplateMacroEnabled,
    ExcelAddInMacroEnabled,
    ExcelBinarySheet,
    MsPowerPoint,
    PowerPointPresentation,
    PowerPointTemplate,
    PowerPointSlideshow,
    PowerPointAddInMacroEnabled,
    PowerPointPresentationMacroEnabled,
    PowerPointTemplateMacroEnabled,
    PowerPointSlideshowMacroEnabled,
    MsAccess,
    Txt,
    Unknown,
}

pub struct PdfExtractor;
pub struct DocxExtractor;
pub struct PptxExtractor;
pub struct TxtExtractor;

impl From<&str> for ContentType {
    fn from(value: &str) -> Self {
        match value {
            // word documents
            "application/msword" => ContentType::MsWord,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                ContentType::WordDocument
            }
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template" => {
                ContentType::WordTemplate
            }
            "application/vnd.ms-word.document.macroEnabled.12" => {
                ContentType::WordDocumentMacroEnabled
            }
            "application/vnd.ms-word.template.macroEnabled.12" => {
                ContentType::WordTemplateMacroEnabled
            }

            // excel documents
            "application/vnd.ms-excel" => ContentType::MsExcel,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                ContentType::ExcelSheet
            }
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template" => {
                ContentType::ExcelTemplate
            }
            "application/vnd.ms-excel.sheet.macroEnabled.12" => ContentType::ExcelSheetMacroEnabled,
            "application/vnd.ms-excel.template.macroEnabled.12" => {
                ContentType::ExcelTemplateMacroEnabled
            }
            "application/vnd.ms-excel.addin.macroEnabled.12" => ContentType::ExcelAddInMacroEnabled,
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => {
                ContentType::ExcelBinarySheet
            }

            // powerpoint
            "application/vnd.ms-powerpoint" => ContentType::MsPowerPoint,
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                ContentType::PowerPointPresentation
            }
            "application/vnd.openxmlformats-officedocument.presentationml.template" => {
                ContentType::PowerPointTemplate
            }
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow" => {
                ContentType::PowerPointSlideshow
            }
            "application/vnd.ms-powerpoint.addin.macroEnabled.12" => {
                ContentType::PowerPointAddInMacroEnabled
            }
            "application/vnd.ms-powerpoint.presentation.macroEnabled.12" => {
                ContentType::PowerPointPresentationMacroEnabled
            }
            "application/vnd.ms-powerpoint.template.macroEnabled.12" => {
                ContentType::PowerPointTemplateMacroEnabled
            }
            "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" => {
                ContentType::PowerPointSlideshowMacroEnabled
            }

            // microsoft access for some reason?
            "application/vnd.ms-access" => ContentType::MsAccess,

            // plain text
            "text/plain" => ContentType::Txt,

            // pdfs
            "application/pdf" => ContentType::Pdf,

            _ => ContentType::Unknown,
        }
    }
}

impl Extract for PdfExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let text = extract_text_from_mem(data)?;
        Ok(text)
    }
}

impl Extract for DocxExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let doc = read_docx(data)?;
        let mut documetn_text = String::new();
        let children = doc.document.children;
        for child in children {
            match child {
                docx_rs::DocumentChild::Paragraph(paragraph) => {
                    for child in paragraph.children {
                        match child {
                            docx_rs::ParagraphChild::Run(run) => {
                                for child in run.children {
                                    match child {
                                        docx_rs::RunChild::Text(text) => {
                                            documetn_text.push_str(&text.text);
                                        }
                                        docx_rs::RunChild::Sym(_) => (),
                                        docx_rs::RunChild::DeleteText(_) => (),
                                        docx_rs::RunChild::Tab(_) => (),
                                        docx_rs::RunChild::Break(_) => (),
                                        docx_rs::RunChild::Drawing(_) => (),
                                        docx_rs::RunChild::Shape(_) => (),
                                        docx_rs::RunChild::CommentStart(_) => (),
                                        docx_rs::RunChild::CommentEnd(_) => (),
                                        docx_rs::RunChild::FieldChar(_) => (),
                                        docx_rs::RunChild::InstrText(_) => (),
                                        docx_rs::RunChild::DeleteInstrText(_) => (),
                                        docx_rs::RunChild::InstrTextString(_) => (),
                                    }
                                }
                            }
                            docx_rs::ParagraphChild::Insert(_) => (),
                            docx_rs::ParagraphChild::Delete(_) => (),
                            docx_rs::ParagraphChild::BookmarkStart(_) => (),
                            docx_rs::ParagraphChild::Hyperlink(_) => (),
                            docx_rs::ParagraphChild::BookmarkEnd(_) => (),
                            docx_rs::ParagraphChild::CommentStart(_) => (),
                            docx_rs::ParagraphChild::CommentEnd(_) => (),
                            docx_rs::ParagraphChild::StructuredDataTag(_) => (),
                        }
                    }
                }
                docx_rs::DocumentChild::Table(_) => (),
                docx_rs::DocumentChild::BookmarkStart(_) => (),
                docx_rs::DocumentChild::BookmarkEnd(_) => (),
                docx_rs::DocumentChild::CommentStart(_) => (),
                docx_rs::DocumentChild::CommentEnd(_) => (),
                docx_rs::DocumentChild::StructuredDataTag(_) => (),
                docx_rs::DocumentChild::TableOfContents(_) => (),
            }
        }
        Ok(documetn_text)
    }
}

impl Extract for PptxExtractor {
    fn extract(_data: &[u8]) -> Result<String, anyhow::Error> {
        Ok("".to_string())
    }
}

impl Extract for TxtExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let text = String::from_utf8_lossy(data); // losy because we don't care about encoding
        Ok(text.to_string())
    }
}