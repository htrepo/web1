use axum::Router;
use crate::web::router::router;
use tower_http::trace::TraceLayer;

/// Binds the TCP listener and starts serving.
pub async fn start() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app: Router = router().layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
