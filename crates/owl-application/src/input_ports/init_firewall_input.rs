use crate::UsecaseError;
use anyhow::Result;
use owl_infra::OwlConfig;

pub trait InitFirewallInput {
    async fn execute(&mut self, conf: &OwlConfig) -> Result<(), UsecaseError>;
}
