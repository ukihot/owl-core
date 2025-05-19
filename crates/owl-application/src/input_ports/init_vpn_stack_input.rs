use crate::UsecaseError;
use anyhow::Result;
use owl_infra::OwlConfig;

pub trait InitVpnStackInput {
    fn execute(
        &mut self,
        conf: &OwlConfig,
    ) -> impl std::future::Future<Output = Result<(), UsecaseError>> + Send;
}
