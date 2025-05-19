use crate::UsecaseError;
use crate::output_ports::init_firewall_output::InitFirewallOutput;
use owl_infra::OwlConfig;

pub struct InitFirewallInteractor<'a, P>
where
    P: InitFirewallOutput + Send + 'a + ?Sized,
{
    presenter: &'a mut P,
}

impl<'a, P> InitFirewallInteractor<'a, P>
where
    P: InitFirewallOutput + Send + 'a + ?Sized,
{
    pub fn new(presenter: &'a mut P) -> Self {
        Self { presenter }
    }

    pub async fn execute(&mut self, _conf: &OwlConfig) -> Result<(), UsecaseError> {
        // nftables / iptables 呼び出し予定
        self.presenter.on_success();
        Ok(())
    }
}
