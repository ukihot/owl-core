use owl_infra::OwlConfig;

use crate::UsecaseError;
use crate::output_ports::init_vpn_stack_output::InitVpnStackOutput;

pub struct InitVpnStackInteractor<'a, P>
where
    P: InitVpnStackOutput + Send + 'a + ?Sized,
{
    presenter: &'a mut P,
}

impl<'a, P> InitVpnStackInteractor<'a, P>
where
    P: InitVpnStackOutput + Send + 'a + ?Sized,
{
    pub fn new(presenter: &'a mut P) -> Self {
        Self { presenter }
    }

    pub async fn execute(&mut self, _conf: &OwlConfig) -> Result<(), UsecaseError> {
        // wireguard‑rs と tun crate を組み込み予定
        self.presenter.on_success();
        Ok(())
    }
}
