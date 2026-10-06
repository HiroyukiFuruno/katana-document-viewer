# 自己レビュー（現公開依存でPASS）

## 結論

現公開KRR 0.4.23のsource/graphでrelease-checkが終了コード0でPASS。保存先の種類検証と実host imageによる無効化の最新修正後、全体coverageは3779関数/31111行とも100%、未カバー0。package検証・publish dry-run・未公開版確認もPASS。KRR新版の公開取り込み・同一最終graphのrelease-check・current HEAD review・公開は未完。

ローカル全体ログ: RTK tee 1791262069_just_VER_8102d9.log、coverage 1791261990_just_coverage.log。最新sourceのmacOS検証であり、Linuxの最新HEAD CI成功は別途確認する。

## 指摘と判断

- P1: cached pageの検証が固定strict上限を使い、呼出側設定と一致しなかった。呼出側limitsへ修正し、dimension/pixelの小さいcustom上限に対する拒否回帰を追加。単体回帰と全体coverageでPASS。
- P2: process強制終了時の孤児temporary fileも容量を消費する。host所有clear方針に従って明示Capacityを返し、clearで全残存fileを除去する。自動evictionは導入せず、この復旧契約を公開docsへ追記した。
- P2提案: 既存の壊れた保存先をsaveが無検査で再利用する可能性。公開render/openは先にloadを行い、壊れたentry/symlink/directoryに対して明示エラーで停止するため、提案の「render成功・保存成功扱い」は通常の公開経路で再現しない。private専用directoryの外部改変競合は今回の責任境界外で、loadのchecksum/key結合と排他を維持する。
- ZIP正規化仮説: 合法Stored/Deflated descriptorの実worker回帰が成功したため棄却。既存preflight/安全制約は変更していない。
- P2: header未満の切れたentryがCapacity扱いだった。Corruptへ分類を修正し、実workerが保存したentryを切った公開API回帰を追加した。上限を超えるentryは引き続きCapacityを返す。
- P2: 永続PDF sessionからdecode済み目次を取得できなかった。既存sessionのoutline accessorを公開し、匿名の実PDF bookmarkが失われないことを直接sessionと比較して確認した。
- P2: 空environment revisionが容量不足と区別できなかった。InvalidEnvironmentRevisionを追加し、公開APIの実constructor回帰で区別を確認した。
- P1: custom上限と破損寸法の組合せでRGBA byte長がoverflowし、debugではpanic・releaseでは空RGBAを受理し得た。saturating multiplicationへ修正し、2^31×2^31/空RGBAの実cache entryがCorruptとして拒否される回帰でPASS。

公開APIエラー契約の5回帰（実worker容量/切れたheader、unsupported XLSX、空revision、非空PDF目次）はPASS。修正後ASTもPASS。これ以降はsourceを固定し、release-checkを実行する。KRR新版の取り込み時は変更graphのゲートを再実行する。

## 確認範囲

library APIとhost保存方針の責任分離、PDF/Office sessionのdrop/実resource counter、内容/engine/worker/settingsの無効化、bounded storage/checksum/同directory atomic persist/排他、既存API互換性、公開入力のみを使うfixtureと私有測定の非公開を確認。実画面・clipboard・下流採用はKDV-local完了条件へ追加しない。

## Ubuntu CIアクセサ回帰

448d6640のmacOS CIはPASS。Ubuntu coverageはOffice conversion_cache_hit/artifactとPDF outlineの3関数・9行不足で失敗した。既存integrationは成功しているが、library単体側も直接実PDFfixtureで値を検証する回帰を追加した。本体実装は変更していない。追加後cache単体24回帰とASTはPASS。Linux100%復旧は次HEAD CIで未確認であり、成功扱いしない。最終KRR graphのfull release-checkも引き続き必要。

## PR68 P2総容量縮小後のload

21b383bf reviewの指摘を採用。既存の総容量契約でありDoD変更はない。実filesystem回帰 load_rejects_a_cache_whose_total_size_exceeds_the_current_capacity は修正前RED（超過loadを受理）・修正後GREEN。loadの排他lock内でused_bytesとmax_bytesを比較し、読込前にCapacityを返す。exact上限のload、clear後miss/再保存/再利用、総容量が大きくても1件128MiB上限の回帰もPASS。9d796fe3で修正し各threadへ根拠を返信・resolve、fresh取得で解決済みを確認した。同HEADのcloud reviewに追加指摘なし。

9d796fe3 Ubuntu coverageの不足1行は、used_bytesが先にunsafe entryを拒否し、対象entryの安全性拒否に到達しなくなったことが原因。対象metadataの安全性を先に検査し、総量検査、metadata結果の処理へ進む順序に修正した。load_reports_capacity_for_a_missing_key_when_the_cache_is_over_capacity を追加して、欠損キーでも総量超過を拒否する契約を維持。cache単体27件、AST、全release-checkとmacOS100% coverageがPASS。静的再レビューでP0/P1追加なし。最新HEADのLinux CIとcloud reviewは取得・確認を続ける。clearは容量縮小後も可能なまま。総量検査はentry数に比例するdirectory scanを伴い、最終KRR graphでも全品質ゲートを維持する。

## 未完停止の再発防止

ユーザー指示でglobal Stop hookを実装・正規のCodex /hooksでreview/trustした。物理台帳のpending/running/failedや根拠不足を検出して自動継続する。10回帰PASS、実CLIでpending終了試行→hook自動継続→done終了の結合試験もPASS。通常回答・明示中断・独立作業完了後の上流公開待ちを区別する。台帳内容の意味的な正しさは主担当が担い、hookを完了の代用にしない。

## 1bdfa63b追加レビュー対応（全体ローカル検証PASS）

- P2 destination種類: saveの既存directory/symlink成功を実filesystem2回帰でRED確認。symlink_metadataでregular fileだけを再利用し、不正種類はUnsafeDirectory、metadata失敗はIoへ修正した。directory/symlinkのGREEN、通常既存fileの内容保持、長すぎるfile名のIo回帰を確認。
- P2実engine依存: 固定hayro/office2pdf文字列を撤去し、実際に静的リンクされたhost executableのSHA256をprocess内で共有してmetadata v2へ束縛する。同一KDV版・environmentでもimageが変われば旧artifactはmiss。外部dynamic module/font/configはhost environment_revisionの対象として明記。型識別子やconsumer lock探索には依存しない。
- cache単体31件、AST、実PDF/DOCX/PPTXの独立process restart/invalidation/容量/破損/clear回帰がPASS。匿名fixture9processのcache初期化（実image hash含む）は0.0371–0.0384秒。restart初期化+初回frameはPDF約0.092秒、DOCX約0.135秒、PPTX約0.133秒。DOCX cold最大約1.447秒。入力取得は各初回frame計測内、初期化費用を別途足した値であり、従来の初期化除外値と混同しない。
- 計測根拠: tmp/pr68-engine-process-timing.log（非公開作業ログ）。全体release-check終了0、3779関数/31111行100%、key/store自身も全関数・行100%、package/dry-run/未公開版確認PASS。再レビューでP0/P1追加なし。current HEAD review・CIと各thread reply/resolveは未完で継続する。
