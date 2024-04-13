use pdf_extract::extract_text_from_mem;
use docx_rs::read_docx;

pub trait Extract {
    fn extract(data: &[u8]) -> Result<String, anyhow::Error>;
}

pub struct PdfExtractor;
pub struct DocxExtractor;

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
                                        },
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
                            },
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
                },
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