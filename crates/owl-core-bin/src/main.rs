//! owl‑core‑bin — エントリポイント
//!
//! Axum HTTP サーバを起動し、`owl-api::build_router()` で生成した
//! Router を公開する。

use owl_api::build_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Listens on 0.0.0.0:8080
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind address failed");

    println!(
        "🚀 Owl‑core listening on http://{}",
        listener.local_addr().unwrap()
    );

    // Router 起動
    axum::serve(listener, build_router())
        .await
        .expect("server error");
}
