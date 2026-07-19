## Why

KatanA の interactive HTML viewer は、HTML を静的に解釈または export surface に変換して表示してはならない。HTML、CSS、JavaScript、input、navigation は KRR の in-process Rust/V8 session が一貫して評価し、KDV は KatanA と KRR の間で session lifecycle と更新を受け渡すだけにする。

この変更は KDV `v0.3.0` の release contract である。既存の同一 version を冠した PDF pagination 計画は未実装の将来計画であり、公開前に後続 version へ繰り延べて release target と衝突させない。

## What Changes

- KRR `^0.4.0` の `HtmlBrowserSession` を worker thread で保持する `BrowserSessionAdapter` を追加する。
- KatanA が raw HTML と document URL origin、viewport を KDV に渡せる request 契約を追加する。
- KDV は KRR の frame、navigation、typed error を順序を保って host に返す。
- frame は最新値だけを保持し、navigation と error は失わない。
- KDV は external browser binary を探索、download、検証、起動しない。

## Non-Goals

- HTML parser、CSS cascade、JavaScript interpreter、browser hit-test、WebView を KDV に実装しない。
- KRR の static HTML export/render path を interactive viewer に転用しない。
- Chromium/WebView/helper archive の取得、hash 検証、展開、release asset 配置を KDV で行わない。
- KDV が KatanA の navigation policy や UI を所有しない。

## Capabilities

### New Capabilities

- `browser-session-adapter`: KRR browser session の worker-backed adapter

## Impact

- `crates/katana-document-viewer/` に KRR `HtmlBrowserSession` の adapter を追加する。
- KRR `0.4.0` の crates.io 公開後に lockfile を registry source へ固定する。
- KatanA は公開済み KDV `0.3.0` を使い、native UI の input と frame を接続する。
