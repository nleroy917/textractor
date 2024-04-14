use axum::{
    routing::{get, post},
    Router,
    extract::DefaultBodyLimit
};

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024; // 20MB

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
        // `GET /` goes to `root`
        .route("/", get(routes::root))
        .route("/extract", post(routes::extract))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .route("/test", get(routes::show_form));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
