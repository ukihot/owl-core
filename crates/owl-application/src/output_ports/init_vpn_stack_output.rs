use crate::UsecaseError;

pub trait InitVpnStackOutput {
    fn on_success(&mut self);
    fn on_failure(&mut self, err: &UsecaseError);
}
