use crate::UsecaseError;
use anyhow::Result;
use owl_infra::OwlConfig;

pub trait LoadConfigInput {
    fn execute(
        &mut self,
        conf: &OwlConfig,
    ) -> impl std::future::Future<Output = Result<(), UsecaseError>> + Send;
}
