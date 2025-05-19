use crate::UsecaseError;
use crate::output_ports::init_firewall_output::InitFirewallOutput;
use owl_infra::OwlConfig;
use tokio::process::Command;

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

    pub async fn execute(&mut self, conf: &OwlConfig) -> Result<(), UsecaseError> {
        #[cfg(target_os = "linux")]
        {
            self.setup_linux(conf).await?;
        }

        #[cfg(target_os = "windows")]
        {
            self.setup_windows(conf).await?;
        }

        // macOS は PF による明示ルール不要（WireGuard kernel ext / utun）
        #[cfg(target_os = "macos")]
        {
            // 何もしない
        }

        self.presenter.on_success();
        Ok(())
    }

    // ───────────────────────────────────────────────
    #[cfg(target_os = "linux")]
    async fn setup_linux(&self, conf: &OwlConfig) -> Result<(), UsecaseError> {
        // nftables スクリプトを組み立て
        let script = format!(
            "table inet owl {{
                chain input {{
                    type filter hook input priority 0;
                    ct state established,related accept
                    iifname \"wg0\" accept
                    tcp dport {{ 22 }} accept   # SSH
                    tcp dport {{ {} }} accept   # WireGuard handshake
                    counter drop
                }}
            }}",
            conf.interface.listen_port
        );

        let mut child = Command::new("nft")
            .arg("-f")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| UsecaseError::FirewallSetupFailed(e.into()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(script.as_bytes()).await.ok();
        }

        let status = child.wait().await.map_err(|e| {
            UsecaseError::FirewallSetupFailed(format!("nft exec error: {e}").into())
        })?;

        if !status.success() {
            return Err(UsecaseError::FirewallSetupFailed(anyhow::anyhow!(
                "nft returned non‑zero"
            )));
        }
        Ok(())
    }

    // ───────────────────────────────────────────────
    #[cfg(target_os = "windows")]
    async fn setup_windows(&self, conf: &OwlConfig) -> Result<(), UsecaseError> {
        let rule = "netsh advfirewall firewall add rule name=\"Owl‑WireGuard\" dir=in ".to_string();
        let rule = format!(
            "{rule} action=allow protocol=UDP localport={}",
            conf.interface.listen_port
        );

        let status = Command::new("cmd")
            .args(["/C", &rule])
            .status()
            .await
            .map_err(|e| UsecaseError::FirewallSetupFailed(e.into()))?;

        if !status.success() {
            return Err(UsecaseError::FirewallSetupFailed(anyhow::anyhow!(
                "netsh returned non‑zero"
            )));
        }
        Ok(())
    }
}
