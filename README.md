# テスト後のクリーンアップ
## WireGuard 用ルール “Owl‑WireGuard” を削除
netsh advfirewall firewall delete rule name="Owl‑WireGuard"

## （任意）削除できたか確認
netsh advfirewall firewall show rule name="Owl‑WireGuard"
