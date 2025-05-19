use anyhow::Result;
use owl_infra::OwlConfig;
use std::path::Path;
use tokio::fs;

mod app_errors;
use app_errors::AppError;

/// 起動処理
pub async fn setup(config_path: &Path) -> Result<()> {
    // 0) 設定読込
    let conf = load_config(config_path).await?;
    println!("✔ config loaded: {:?}", conf);

    // 1) ファイアウォール初期化
    init_firewall(&conf).await?;
    println!("✔ firewall ready");

    // 2) VPN スタック (WireGuard + TUN) 準備
    init_vpn_stack(&conf).await?;
    println!("✔ VPN stack ready");

    Ok(())
}

async fn load_config(path: &Path) -> Result<OwlConfig, AppError> {
    let data = fs::read_to_string(path)
        .await
        .map_err(AppError::ConfigReadError)?;
    let config: OwlConfig = toml::from_str(&data).map_err(AppError::ConfigParseError)?;
    Ok(config)
}

/* ------- 以下は当面ダミー実装 ------- */
async fn init_firewall(_conf: &OwlConfig) -> Result<()> {
    // nftables / iptables 呼び出し予定
    Ok(())
}

async fn init_vpn_stack(_conf: &OwlConfig) -> Result<()> {
    // wireguard‑rs と tun crate を組み込み予定
    Ok(())
}
