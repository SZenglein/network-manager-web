use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request body for saving a WiFi network or creating a hotspot
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SaveNetworkRequest {
    pub ssid: String,
    pub mode: Mode,
    pub band: WifiBand,
    pub security: WifiSecurity,
    pub autoconnect: bool,
    /// The priority to use when multiple networks are configured to autoconnect. Higher values connect first.
    pub autoconnect_priority: i32,
}

/// A saved WiFi connection, which can also be a Hotspot.
///
/// Reading back saved passwords is not yet supported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SavedWifiNetwork {
    pub id: String,
    pub ssid: String,
    pub mode: Mode,
    pub band: WifiBand,
    pub security: WifiSecurityType,
    pub autoconnect: bool,
    /// The priority to use when multiple networks are configured to autoconnect. Higher values connect first.
    pub autoconnect_priority: i32,
}

/// Represents a discovered WiFi network (scan result)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WifiNetworkAp {
    pub ssid: String,
    pub bssid: String,
    // Signal strengh between 0-100
    pub signal_strength: u8,
    pub frequency: u32,
    // List of security modes the network exposes.
    pub security: Vec<WifiSecurityType>,
}

/// Security settings of a WiFi network, primarily used when adding a network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum WifiSecurity {
    /// An open network without security
    Open,
    /// A password-protected network that will automatically be configured using WPA2/WPA3
    WpaPsk { password: String },
}

/// The band of the network. 5GHz or 2.4GHz.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum WifiBand {
    /// Automatic selection or both 2.4GHz and 5GHz.
    Auto,
    /// The 2.4GHz band, also called 802.11bg
    Ghz2_4,
    /// The 5GHz band, also called 802.11a
    Ghz5,
}

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
pub enum WifiSecurityType {
    Open,
    Wep,
    Wpa2Personal,
    Wpa3Personal,
    Enterprise,
}

/// Trait for interacting with WiFi networks on the OS level.
/// Implementations can provide different backends (e.g., NetworkManager, wpa_supplicant).
pub trait WifiBackend: Clone + Send + Sync + 'static {
    type Error: std::error::Error;

    /// List all saved connections (networks and hotspots)
    fn list_saved_connections(
        &self,
    ) -> impl Future<Output = Result<Vec<SavedWifiNetwork>, Self::Error>> + Send;

    /// List available WiFi networks from a scan
    fn list_available_networks(
        &self,
    ) -> impl Future<Output = Result<Vec<WifiNetworkAp>, Self::Error>> + Send;

    /// Save a WiFi network or create a hotspot
    fn save_network(
        &self,
        request: SaveNetworkRequest,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Delete a saved connection by ID
    fn delete_connection(&self, id: String)
    -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Activate a saved connection by ID (UUID)
    fn activate_connection(
        &self,
        id: String,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
