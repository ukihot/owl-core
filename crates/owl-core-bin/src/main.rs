//! owl‑core‑bin — エントリポイント

use anyhow::Result;
use owl_application::OwlApplication;
use owl_presentation::handlers::build_router;
use owl_presentation::presenters::{
    ConsoleConfigPresenter, ConsoleFirewallPresenter, ConsoleVpnPresenter,
};
use std::path::Path;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config_presenter = ConsoleConfigPresenter;
    let mut firewall_presenter = ConsoleFirewallPresenter;
    let mut vpn_presenter = ConsoleVpnPresenter;

    let mut app = OwlApplication::new(
        &mut config_presenter,
        &mut firewall_presenter,
        &mut vpn_presenter,
    );

    app.init(Path::new("Owl.toml")).await?;
    app.spawn_vpn_runtime().await?;

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Owl‑core listening on http://{}", listener.local_addr()?);
    axum::serve(listener, build_router()?).await?;

    Ok(())
}
