use connections_core::WifiBackend;
use nmrs::NetworkManager;

/// Backend for high-level managing of network connections based
/// around the NetworkManager d-bus API.
#[derive(Clone)]
pub struct NetworkManagerBackend {
    nm: NetworkManager,
}

impl WifiBackend for NetworkManagerBackend {
    async fn list_saved_connections(&self) -> Vec<connections_core::SavedWifiNetwork> {
        todo!()
    }

    async fn list_available_networks(&self) -> Vec<connections_core::WifiNetwork> {
        todo!()
    }

    async fn save_network(
        &self,
        _request: connections_core::SaveNetworkRequest,
    ) -> connections_core::SavedWifiNetwork {
        todo!()
    }

    async fn delete_connection(&self, _id: String) {
        todo!()
    }

    async fn update_priority(
        &self,
        _id: String,
        _priority: i32,
    ) -> connections_core::SavedWifiNetwork {
        todo!()
    }
}
