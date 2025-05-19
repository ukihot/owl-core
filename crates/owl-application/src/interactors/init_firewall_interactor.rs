use crate::UsecaseError;
use crate::output_ports::init_firewall_output::InitFirewallOutput;
use crate::usecase_errors::InitFirewallError;

use owl_infra::OwlConfig;
use std::fs::read_to_string;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
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

    pub async fn execute(&mut self, config: &OwlConfig) -> Result<(), UsecaseError> {
        if Self::is_wsl2().unwrap_or(false) {
            println!("WSL2 environment detected. Skipping firewall setup.");
            self.presenter.on_success();
            return Ok(());
        }
        self.setup_nftables_firewall(config)
            .await
            .map_err(UsecaseError::from)
            .map(|_| self.presenter.on_success())
    }

    /// WSL2判定を Result<bool, std::io::Error> で返す純粋関数化
    fn is_wsl2() -> Result<bool, std::io::Error> {
        read_to_string("/proc/version").map(|v| {
            let v = v.to_lowercase();
            v.contains("microsoft") && !v.contains("wsl1")
        })
    }

    /// Linux用nftablesファイアウォールルールをセットアップする
    async fn setup_nftables_firewall(&self, config: &OwlConfig) -> Result<(), InitFirewallError> {
        let nft_script = format!(
            r#"delete table inet owl
        table inet owl {{
            chain input {{
                type filter hook input priority 0; policy drop;
                ct state established,related accept;
                iifname "wg0" accept;
                tcp dport 22 accept;          # SSH
                tcp dport {port} accept;      # WireGuard handshake
                counter;
                drop;
            }}
        }}
        "#,
            port = config.interface.listen_port
        );

        // ── Dump script for debug
        println!("[nft dump] script to be passed:\n{}", nft_script);

        // ── Spawn
        let mut child = Command::new("sudo")
            .arg("nft")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // ── Write
        if let Some(mut stdin) = child.stdin.take() {
            if let Err(e) = stdin.write_all(nft_script.as_bytes()).await {
                return Err(InitFirewallError::Write {
                    source: e,
                    script: nft_script.clone(),
                });
            }
            // 明示的にクローズ
            drop(stdin);
        }

        // ── Execution
        let output = child
            .wait_with_output()
            .await
            .map_err(|e| InitFirewallError::Spawn { source: e })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            println!("[nft dump] stdout: {}", stdout);
            println!("[nft dump] stderr: {}", stderr);
            return Err(InitFirewallError::Execution {
                code: output.status.code(),
                stderr,
            });
        }
        // 成功時も標準出力をダンプ
        println!("[nft dump] stdout: {}", stdout);
        println!("[nft dump] stderr: {}", stderr);

        Ok(())
    }
}
