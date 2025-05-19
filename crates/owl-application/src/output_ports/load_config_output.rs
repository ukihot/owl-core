use crate::UsecaseError;
use owl_infra::OwlConfig;

pub trait LoadConfigOutput {
    fn on_success(&mut self, config: &OwlConfig);
    fn on_failure(&mut self, err: &UsecaseError);
}
