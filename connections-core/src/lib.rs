use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Operating mode of a saved WiFi connection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Mode {
    /// Station mode - connects to an existing network
    Sta,
    /// Access point mode - creates a hotspot
    Ap,
}

/// Security type of a WiFi network
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SecurityType {
    Open,
    Wpa,
    Wpa2,
    Wpa3,
    WpaEnterprise,
    Wpa2Enterprise,
    Wpa3Enterprise,
    Wep,
    Sae,
}

/// Represents a discovered WiFi network (scan result)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: i32,
    pub frequency: u32,
    pub max_bitrate: u32,
    pub security: SecurityType,
}

/// A saved WiFi connection (either STA or AP mode)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SavedWifiNetwork {
    pub id: String,
    pub ssid: String,
    pub mode: Mode,
    pub frequency: u32,
    pub password: Option<String>,
    pub security: SecurityType,
    pub priority: i32,
}

/// Request body for saving a WiFi network or creating a hotspot
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SaveNetworkRequest {
    pub ssid: String,
    pub mode: Mode,
    pub frequency: u32,
    pub password: Option<String>,
    pub security: SecurityType,
    pub priority: i32,
}

/// Request body for updating the priority of a saved connection
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePriorityRequest {
    pub id: String,
    pub priority: i32,
}

/// Trait for interacting with WiFi networks on the OS level.
/// Implementations can provide different backends (e.g., NetworkManager, wpa_supplicant).
pub trait WifiBackend: Clone + Send + Sync + 'static {
    /// List all saved connections (networks and hotspots)
    fn list_saved_connections(&self) -> impl Future<Output = Vec<SavedWifiNetwork>> + Send;

    /// List available WiFi networks from a scan
    fn list_available_networks(&self) -> impl Future<Output = Vec<WifiNetwork>> + Send;

    /// Save a WiFi network or create a hotspot
    fn save_network(
        &self,
        request: SaveNetworkRequest,
    ) -> impl Future<Output = SavedWifiNetwork> + Send;

    /// Delete a saved connection by ID
    fn delete_connection(&self, id: String) -> impl Future<Output = ()> + Send;

    /// Update the priority of a saved connection
    fn update_priority(
        &self,
        id: String,
        priority: i32,
    ) -> impl Future<Output = SavedWifiNetwork> + Send;
}
