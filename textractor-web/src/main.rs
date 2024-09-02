use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, MatchedPath},
    http::{HeaderMap, Request},
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer, self};
use tracing::{info_span, Span, Level};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const CONTENT_LENGTH_LIMIT: usize = 256 * 1024 * 1024; // 256 mb

pub mod docs;
pub mod errors;
pub mod extraction;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", docs::ApiDoc::openapi()))
        // `GET /` goes to `root`
        .route("/", get(routes::root))
        .route("/extract", post(routes::extract))
        .route("/test", get(routes::show_form))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO))
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
