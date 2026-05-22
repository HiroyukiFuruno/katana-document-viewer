## MODIFIED Requirements

### Requirement: `--theme` JSON入口は別changeで扱わなければならない

システムは、完全テーマJSONを受け取るCLI入口を `add-complete-theme-contract` では実装せず、このchangeで扱わなければならない（MUST）。

#### Scenario: 将来のJSON入口名を確認する

- **WHEN** CLIの完全テーマJSON入口を設計する
- **THEN** option名は `--theme` とする
- **THEN** `--thema` は採用しない
