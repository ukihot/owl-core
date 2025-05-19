use owl_infra::OwlConfig;
use crate::{UsecaseError, output_ports::init_vpn_stack_output::InitVpnStackOutput};
use tun::{Configuration, Device as TunDevice};
use boringtun::device::{Device, DeviceConfig, DeviceHandle};
use base64::engine;
use base64::Engine;
use base64;

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

    pub async fn execute(&mut self, conf: &OwlConfig) -> Result<(), UsecaseError> {
        let tun = create_tun(conf)?;                 // TUN デバイスの生成
        let handle = init_wireguard(&tun, conf)?;    // WireGuard デバイス初期化

        tokio::task::spawn_blocking(move || {
            handle.poll()
        });

        self.presenter.on_success();                // 成功通知
        Ok(())
    }
}

fn create_tun(conf: &OwlConfig) -> Result<TunDevice, UsecaseError> {
    let addr4 = conf.interface.address4
        .as_ref()
        .ok_or(UsecaseError::Address4NotSet)?;
    let mut cfg = Configuration::default();
    cfg.address(addr4.addr()).netmask(addr4.netmask()).up();

    tun::create(&cfg).map_err(|e| UsecaseError::ConfigReadError(
        std::io::Error::new(std::io::ErrorKind::Other, format!("TUN作成失敗: {e}"))
    ))
}

fn init_wireguard(
    tun: &TunDevice,
    conf: &OwlConfig
) -> Result<DeviceHandle, UsecaseError> {
    let name = conf.interface.name.as_ref().ok_or(UsecaseError::ConfigReadError(
        std::io::Error::new(std::io::ErrorKind::Other, "インターフェース名が設定されていません")
    ))?;
    let wg_cfg = DeviceConfig {
        ..DeviceConfig::default()
    };

    let handle: DeviceHandle = Device::new(name.as_str(), wg_cfg)
        .map_err(|e| UsecaseError::ConfigReadError(
            std::io::Error::new(std::io::ErrorKind::Other, format!("WG初期化失敗: {e}"))
        ))?;

    // Set the private key using the device API (uapi or other method)
    let private_key_bytes = engine::general_purpose::STANDARD
        .decode(&conf.interface.private_key)
        .map_err(UsecaseError::KeyDecodeError)?;
    let private_key_str = base64::engine::general_purpose::STANDARD.encode(&private_key_bytes);

    Ok(handle)
}