use utoipa::OpenApi;

use crate::models;
use crate::routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::root,
        routes::extract,
    ),
    components(
        schemas(
            models::ServerInfo,
            models::ExtractionResult,
            models::ExtractionResponse,
            models::FileUpload
        )
    ),
    tags(
        (name = "textractor", description = "API for extracting text from files")
    )
)]
pub struct ApiDoc;
