use std::io::{Cursor, Read};
use zip::ZipArchive;
use xml::reader::{EventReader, XmlEvent};
use anyhow::{Context, Result};
use docx_rs::read_docx;
use pdf_extract::extract_text_from_mem;

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
       // Open the PPTX file as a zip archive
       let cursor = Cursor::new(data);
       let mut archive = ZipArchive::new(cursor).context("Failed to read ZIP archive")?;

       // Collect text from each slide
       let mut result_text = String::new();
       for i in 0..archive.len() {
           let mut file = archive.by_index(i).context("Failed to access file in ZIP archive")?;
           if file.name().starts_with("ppt/slides/") && file.name().ends_with(".xml") {
               let mut content = String::new();
               file.read_to_string(&mut content).context("Failed to read slide content")?;

               // Parse the XML content of each slide
               let parser = EventReader::new(content.as_bytes());
               let mut is_text = false;

               for event in parser {
                   match event {
                       Ok(XmlEvent::StartElement { name, .. }) => {
                           if name.local_name == "t" {
                               is_text = true;
                           }
                       }
                       Ok(XmlEvent::Characters(chars)) => {
                           if is_text {
                               result_text.push_str(&chars);
                               result_text.push(' ');
                               is_text = false;
                           }
                       }
                       Ok(XmlEvent::EndElement { name }) => {
                           if name.local_name == "p" {
                               result_text.push('\n');
                           }
                       }
                       Err(e) => {
                           return Err(anyhow::Error::new(e).context("Failed to parse XML"));
                       }
                       _ => {}
                   }
               }
           }
       }

       Ok(result_text)
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
        ContentType::MsPowerPoint => Some(PptxExtractor::extract(data)?), // TODO: implement PptxExtractor
        ContentType::PowerPointPresentation => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointTemplate => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointSlideshow => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointAddInMacroEnabled => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointPresentationMacroEnabled => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointTemplateMacroEnabled => Some(PptxExtractor::extract(data)?),
        ContentType::PowerPointSlideshowMacroEnabled => Some(PptxExtractor::extract(data)?),
        ContentType::Txt => Some(TxtExtractor::extract(data)?),
        ContentType::Epub => None, // TODO: implement epub extractor
        ContentType::Mobi => None, // TODO: implement epub extractor
        ContentType::Unknown => None,
    };
    Ok(result)
}
