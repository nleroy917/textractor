use axum::{routing::{get, post}, Router};

pub mod extraction;
pub mod routes;
pub mod models;

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
        .route("/extract", post(routes::extract));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}