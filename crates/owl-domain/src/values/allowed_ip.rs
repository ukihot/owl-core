use ipnet::IpNet;

/// 許可IP（CIDR付き）
/// ドメイン層では文字列ではなく構造体で扱う
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedIp(pub IpNet);
