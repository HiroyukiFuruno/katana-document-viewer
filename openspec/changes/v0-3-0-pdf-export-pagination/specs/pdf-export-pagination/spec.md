## ADDED Requirements

### Requirement: PDF export はページ境界制御と事前 preview を提供しなければならない

システムは、Markdown を PDF export する際、見出し・コードブロック・ダイアグラム・テーブルがページ途中で分断されないようにする改ページルールを実装し、export 前にページ分割 preview を提供しなければならない（MUST）。

#### Scenario: 改ページルールを適用する

- **WHEN** Markdown を PDF として export する
- **THEN** h1 / h2 見出しの前に改ページが挿入される（設定可能）
- **THEN** コードブロック・ダイアグラム・テーブルが分断される場合は前ページに留めるか次ページへ送る
- **THEN** 強制改ページ記法（`---` 等）が尊重される

#### Scenario: export 前にページ分割を preview する

- **WHEN** ユーザーが PDF export を実行する
- **THEN** 保存ダイアログを開く前にページ分割 preview が表示される
- **THEN** preview と実際の PDF 出力が同じページ計算結果を使う

#### Scenario: ExportConfig 経由で改ページルール / 用紙設定を注入する

- **WHEN** ホストが `ExportConfig`（用紙サイズ・余白・改ページルール）を kcf の `Exporter` に渡す
- **THEN** kcf は `ExportConfig` に従って PDF を生成する
