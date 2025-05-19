use ipnet::IpNet;
use serde::Deserialize;
use std::net::SocketAddr;

/// ルート構造体 ─ TOML のトップレベル
#[derive(Debug, Deserialize)]
pub struct OwlConfig {
    pub interface: Interface,
    #[serde(default)]
    pub peers: Vec<Peer>,
    #[serde(default)]
    pub firewall: Option<Firewall>,
    #[serde(default)]
    pub logging: Option<Logging>,
    #[serde(default)]
    pub auth: Option<Auth>,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub listen_port: u16,
    pub private_key: String, // base64
    #[serde(default)]
    pub address6: Option<IpNet>, // fd00::1/64
    #[serde(default)]
    pub address4: Option<IpNet>, // 10.10.0.1/24
    #[serde(default)]
    pub dns: Option<Vec<String>>,
    #[serde(default)]
    pub mtu: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Peer {
    pub public_key: String,
    pub allowed_ips: Vec<IpNet>,
    #[serde(default)]
    pub endpoint: Option<SocketAddr>,
    #[serde(default)]
    pub persistent_keepalive: Option<u16>,
    #[serde(default)]
    pub wg_policy: Option<PeerPolicy>,
}

#[derive(Debug, Deserialize)]
pub struct PeerPolicy {
    pub role: Option<Role>, // "user" | "admin"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    General,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct Firewall {
    pub rules: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub loki_endpoint: String,
    pub tenant_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub issuer: String,
    pub audience: String,
    pub jwks_uri: String,
}
