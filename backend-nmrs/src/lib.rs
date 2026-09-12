use connections_core::{
    Mode, SaveNetworkRequest, SavedWifiNetwork, WifiBackend, WifiBand, WifiNetworkAp, WifiSecurity,
    WifiSecurityType,
};
use nmrs::{NetworkManager, builders::WifiConnectionBuilder};

/// Backend for high-level managing of network connections based
/// around the NetworkManager d-bus API.
#[derive(Clone)]
pub struct NetworkManagerBackend {
    nm: NetworkManager,
}

impl NetworkManagerBackend {
    /// Create a new backend with a NetworkManager connection
    #[allow(clippy::result_large_err)]
    pub async fn new() -> Result<Self, nmrs::ConnectionError> {
        Ok(Self {
            nm: NetworkManager::new().await?,
        })
    }

    /// Map nmrs WifiKeyMgmt to our SecurityType
    fn map_security_type(security: &nmrs::WifiSecuritySummary) -> WifiSecurityType {
        match security.key_mgmt {
            nmrs::WifiKeyMgmt::None => WifiSecurityType::Open,
            nmrs::WifiKeyMgmt::Wep => WifiSecurityType::Wep,
            nmrs::WifiKeyMgmt::WpaPsk => WifiSecurityType::Wpa2Personal,
            nmrs::WifiKeyMgmt::WpaEap => WifiSecurityType::Enterprise,
            nmrs::WifiKeyMgmt::Sae => WifiSecurityType::Wpa3Personal,
            nmrs::WifiKeyMgmt::Owe => WifiSecurityType::Wpa3Personal,
            nmrs::WifiKeyMgmt::OweTransitionMode => WifiSecurityType::Wpa3Personal,
            _ => WifiSecurityType::Open,
        }
    }

    /// Map nmrs Mode string to our Mode enum
    fn map_mode(mode_str: &Option<String>) -> Mode {
        match mode_str.as_deref() {
            Some("ap") | Some("hotspot") => Mode::Ap,
            _ => Mode::Sta,
        }
    }

    /// Map SecurityFeatures to SecurityType for access points
    fn map_security_features(features: &nmrs::SecurityFeatures) -> Vec<WifiSecurityType> {
        let mut security = Vec::new();

        if features.eap || features.eap_suite_b_192 {
            security.push(WifiSecurityType::Enterprise);
        }

        if features.sae {
            security.push(WifiSecurityType::Wpa3Personal);
        } else if features.psk {
            security.push(WifiSecurityType::Wpa2Personal);
        }

        if features.wep40 || features.wep104 {
            security.push(WifiSecurityType::Wep);
        }

        if features.owe || features.owe_transition_mode || !features.privacy {
            security.push(WifiSecurityType::Open);
        }

        security
    }
}

impl WifiBackend for NetworkManagerBackend {
    type Error = nmrs::ConnectionError;

    async fn list_saved_connections(&self) -> Result<Vec<SavedWifiNetwork>, Self::Error> {
        let saved = self.nm.list_saved_connections().await?;

        Ok(saved
            .into_iter()
            .filter_map(|conn| {
                if conn.connection_type != "802-11-wireless" {
                    return None;
                }

                let (ssid, mode_str, security, band) = match conn.summary {
                    nmrs::SettingsSummary::Wifi {
                        ssid,
                        mode,
                        security,
                        band,
                        channel: _,
                        bssid: _,
                        hidden: _,
                        mac_randomization: _,
                    } => Some((ssid, mode, security, band)),
                    _ => None,
                }?;

                let security_type = security
                    .map(|sec| Self::map_security_type(&sec))
                    .unwrap_or(WifiSecurityType::Open);

                let band = match &band.as_deref() {
                    None => WifiBand::Auto,
                    Some("a") => WifiBand::Ghz5,
                    Some("bg") => WifiBand::Ghz2_4,
                    _ => WifiBand::Auto,
                };

                let autoconnect = conn.autoconnect;
                let autoconnect_priority = conn.autoconnect_priority;

                Some(SavedWifiNetwork {
                    id: conn.uuid,
                    ssid,
                    mode: Self::map_mode(&mode_str),
                    band,
                    security: security_type,
                    autoconnect,
                    autoconnect_priority,
                })
            })
            .collect())
    }

    async fn list_available_networks(&self) -> Result<Vec<WifiNetworkAp>, Self::Error> {
        let aps = self.nm.list_access_points(None).await?;

        Ok(aps
            .into_iter()
            .map(|ap| {
                let security_type = Self::map_security_features(&ap.security);

                WifiNetworkAp {
                    ssid: ap.ssid,
                    bssid: ap.bssid,
                    signal_strength: ap.strength,
                    frequency: ap.frequency_mhz,
                    security: security_type,
                }
            })
            .collect())
    }

    async fn save_network(&self, request: SaveNetworkRequest) -> Result<(), Self::Error> {
        let SaveNetworkRequest {
            ssid,
            mode,
            band,
            security,
            autoconnect,
            autoconnect_priority,
        } = request;

        let builder = WifiConnectionBuilder::new(ssid);

        let builder = match mode {
            Mode::Sta => builder
                .mode(nmrs::builders::WifiMode::Infrastructure)
                .ipv4_auto()
                .ipv6_auto(),
            Mode::Ap => builder
                .mode(nmrs::builders::WifiMode::Ap)
                .ipv4_shared()
                .ipv6_ignore(),
        };

        let builder = match security {
            WifiSecurity::Open => builder.open(),
            WifiSecurity::WpaPsk { password } => builder.wpa_psk(password),
        };

        let builder = match band {
            connections_core::WifiBand::Auto => builder,
            connections_core::WifiBand::Ghz2_4 => builder.band(nmrs::builders::WifiBand::Bg),
            connections_core::WifiBand::Ghz5 => builder.band(nmrs::builders::WifiBand::Bg),
        };

        let builder = builder
            .autoconnect(autoconnect)
            .autoconnect_priority(autoconnect_priority);

        let settings = builder.build();
        let _added = self.nm.add_connection(settings).await?;

        Ok(())
    }

    async fn delete_connection(&self, id: String) -> Result<(), Self::Error> {
        self.nm.delete_saved_connection(&id).await
    }

    async fn activate_connection(&self, id: String) -> Result<(), Self::Error> {
        use nmrs::ConnectByUuidConfig;

        self.nm
            .connect_by_uuid(&id, ConnectByUuidConfig::default())
            .await
    }
}
