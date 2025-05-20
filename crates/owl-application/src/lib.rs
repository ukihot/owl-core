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

pub struct OwlApplication<'a> {
    pub config_presenter: &'a mut (dyn LoadConfigOutput + Send),
    pub fw_presenter: &'a mut (dyn InitFirewallOutput + Send),
    pub vpn_presenter: &'a mut (dyn InitVpnStackOutput + Send),
    tun: Option<tun::Device>,
    device: Option<boringtun::device::Device>,
}

impl<'a> OwlApplication<'a> {
    pub fn new(
        config_presenter: &'a mut (dyn LoadConfigOutput + Send),
        fw_presenter: &'a mut (dyn InitFirewallOutput + Send),
        vpn_presenter: &'a mut (dyn InitVpnStackOutput + Send),
    ) -> Self {
        Self {
            config_presenter,
            fw_presenter,
            vpn_presenter,
            tun: None,
            device: None,
        }
    }

    pub async fn init(&mut self, config_path: &Path) -> Result<&mut Self, UsecaseError> {
        // (1) 設定読込
        let conf = LoadConfigInteractor::new(config_path.to_path_buf(), self.config_presenter)
            .execute()
            .await?;
        // (2) FW 初期化
        InitFirewallInteractor::new(self.fw_presenter)
            .execute(&conf)
            .await?;
        // (3) VPN 準備
        let (tun, device) = InitVpnStackInteractor::new(self.vpn_presenter)
            .execute(&conf)
            .await?;
        self.tun = Some(tun);
        self.device = Some(device);

        Ok(self)
    }

    /// VPNランタイムをspawnする
    pub async fn spawn_vpn_runtime(&mut self) -> Result<(), UsecaseError> {
        // TUN, WGデバイスを取得(self.tun, self.device)

        // UDPソケットを仮で作成

        // 1. Create epoll

        // 2. Register TUN & UDP (devices implement AsFd)

        // 3. Event loop
        
        Ok(())
    }
}