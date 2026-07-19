# Implementation Status

- [x] KDV は HTML parser、CSS/JS evaluator、WebView を持たず、KRR session adapter だけを実装した。
- [x] raw HTML と完全な document URL origin を KRR `HtmlBrowserSource` のまま渡す。
- [x] input、resize、refresh、explicit navigation、browser navigation、close を session thread 経由で伝播する。
- [x] frame coalescing と navigation/error FIFO を契約テストで確認した。
- [x] browser adapter source は regions、functions、lines ともに 100% coverage。
- [x] `rtk cargo test -p katana-document-viewer --test browser_session_adapter_contract -- --test-threads=1` は 2 tests 成功。
- [x] `rtk just coverage-missing` は browser adapter source の未カバー 0 で成功。
- [x] `rtk just ast-lint` と strict clippy を通過した。
- [x] release-line manifest と mechanical target check が `v0.3.x` を browser session adapter に固定し、PDF pagination と multi-format の計画を `v0.4.0`、`v0.5.0` に繰り延べる。
- [x] release contract は adapter-only の KDV v0.3.0 と legacy Storybook UI acceptance
  を分離し、adapter line では registry KRR lock、ownership prohibition、integration
  contract、strict quality gate を機械検証する。証跡: `rtk just VERSION=0.3.0 release-verify`。
