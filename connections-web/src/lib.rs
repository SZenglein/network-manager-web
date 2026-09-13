use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use connections_core::*;
use thiserror::Error;
use utoipa::OpenApi;

/// Route path constants - defined once to prevent divergence between
/// OpenAPI documentation and actual axum route definitions.
const ROUTE_SAVED: &str = "/connections/saved";
const ROUTE_AVAILABLE: &str = "/connections/available";
const ROUTE_SAVED_ID: &str = "/connections/saved/{id}";
const ROUTE_SAVED_ID_ACTIVATE: &str = "/connections/saved/{id}/activate";

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ApiError(#[source] pub Box<dyn std::error::Error>);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.0)).into_response()
    }
}

#[derive(Clone)]
pub struct AppState<B: WifiBackend> {
    backend: B,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        list_saved_connections,
        list_available_networks,
        delete_connection,
        save_network,
        activate_connection,
    ),
    components(
        schemas(
            WifiNetworkAp,
            SavedWifiNetwork,
            SaveNetworkRequest,
            Mode,
        )
    ),
    tags(
        (name = "wifi", description = "WiFi network management API"),
    ),
)]
pub struct ApiDoc;

/// List all saved connections (networks and hotspots)
#[utoipa::path(
    tag = "wifi",
    get,
    path = ROUTE_SAVED,
    responses(
        (status = 200, description = "List of saved connections", body = [SavedWifiNetwork]),
    )
)]
async fn list_saved_connections<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
) -> Result<Json<Vec<SavedWifiNetwork>>, ApiError> {
    let connections = backend
        .list_saved_connections()
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok(Json(connections))
}

/// List available WiFi networks from a scan
#[utoipa::path(
    tag = "wifi",
    get,
    path = ROUTE_AVAILABLE,
    responses(
        (status = 200, description = "List of available WiFi networks", body = [WifiNetworkAp]),
    )
)]
async fn list_available_networks<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
) -> Result<Json<Vec<WifiNetworkAp>>, ApiError> {
    let networks = backend
        .list_available_networks()
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok(Json(networks))
}

/// Delete a saved connection by ID
#[utoipa::path(
    tag = "wifi",
    delete,
    path = ROUTE_SAVED_ID,
    params(
        ("id" = String, Path, description = "Connection ID to delete"),
    ),
    responses(
        (status = 204, description = "Connection deleted successfully"),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_connection<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
    Path(_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    backend
        .delete_connection(_id)
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok(StatusCode::NO_CONTENT)
}

/// Save a WiFi network or create a hotspot
#[utoipa::path(
    tag = "wifi",
    post,
    path = ROUTE_SAVED,
    request_body = SaveNetworkRequest,
    responses(
        (status = 201, description = "Connection saved successfully", body = SavedWifiNetwork),
        (status = 500, description = "Internal server error")
    )
)]
async fn save_network<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
    Json(request): Json<SaveNetworkRequest>,
) -> Result<(StatusCode, ()), ApiError> {
    backend
        .save_network(request)
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok((StatusCode::CREATED, ()))
}

/// Activate a saved connection by ID (UUID)
#[utoipa::path(
    tag = "wifi",
    post,
    path = ROUTE_SAVED_ID_ACTIVATE,
    params(
        ("id" = String, Path, description = "Connection ID (UUID) to activate"),
    ),
    responses(
        (status = 200, description = "Connection activated successfully"),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn activate_connection<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    backend
        .activate_connection(id)
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok(StatusCode::OK)
}

/// Build the API router with the given backend implementation.
pub fn router<B: WifiBackend>(backend: B) -> Router {
    let state = AppState { backend };

    Router::new()
        .route(ROUTE_SAVED, get(list_saved_connections::<B>))
        .route(ROUTE_AVAILABLE, get(list_available_networks::<B>))
        .route(ROUTE_SAVED_ID, delete(delete_connection::<B>))
        .route(ROUTE_SAVED, post(save_network::<B>))
        .route(ROUTE_SAVED_ID_ACTIVATE, post(activate_connection::<B>))
        .with_state(state)
}

/// The openapi documentation for `connections-web`.
///
/// Meant to be nested under your global api documentation, but can be used standalone.
pub fn openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
