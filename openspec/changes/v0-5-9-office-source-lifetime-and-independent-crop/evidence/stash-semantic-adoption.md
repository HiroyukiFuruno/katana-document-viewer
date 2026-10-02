# 既存stashの意味的採否

新しいstash/autostashは作成しない。これは既存5件の実diff・third parent・現行sourceを照合した採否台帳であり、削除済みを示すものではない。

## HTML/CSS候補 `2f041b85eab6bac8717b2df3a0efea32640eb70f`

29追跡差分と1未追跡sourceを確認。inline/style blockのCSS正規化、visibility、span、画像cache等は現行`preview_runtime/direct_html_*`、`export_surface`、`viewer/node_plan`とそれぞれの回帰に置換されている。旧V8依存削除/禁止は現行と同じではない。現行はKRRと同V8 singletonを解決し、公開APIへV8を漏らさない契約であり、旧禁止規則は不採用。

third parentの`direct_html_dependencies.rs`は、現行にはないstylesheet/image依存列挙を含む。単に「統合済み」とは扱わない。この候補は`parent.join`だけで`../`やsymlinkによるsource root外読込を防がず、1MiB検査後のreadもatomic/boundedでない。read失敗の無診断消失、double quote限定、画像は内容読込でなく列挙だけという制約もある。現在の`DirectHtmlPreviewRenderer::render_html(content)`はKRRへHTMLを渡す中立APIでsource-path権限を持たず、公開`PreviewOutput`へのrequired dependency field追加はpatchのstruct構築互換性を壊す。従ってこの未公開候補は安全性・公開API互換性の観点でそのまま採用しない。外部stylesheet対応を達成したとは報告しない。

## 旧egui0.36候補 `3f674f47e90d1d6c7ca4e10d07d2dc49fdfa823b`

10追跡差分と1未追跡self-review証跡を確認。manifest/lock、dependency_tests、release契約、linter fixture、host texture delta、旧OpenSpec4ファイルの変更である。現行はegui host層を取り除き、exact registry KUCとvendor-neutral境界を強制している。optional egui依存/featureを再導入する旧更新は不採用。当時のself-reviewは現在のHEAD検証証拠とはしない。

## Office候補 `ba490d97cf5c9cb546de17dd8ddd9fcc6c2946d6`

29追跡差分と17未追跡ファイルを確認。typed outline/page/sheet、resource、streaming、CRC/local header/bit3/ZIP64の責務は現行sourceと契約回帰に存在する。現行`SpreadsheetViewerLimits::strict()`も`max_logical_cells=25_000_000`を維持しており、欠落という補助監査結果を訂正した。

第三親の`.cargo/config.toml`は旧`office2pdf-katana`のlocal path overrideで、最終registry-only契約に反するため不採用。旧0.5.5 release規則/versionは現行の正規公開履歴と0.5.9契約で置換されており、旧全体の再適用はしない。

## 旧依存候補 `1a0b185010f1883d8a04da0c3dbf51eaa5c1748d`

manifest/lockの2差分、third parentなし。旧workspace0.3.3/KRR0.4.9/egui更新は現行0.5.9/KRR0.4.22/KUC0.4.1/V8152.2.0/office2pdf0.8.0に置換されている。syn2→3は現行3.0.6へ反映済みであり、単なる「majorが不整合」という根拠で拒否しない。旧lockの丸ごと復元は現行依存の後退となるため不採用。

## Browser候補 `d29771e89009bc1dfbc55f4bc1248969f858cbb7`

11追跡差分、third parentなし。現在のcommand coalescing/idle/worker shutdown/typed stopped-errorと契約回帰に置換されている。正式履歴`baeb604`（PR#30）はqueue/worker/startup/adapter回帰を含み、archive `2026-09-28-v0-3-0-browser-session-adapter/tasks.md`にもv0.3.4/v0.3.5の公開証跡がある。GitHub Release両版をmainがライブ確認済み。監査の旧root `src/`表記は現行`crates/katana-document-viewer/src/`へ正規化して実sourceを確認した。

## 残す検証境界

最新graphの通常`just check`はPASSし、browser/HTML/Office/依存境界を含む現行回帰を実行した。最新追加のworker異常系を含むstrict coverageもfunctions3739/3739・lines30683/30683で100%。完全release-check・正規release/fresh registry consumerは未完了。stashはこの台帳の作成だけで整理完了とせず、最終採否と公開後の不要物整理を分けて扱う。
