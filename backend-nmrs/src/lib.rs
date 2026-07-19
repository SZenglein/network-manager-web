use connections_core::WifiBackend;
use nmrs::NetworkManager;

/// Backend for high-level managing of network connections based
/// around the NetworkManager d-bus API.
#[derive(Clone)]
pub struct NetworkManagerBackend {
    nm: NetworkManager,
}

impl WifiBackend for NetworkManagerBackend {
    type Error = nmrs::ConnectionError;

    async fn list_saved_connections(
        &self,
    ) -> Result<Vec<connections_core::SavedWifiNetwork>, Self::Error> {
    }

    async fn list_available_networks(
        &self,
    ) -> Result<Vec<connections_core::WifiNetwork>, Self::Error> {
        todo!()
    }

    async fn save_network(
        &self,
        _request: connections_core::SaveNetworkRequest,
    ) -> Result<connections_core::SavedWifiNetwork, Self::Error> {
        todo!()
    }

    async fn delete_connection(&self, _id: String) -> Result<(), Self::Error> {
        todo!()
    }

    async fn update_priority(
        &self,
        _id: String,
        _priority: i32,
    ) -> Result<connections_core::SavedWifiNetwork, Self::Error> {
        todo!()
    }
}
