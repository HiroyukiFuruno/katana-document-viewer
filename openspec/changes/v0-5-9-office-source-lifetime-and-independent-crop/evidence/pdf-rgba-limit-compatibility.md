# PDF容量上限とRGBA容量上限の互換性

## 最新の検証結果

標準flags/job1で修正後の実consumer IT（session37356）exit0、1pass/0fail。1MiB PDF上限、直接APIとのpixel/metadata一致、4回のchild launchとcache再利用を確認した。実worker異常系（session61000）もexit0、5pass/0failで、128bytes requestの受理後output-limit到達、129bytes拒否、OS CPU hard-limit拒否、不正引数usage64、実file/response write失敗を維持した。

通常全件 `just JOBS=1 check` のsession56190はAST lintで応答処理35行の制限違反を検出しexit101。identity/status検証をfocused helperへ分割し、session82463の再実行はexit0。本体1956pass/0fail/1ignored、実IT、Storybook通常経路640+1pass/0fail、provenance27件とfmt/strict Clippy/AST/entrypoint/document boundary/harness/V8 singletonが成功した。raw `issue59-rss.oBZVZI/just-check-v059-rgba-limit-final-rerun.log`。coverage/semver/RSS/fidelity/完全release-checkの修正後再検証は未完了。以下の「再build中」はRED/修正準備当時の履歴であり、現時点のGREENは本節が正。

readonly契約レビューで、既存 `OfficeWorkerConfig.max_output_bytes` は変換PDFファイルの上限であり、初期raster隔離候補がそれをRGBAへも流用していることを確認した。

公開型は変更せず、既存の実Office frame/cache ITへ1MiBのカスタムPDF上限を追加した。修正前候補のpackage検証済みrlibと凍結workerで実consumerをlink/runした結果、PDF55,832bytesの変換は成功し、そのRGBA2,001,580bytesは `Office(EngineFailure { stage: "output_limit", ... })` で拒否された。test exit101、0pass/1fail。上限以内のPDFが描画で拒否される実REDであり、静的推測だけではない。

- RED rlib SHA `e54746ff39d34242e4fb6ff9c978a053e52281a8d7e395afc702116f557d0e62`
- RED worker SHA `05edb527f0b60533aaf1eeb7cbf0217a56df573b0f0d535e432f440181d2e6e6`
- 回帰source `tests/office_raster_worker_contract.rs`

修正候補ではprivateなrequest JSONに `max_rgba_bytes` を追加し、親は従来のPDF render pixel上限×4を渡す。子は同値とstrict PDF pixel上限×4の小さい方でRGBAを制限し、親もdimension/pixel/bytes/実file長を再照合する。converted PDFのinput byte上限は元の設定値のまま。既存public API、128bytes request上限、64KiB response上限、安全上限、sandbox/deadline、typed errors、cacheを変えない。

初回Cargo RED実行はprofile/flags相違による大きな再buildを開始したため、自己所有processだけを中断し、凍結済み実artifactでREDを先に確定した。現在は標準の `RUSTFLAGS=-D warnings`、`CARGO_BUILD_JOBS=1` で新候補を再build中。新候補GREEN、完全gate/coverage/semver/RSS/Office fidelityの再検証は未完了。以前の完全release-check PASSを本修正後の結果に流用しない。

追加readonlyレビュー: agent `01a0fc10-58cf-7c31-9f69-454e29867e16` / `gpt-6-luna` / `medium`、private protocol/親子RGBA制限/回帰sourceにP0/P1なし。128bytesちょうどの実worker通過が不足との指摘を受け、既存RGBA上限回帰のJSONを128bytesへpaddingした。`output_limit`まで到達することでrequestが受理されたことを実OS workerで確認する。現候補の実行結果は未確定。
