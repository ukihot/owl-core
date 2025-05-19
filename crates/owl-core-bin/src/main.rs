//! owl‑core‑bin — エントリポイント

use anyhow::Result;
use owl_api::build_router;
use owl_application::AppBuilder;
use owl_presentation::{ConsoleConfigPresenter, ConsoleFirewallPresenter, ConsoleVpnPresenter};
use std::path::Path;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // VPN エンジン初期化シーケンス
    let mut config_presenter = ConsoleConfigPresenter;
    let mut fw_presenter = ConsoleFirewallPresenter;
    let mut vpn_presenter = ConsoleVpnPresenter;

    AppBuilder {
        config_presenter: &mut config_presenter,
        fw_presenter: &mut fw_presenter,
        vpn_presenter: &mut vpn_presenter,
    }
    .init(Path::new("Owl.toml"))
    .await?;

    // HTTP API サーバ起動
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Owl‑core listening on http://{}", listener.local_addr()?);
    axum::serve(listener, build_router()?).await?;

    Ok(())
}
