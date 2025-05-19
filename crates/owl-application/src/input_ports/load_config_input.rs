use crate::UsecaseError;
use anyhow::Result;
use owl_infra::OwlConfig;

pub trait LoadConfigInput {
    async fn execute(&mut self) -> Result<OwlConfig, UsecaseError>;
}
