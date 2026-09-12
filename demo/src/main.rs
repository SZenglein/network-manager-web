use axum::Router;
use backend_nmrs::NetworkManagerBackend;
use connections_web::router as api_router;
use nmrs::NetworkManager;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nm = NetworkManager::new().await?;
    let networks = nm.list_saved_connections().await?;

    println!("{:?}", networks);

    let backend = NetworkManagerBackend::new().await?;

    // Use the connections_web router for API routes
    let api = api_router(backend);

    // Nest the API under /api
    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new("demo/static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on http://{}:", addr);
    println!("  API: http://{}/api/connections", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
