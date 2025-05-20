use crate::{UsecaseError, output_ports::init_vpn_stack_output::InitVpnStackOutput};
use base64;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as Base64Std;
use boringtun::device::Device;
use boringtun::device::DeviceConfig;
use owl_infra::OwlConfig;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use tun::{Configuration, Device as TunDevice};

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

    pub async fn execute(&mut self, conf: &OwlConfig) -> Result<(TunDevice, Device), UsecaseError> {
        let tun = create_tun(conf)?; // TUN デバイスの生成
        let device = init_wireguard(conf).await?; // WireGuard デバイス初期化

        self.presenter.on_success(); // 成功通知
        Ok((tun, device))
    }
}

fn create_tun(conf: &OwlConfig) -> Result<TunDevice, UsecaseError> {
    let addr4 = conf
        .interface
        .address4
        .as_ref()
        .ok_or(UsecaseError::Address4NotSet)?;
    let mut cfg = Configuration::default();
    cfg.address(addr4.addr()).netmask(addr4.netmask()).up();

    tun::create(&cfg).map_err(|e| {
        UsecaseError::ConfigReadError(std::io::Error::other(format!("TUN作成失敗: {e}")))
    })
}

async fn init_wireguard(conf: &OwlConfig) -> Result<Device, UsecaseError> {
    // 1. インターフェイス名とキー準備
    let name = conf
        .interface
        .name
        .as_ref()
        .ok_or(UsecaseError::InterfaceNameNotSet)?;
    let private_key_bytes = Base64Std
        .decode(&conf.interface.private_key)
        .map_err(UsecaseError::KeyDecodeError)?;
    let private_key_str = Base64Std.encode(&private_key_bytes);

    // 2. Device の生成
    let device = Device::new(name, DeviceConfig::default()).map_err(|e| {
        UsecaseError::WireguardInitError(std::io::Error::other(format!("WG初期化失敗: {e}")))
    })?;

    // 3. UAPI ソケットへ設定を書き込む
    let socket_path = format!("/var/run/wireguard/{}", name);
    let mut uapi = UnixStream::connect(&socket_path).await.map_err(|e| {
        UsecaseError::UapiConnectError(std::io::Error::other(format!("UAPI接続失敗: {e}")))
    })?;
    let mut config_uapi = String::new();
    config_uapi += &format!("private_key={}\n", private_key_str);
    //    ここでピア設定なども追記する…
    uapi.write_all(config_uapi.as_bytes()).await.map_err(|e| {
        UsecaseError::UapiWriteError(std::io::Error::other(format!("UAPI書き込み失敗: {e}")))
    })?;

    Ok(device)
}
