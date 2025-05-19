// crates/owl-application/src/usecase_errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitFirewallError {
    #[error("nftコマンドの起動に失敗: {source}")]
    Spawn {
        #[from]
        source: std::io::Error,
    },

    #[error(
        "nftスクリプトの書き込みに失敗: {source}\n--- script begin ---\n{script}\n--- script end ---"
    )]
    Write {
        source: std::io::Error,
        script: String,
    },

    #[error("nftコマンドが異常終了 (exit code: {code:?}):\n{stderr}")]
    Execution { code: Option<i32>, stderr: String },
}

#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("ファイアウォール初期化エラー: {0}")]
    InitFirewall(#[from] InitFirewallError),

    #[error("設定ファイルの読み込みエラー: {0}")]
    ConfigReadError(#[source] std::io::Error),

    #[error("設定ファイルのパースエラー: {0}")]
    ConfigParseError(#[source] toml::de::Error),

    #[error("interface.address4が設定されていません")]
    Address4NotSet,

    #[error("秘密鍵のデコードに失敗: {0}")]
    KeyDecodeError(#[source] base64::DecodeError),
    // ほかのユースケースエラー...
}
