## Context

KRR は in-process Rust/V8 runtime、resource policy、HTML/CSS/JS/input/navigation を所有する。KDV が browser を再実装すると表示結果と security policy が分岐するため、KDV は worker-backed adapter に限定する。Chromium、WebView、external helper process はこの release line に含めない。

## Decisions

### KDV request は raw HTML と完全な document URL origin をそのまま渡す

`BrowserSessionRequest` は KRR の `HtmlBrowserSource` と viewport を持つ。KDV は HTML を parse、sanitize、transform しない。相対 URL と same-origin 判定は KRR が document URL origin を使って処理する。

### worker は command と update を分離する

adapter は command queue を worker に送り、worker は KRR session を単独所有する。worker が返す frame は最新値へ coalesce する。navigation と error は FIFO の更新として残すので、KatanA は link click と JavaScript navigation を取りこぼさない。

### session runtime は KRR crate 内に閉じる

KDV は process config、環境変数、host filesystem を使って browser binary を推測しない。KRR session は crate 内の Rust/V8 runtime を起動し、KDV には typed frame/navigation/error だけを公開する。

### error は typed update として host に返す

launch、protocol、command dispatch の失敗は `BrowserSessionAdapterError` を `BrowserSessionUpdate::Error` として返す。worker を停止させる close、queue full、worker panic も区別する。

## Verification

- normal library target の integration contract が initial frame、text input、resize、refresh、explicit navigation、link navigation、close を確認する。
- resize、navigation、refresh の KRR error が adapter update に現れることを確認する。
- state と worker unit tests が coalescing、FIFO、poisoned lock、worker stop を確認する。
- coverage report で今回追加した browser adapter source の regions、functions、lines をすべて 100% にする。
