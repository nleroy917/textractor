use std::io::Write;

use anyhow::Result;
use docx_rs::read_docx;
use pdf_extract::extract_text_from_mem;
use tempfile::NamedTempFile;
use msoffice_pptx::document::PPTXDocument;

use crate::detection::ContentType;

pub trait Extract {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error>;
}

pub struct PdfExtractor;
pub struct DocxExtractor;
pub struct PptxExtractor;
pub struct TxtExtractor;

impl Extract for PdfExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let text = extract_text_from_mem(data)?;
        Ok(text)
    }
}

impl Extract for DocxExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let doc = read_docx(data)?;
        let mut document_text = String::new();
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
                                            document_text.push_str(&text.text);
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
        Ok(document_text)
    }
}

impl Extract for PptxExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let mut file = NamedTempFile::new()?;
        file.write_all(data)?;
        let path = file.into_temp_path();
        let pptx = PPTXDocument::from_file(&path).unwrap();
        let mut text = String::new();
        for (_, slide) in &pptx.slide_map {
            // Do something with slides
            for shape in slide.common_slide_data.shape_tree.shape_array.iter() {
                match shape {
                    msoffice_pptx::pml::ShapeGroup::Shape(s) => {
                        match &s.text_body {
                            Some(text) => {
                                for paragraph in text.paragraph_array.iter() {
                                    for text_run in paragraph.text_run_list.iter() {
                                        match text_run {
                                            _ => {},
                                        }
                                    }
                                }
                            },
                            None => ()
                        }
                    },
                    msoffice_pptx::pml::ShapeGroup::GroupShape(_) => todo!(),
                    msoffice_pptx::pml::ShapeGroup::GraphicFrame(_) => (),
                    msoffice_pptx::pml::ShapeGroup::Connector(_) => (),
                    msoffice_pptx::pml::ShapeGroup::Picture(_) => (),
                    msoffice_pptx::pml::ShapeGroup::ContentPart(_) => (),
                }
            }
          }
        Ok("".to_string())
    }
}

impl Extract for TxtExtractor {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error> {
        let text = String::from_utf8_lossy(data); // losy because we don't care about encoding
        Ok(text.to_string())
    }
}

///
/// Extracts text from a document. This function will attempt to detect the type of document and
/// extract text from it. If the document type is not supported, it will return None.
/// 
/// # Arguments
/// - `data` - The document data to extract text from.
/// 
/// # Returns
/// - `Ok(Some(String))` - The extracted text.
pub fn extract(data: &[u8]) -> Result<Option<String>> {
    let file_type = ContentType::from(data);
    let result = match file_type {
        ContentType::Pdf => Some(PdfExtractor::extract(data)?),
        ContentType::MsWord => Some(DocxExtractor::extract(data)?),
        ContentType::WordDocument => Some(DocxExtractor::extract(data)?),
        ContentType::WordTemplate => Some(DocxExtractor::extract(data)?),
        ContentType::WordDocumentMacroEnabled => Some(DocxExtractor::extract(data)?),
        ContentType::WordTemplateMacroEnabled => Some(DocxExtractor::extract(data)?),
        ContentType::MsExcel => None, // TODO: implement ExcelExtractor
        ContentType::ExcelSheet => None, // TODO: implement ExcelExtractor
        ContentType::ExcelTemplate => None, // TODO: implement ExcelExtractor
        ContentType::ExcelSheetMacroEnabled => None, // TODO: implement ExcelExtractor
        ContentType::ExcelTemplateMacroEnabled => None, // TODO: implement ExcelExtractor
        ContentType::ExcelAddInMacroEnabled => None, // TODO: implement ExcelExtractor
        ContentType::ExcelBinarySheet => None, // TODO: implement ExcelExtractor
        ContentType::MsPowerPoint => None, // TODO: implement PptxExtractor
        ContentType::PowerPointPresentation => None, // TODO: implement PptxExtractor
        ContentType::PowerPointTemplate => None, // TODO: implement PptxExtractor
        ContentType::PowerPointSlideshow => None, // TODO: implement PptxExtractor
        ContentType::PowerPointAddInMacroEnabled => None, // TODO: implement PptxExtractor
        ContentType::PowerPointPresentationMacroEnabled => None, // TODO: implement PptxExtractor
        ContentType::PowerPointTemplateMacroEnabled => None, // TODO: implement PptxExtractor
        ContentType::PowerPointSlideshowMacroEnabled => None, // TODO: implement PptxExtractor
        ContentType::Txt => Some(TxtExtractor::extract(data)?),
        ContentType::Epub => None, // TODO: implement epub extractor
        ContentType::Mobi => None, // TODO: implement epub extractor
        ContentType::Unknown => None,
    };
    Ok(result)
}
