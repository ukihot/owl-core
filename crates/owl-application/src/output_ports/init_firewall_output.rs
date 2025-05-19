use crate::UsecaseError;

pub trait InitFirewallOutput {
    fn on_success(&mut self);
    fn on_failure(&mut self, err: &UsecaseError);
}
