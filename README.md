# owl-core

## 概要
`owl-core` は、Linux 環境向けに設計された VPN エンジンのコアコンポーネントです。
WireGuard ベースのトンネル作成および管理機能を提供し、安全で拡張性の高い社内ネットワークの構築を支援します。

## 対応プラットフォーム
- Linux (主要ディストリビューション)

**Windows は現時点で非対応です。**
`wireguard-control` クレートが Linux の netlink ソケットや ioctl を利用しており、Windows ネイティブの WireGuard ドライバ（wireguard-nt）を制御する実装は含まれていません。
Windows 環境での利用は想定しておらず、Linux サーバー上での運用を前提としています。

## 主要機能
- WireGuard インタフェースの初期化・設定
- TUN/TAP デバイスの生成・管理
- VPN ピアの接続管理およびアクセス制御
- TOML フォーマットによる設定ファイル読み込み
- RBAC (Role-Based Access Control) による権限管理
- Prometheus/Grafana 連携による監視機能の基盤

## 開発の方向性
- `wireguard-control` クレートを活用しつつ、低レイヤの WireGuard 設定管理を実装
- 監査ログの一元管理およびセキュアなアクセス管理の実現
- Docker 環境でのコンテナ化対応による運用の易化
- 高度なネットワークポリシー適用および動的設定反映の実装

## Owl-core 開発環境への参加方法（Dev Container 利用）

このプロジェクトでは、開発環境の差異を排除し、全員が同じ環境で作業できるように Visual Studio Code の Dev Container 機能を利用しています。

以下の手順で開発環境に入ってください。

---

### 前提条件

- Visual Studio Code がインストール済みであること
- VS Code の拡張機能「Remote - Containers」がインストール済みであること
- Docker がインストール・起動していること

---

### 開発環境に入る手順

1. 本リポジトリをローカルにクローン
   ```bash
   git clone <リポジトリのURL>
   cd <リポジトリディレクトリ>

