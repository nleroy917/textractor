use infer;

#[derive(Debug, PartialEq)]
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
    Txt,
    Epub,
    Mobi,
    Unknown,
}

impl From<&[u8]> for ContentType {
    fn from(value: &[u8]) -> Self {
        let file_type = infer::get(value);
        if let Some(file_type) = file_type {
            match file_type.mime_type() {
                "application/pdf" => ContentType::Pdf,
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
                "application/vnd.ms-excel" => ContentType::MsExcel,
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                    ContentType::ExcelSheet
                }
                "application/vnd.openxmlformats-officedocument.spreadsheetml.template" => {
                    ContentType::ExcelTemplate
                }
                "application/vnd.ms-excel.sheet.macroEnabled.12" => {
                    ContentType::ExcelSheetMacroEnabled
                }
                "application/vnd.ms-excel.template.macroEnabled.12" => {
                    ContentType::ExcelTemplateMacroEnabled
                }
                "application/vnd.ms-excel.addin.macroEnabled.12" => {
                    ContentType::ExcelAddInMacroEnabled
                }
                "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => {
                    ContentType::ExcelBinarySheet
                }
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
                "text/plain" => ContentType::Txt,
                "application/epub+zip" => ContentType::Epub,
                "application/x-mobipocket-ebook" => ContentType::Mobi,
                _ => ContentType::Unknown,
            }
        } else {
            ContentType::Unknown
        }
    }
}
