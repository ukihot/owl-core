use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("設定ファイルの読み込みに失敗: {0}")]
    ConfigReadError(#[source] std::io::Error),
    #[error("設定ファイルのパースに失敗: {0}")]
    ConfigParseError(#[source] toml::de::Error),
}
