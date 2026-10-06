# 自己レビュー（現公開依存でPASS）

## 結論

現公開KRR 0.4.23のsource/graphでrelease-checkが終了コード0でPASS。最新修正後の全体coverageは3776関数/31090行とも100%、未カバー0。package検証・publish dry-run・未公開版確認もPASS。Draftレビュー準備は完了。KRR新版の公開取り込み・同一最終graphのrelease-check・current HEAD review・公開は未完。

ローカル全体ログ: RTK tee 1791244971_just_VER_8102d9.log、coverage 1791244880_just_coverage.log。

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
