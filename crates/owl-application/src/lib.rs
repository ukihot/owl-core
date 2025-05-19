use anyhow::Result;
use std::path::Path;

pub mod usecase_errors;
use usecase_errors::UsecaseError;

pub mod input_ports;
pub mod interactors;
pub mod output_ports;

use interactors::init_firewall_interactor::InitFirewallInteractor;
use interactors::init_vpn_stack_interactor::InitVpnStackInteractor;
use interactors::load_config_interactor::LoadConfigInteractor;
use output_ports::init_firewall_output::InitFirewallOutput;
use output_ports::init_vpn_stack_output::InitVpnStackOutput;
use output_ports::load_config_output::LoadConfigOutput;

pub struct AppBuilder<'a> {
    pub config_presenter: &'a mut (dyn LoadConfigOutput + Send),
    pub fw_presenter: &'a mut (dyn InitFirewallOutput + Send),
    pub vpn_presenter: &'a mut (dyn InitVpnStackOutput + Send),
}

impl<'a> AppBuilder<'a> {
    pub async fn init(self, config_path: &Path) -> Result<()> {
        // 0) 設定読込
        let mut config_interactor =
            LoadConfigInteractor::new(config_path.to_path_buf(), self.config_presenter);
        let conf = config_interactor.execute().await?;

        // 1) ファイアウォール初期化
        let mut fw_interactor = InitFirewallInteractor::new(self.fw_presenter);
        fw_interactor.execute(&conf).await?;

        // 2) VPN スタック (WireGuard + TUN) 準備
        let mut vpn_interactor = InitVpnStackInteractor::new(self.vpn_presenter);
        vpn_interactor.execute(&conf).await?;

        Ok(())
    }
}
