use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use connections_core::*;
use thiserror::Error;
use utoipa::OpenApi;

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
        update_priority,
        save_network,
    ),
    components(
        schemas(
            WifiNetwork,
            SavedWifiNetwork,
            SaveNetworkRequest,
            UpdatePriorityRequest,
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
    path = "/api/connections",
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
    path = "/api/networks/available",
    responses(
        (status = 200, description = "List of available WiFi networks", body = [WifiNetwork]),
    )
)]
async fn list_available_networks<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
) -> Result<Json<Vec<WifiNetwork>>, ApiError> {
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
    path = "/api/connections/{id}",
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

/// Update the priority of a saved connection
#[utoipa::path(
    tag = "wifi",
    put,
    path = "/api/connections/{id}/priority",
    params(
        ("id" = String, Path, description = "Connection ID to update"),
    ),
    request_body = UpdatePriorityRequest,
    responses(
        (status = 200, description = "Priority updated successfully", body = SavedWifiNetwork),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_priority<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
    Path(_id): Path<String>,
    Json(request): Json<UpdatePriorityRequest>,
) -> Result<(StatusCode, Json<SavedWifiNetwork>), ApiError> {
    let connection = backend
        .update_priority(request.id, request.priority)
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok((StatusCode::OK, Json(connection)))
}

/// Save a WiFi network or create a hotspot
#[utoipa::path(
    tag = "wifi",
    post,
    path = "/api/connections",
    request_body = SaveNetworkRequest,
    responses(
        (status = 201, description = "Connection saved successfully", body = SavedWifiNetwork),
        (status = 500, description = "Internal server error")
    )
)]
async fn save_network<B: WifiBackend>(
    State(AppState { backend }): State<AppState<B>>,
    Json(request): Json<SaveNetworkRequest>,
) -> Result<(StatusCode, Json<SavedWifiNetwork>), ApiError> {
    let network = backend
        .save_network(request)
        .await
        .map_err(|e| ApiError(Box::new(e)))?;
    Ok((StatusCode::CREATED, Json(network)))
}

/// Build the API router with the given backend implementation.
pub fn router<B: WifiBackend>(backend: B) -> Router<AppState<B>> {
    let state = AppState { backend };

    Router::new()
        .route("/api/connections", get(list_saved_connections::<B>))
        .route("/api/connections/{id}", delete(delete_connection::<B>))
        .route("/api/connections/{id}/priority", put(update_priority::<B>))
        .route("/api/networks/available", get(list_available_networks::<B>))
        .route("/api/connections", post(save_network::<B>))
        .route("/api-docs/openapi.json", get(get_openapi))
        .with_state(state)
}

async fn get_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
