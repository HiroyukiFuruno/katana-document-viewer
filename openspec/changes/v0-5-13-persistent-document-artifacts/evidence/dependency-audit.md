# 依存確認 2026-10-06

- 開始時: KDV 0.5.12、KRR 0.4.23、office2pdf 0.8.1、zip直依存8.6.0。workspace全dependencyとCargo.lockを確認。
- `cargo outdated --workspace --depth 1 --exit-code 0`: All dependencies are up to date。現在の互換要件内でdirect dependency更新候補なし。major候補の評価と全推移lock確認は継続する。
- 永続artifact内容hashに、既に推移graphへ存在するsha2 0.11をdirect dependencyとして追加。ゲートは変更しない。
- KRRの既存チャットはHTML Chrome比較99以上とMermaid修正の対応中。現時点の最新公開は0.4.23のため、完了した新版の公開確認を待ってから依存下限・lock・公開契約へ反映する。
- 公開前: registry source/checksum、inverse tree/V8 singleton、全品質/semver/score/coverageを再検証する。
- `cargo outdated --workspace --aggressive --depth 1 --exit-code 0` もAll dependencies are up to date。現在の公開direct major候補はなく、推移graphはworkspace updateで更新候補0件。最終KRR版公開後にlock graphを再評価する。
