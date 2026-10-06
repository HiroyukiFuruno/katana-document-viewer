# Office/PDFの再起動後cache

`PersistentDocumentCache::new(directory, max_bytes, environment_revision)` にhost所有の専用directoryを渡し、`PersistentOfficeViewerSession::open` または `PersistentPdfViewerSession::open` を使用する。既存のsession APIはcacheを書き込まない。

Office（DOCX/PPTX）は変換PDFと診断情報、PDF/Officeの描画はpageとscaleごとのRGBA artifactを保存する。XLSXは既存spreadsheet sessionを使う。`conversion_cache_hit()` と renderの戻り値 `(page, hit)` で再利用を確認できる。PDFのgeometry/outline取得のdecodeは毎回実行し、`artifact()` / `outline()` から取得できる。変換とrasterを再利用する。cache自体はsourceやlive sessionを保持せず、sessionをdropするとmemoryを解放する。

描画は既存sessionの容量制限付きLRUを先に参照し、memoryにない場合だけdiskを読む。diskから復元したpageも同じLRUへ登録する。renderの `hit` はmemoryまたはdiskの再利用を表し、`clear()` 後も開いたsessionのmemory cacheは保持する。memory容量を0にした場合はdisk経路を使用する。

内容・URI/revision/MIME/format・KDV/schema/engine版・worker実体/設定が変わればmissになる。page/scale/renderer limitsも描画keyに含む。hostは外部font、viewportや描画環境に影響する設定が変わるたびに、空でない `environment_revision` を更新する。

directoryは他用途と共有しない。Unixでは0700で作成/保護し、Windowsのprivate ACLはhostが設定する。ファイルはsource名を含まないSHA256 keyで保存するが、artifactには文書内容が含まれるため、機密入力の保存可否・保存期間・削除UI・disk encryption・バックアップ除外はhostが所有する。hostが永続化を許可しない入力には既存session APIを使う。

KDVは本体128MiB・metadata 1MiBの上限を強制し、prefix 8byteとheader 72byteを別に確保する。総容量上限には付加情報も含む物理保存サイズを数えるため、128MiBの本体にはその付加情報分の容量も必要となる。他プロセスと排他し、同じdirectory内のtemporary fileから原子書込する。欠損はmiss、破損/checksum不一致・容量超過・I/O失敗は `PersistentCacheError` として返す。破損時のclear/retry方針はhostが明示的に決める。暗黙の再変換は行わない。`clear()` は同時利用と排他してartifactを削除し、既に開いたsessionは有効なままとする。

書込中のprocess強制終了で残ったtemporary fileも容量に含む。容量不足時にはhostが `clear()` を実行して残存fileも除去できる。内容を自動削除するevictionや、hostの許可なしの再変換は行わない。

独立プロセス回帰は `persistent_cache_contract`。実workerを使ったPDF/DOCX/PPTXのcold、同一process close/reopen、独立process restart、およびrevision変更時missを検証し、restartのconversion/raster再実行がないことをtraceで確認する。元のsource/preflight/ページ/描画制約はhit時にも適用する。
