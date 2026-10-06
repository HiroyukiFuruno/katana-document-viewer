# 自己レビュー（現公開依存でPASS）

## 結論

現公開KRR 0.4.23の最新source/graphでrelease-checkの全工程を回収してPASS。RGBAと変換PDFの生バイト保存・PDF検証後persistを含む修正後、全体coverageは3787関数/31196行とも100%、未カバー0。全体検査はcoverage中に中断されたため、同sourceを維持してcoverageと残りの工程を再実行し、個別の終了0を確認した。KRR新版の公開取り込み・同一最終graphのrelease-check・current HEAD review・公開は未完。

ローカル全体ログ: tmp/pr68-conversion-binary-release-check-retry.log（semver/check PASS）、tmp/pr68-conversion-binary-coverage-resume.log（coverage PASS）、tmp/pr68-conversion-binary-package.log、tmp/pr68-conversion-binary-publish-dry-run.log（各終了0）。最新sourceのmacOS検証であり、Linuxの最新HEAD CI成功は別途確認する。

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

ユーザー指示でglobal Stop hookを実装・正規のCodex /hooksでreview/trustした。物理台帳のpending/running/failedや根拠不足を検出して自動継続する。11回帰PASS、実CLIでpending終了試行→hook自動継続→done終了の結合試験もPASS。通常回答・明示中断・独立作業完了後の上流公開待ちを区別する。台帳内容の意味的な正しさは主担当が担い、hookを完了の代用にしない。

## cffa46d5追加レビュー対応

同じsessionの描画では、request検証後に既存の容量制限付きPDF LRUを先に参照する。diskから検証・復元したpageも同LRUへ登録し、Officeも既存PDF sessionへ委譲する。独自の二重LRUは追加しない。renderのhitはmemoryまたはdiskの再利用を示し、disk clear後も開いたsessionのmemoryを保持する。memory容量0では従来のdisk経路を使用する。

実PDFのwarm/clear、disk復元後のclear、memory容量0、実PDF出力から構築したOffice sessionのdisk復元後clearを検証した。cache単体41件PASS。`just VERSION=0.5.13 JOBS=2 release-check` は終了0、全既存検査・semver196・3791関数/31226行100%・package/dry-run/未公開版確認PASS。新しいPDF/Office helperも全関数・行100%。根拠は非公開ログtmp/pr68-live-cache-release-check.log、RTK tee1791275274_just_coverage.log。

検査開始後に別件の容量整理導線を追加したが、上記のruntime source/dependency graphは変更していない。新しい導線は既定40GiB・閾値以下の非削除・自repo範囲・使用中保護・コマンド終了までの排他・ハードリンクの重複除外を持ち、`just build-cache-script-test` 20件PASS。toolchain/global option・空白を含むcheckoutとsubdirectoryの実動作も確認した。全既存ゲートとcoverageの閾値は変更していない。最終commitの通常pre-push/current HEAD review/CIと最終KRR graphの検証は継続する。

cffa46d5 Ubuntu CIのXLSX fallback testは初回失敗・失敗job再実行成功。初回の実error variantは記録されておらず、原因を修正済みとは主張しない。次の失敗時に実errorを示す診断を追加し、期待するEngineFailure/spreadsheet_openの基準は維持する。新guard経由のXLSX実worker契約11件PASS（tmp/pr68-xlsx-spawn-diagnostic-contract.log）。macOS/Windowsと再実行Ubuntuは成功した。各review threadのreply/resolve・fresh確認は結果取得まで未完。

## 1bdfa63b追加レビュー対応（全体ローカル検証PASS）

- P2 destination種類: saveの既存directory/symlink成功を実filesystem2回帰でRED確認。symlink_metadataでregular fileだけを再利用し、不正種類はUnsafeDirectory、metadata失敗はIoへ修正した。directory/symlinkのGREEN、通常既存fileの内容保持、長すぎるfile名のIo回帰を確認。
- P2実engine依存: 固定hayro/office2pdf文字列を撤去し、実際に静的リンクされたhost executableのSHA256をprocess内で共有してmetadata v2へ束縛する。同一KDV版・environmentでもimageが変われば旧artifactはmiss。外部dynamic module/font/configはhost environment_revisionの対象として明記。型識別子やconsumer lock探索には依存しない。
- cache単体31件、AST、実PDF/DOCX/PPTXの独立process restart/invalidation/容量/破損/clear回帰がPASS。匿名fixture9processのcache初期化（実image hash含む）は0.0371–0.0384秒。restart初期化+初回frameはPDF約0.092秒、DOCX約0.135秒、PPTX約0.133秒。DOCX cold最大約1.447秒。入力取得は各初回frame計測内、初期化費用を別途足した値であり、従来の初期化除外値と混同しない。
- 計測根拠: tmp/pr68-engine-process-timing.log（非公開作業ログ）。全体release-check終了0、3779関数/31111行100%、key/store自身も全関数・行100%、package/dry-run/未公開版確認PASS。再レビューでP0/P1追加なし。current HEAD review・CIと各thread reply/resolveは未完で継続する。

93bf2452で上記2件を公開し、各threadへ検証根拠をreply/resolve、fresh取得で3件すべて解決済みを確認した。1bdfa63bの3OS CIも成功し、Ubuntu coverage不足を解消した。最新93bf2452レビューの追加2件を以下で追跡する。

## 93bf2452追加レビュー対応（全体ローカル検証PASS）

- P2 RGBA保存容量: 生RGBAと長さ付きJSON metadataへ変更。4096×4096白ページ67,108,864bytesを128MiB cacheで保存・読込できる実filesystem回帰と、短いprefix/長さoverflow/metadata欠損・不正/metadata内RGBA混入/既存surface制約の回帰を確認した。JSON旧形式へ依存した検証も新形式で本来の安全性検査を通るよう更新した。
- P2 PDF検証前persist: conversion取得からdisk保存を分離し、既存OfficeStaticViewerSessionのdecode/制約検証に成功してから保存する。実PDFで保存・hit、invalid PDFでsession失敗かつartifact欠損を検証した。変換を代替するmock workerは追加していない。
- cache単体35件とAST PASS。全release-check終了0、3784関数/31177行100%、page codec自身も3関数/42行100%。根拠は非公開作業ログtmp/pr68-binary-validation-release-check.logとRTK tee1791265370_just_coverage.log。同sourceで全テスト・score/semver/strict lint・package/dry-run/未公開版確認PASS。静的再レビューに追加欠陥なし。current HEAD review/CI・各thread reply/resolveは結果回収まで完了扱いしない。

上記2件はe7f6cd1cでreply/resolveし、fresh取得で5件とも解決済み。最新レビューで変換PDF本体のJSON数値配列にも容量膨張が残ることを検出した。

## e7f6cd1c追加レビュー対応（全体ローカル検証PASS）

RGBAと変換PDFで長さ付きmetadataと生バイトを共通payload codecへ統合。変換metadata内のPDF混入を拒否し、現行worker出力上限・cache entry上限・PDF decode後persistを維持する。64MiB payloadのcache往復とprefix/長さ/metadata破損回帰を追加した。payload回帰は保存形式の境界検証であり、64MiBの実文書decodeを主張しない。実worker/PDF/DOCX/PPTXの契約は既存統合回帰で確認する。静的再レビューに新たな欠陥なし。検証結果・全品質/coverage・review reply/resolve・fresh確認は回収まで未完。

追加後cache単体37件とAST PASS。単体ログtmp/pr68-conversion-binary-unit.log（非公開）。公開APIのrestored_conversion_cannot_exceed_worker_output_limitは生PDFの長さを書き換え、checksum/key bindingを再計算する形へ更新し、実workerを使うerror contract5件PASS。semver196項目・全check・全coverage（3787関数/31196行100%）・version/package/dry-run/未公開版確認が同sourceでPASS。中断前後の工程を混同せず、上記各ログの結果を回収した。追加sourceのcommit/push・thread reply/resolve・fresh current HEAD review/CIは継続する。

容量guard導入後の通常pushで、Storybook静的検査のliteral Cargo認識が失敗した。既知の` scripts/maintenance/cargo-guard `実行だけを既存Cargo表記へ正規化し、必須引数・selectorを維持した。`just build-cache-script-test` 21 PASS（実default guardとCARGO=cargoの受理、未知wrapper拒否）、`just ast-lint` PASS。runtime sourceは全release-check成功時から変更なし。最新push・review・CIは継続回収する。

## 128MiB payload境界と容量guardの追加確認

公開worker既定の128MiB PDFを保持できるよう、生payload128MiB・metadata1MiB・封筒80byteを分離した。総quotaは実ファイル容量で判定し、上限超過・非framed oversized・checksum破損を拒否する。実filesystemを含むcache単体51件を全coverageで検証した。最初のrelease-checkは物理上限拒否の未通過1行で失敗したため、上限+1byteの回帰を追加し、標準coverageを再実行した。`tmp/pr68-payload-budget-coverage-resume.log` は終了0、3796関数・31274行とも100%。元のrelease-checkを終了0とは扱わない。残りの標準release工程（llvm target clean/version/package/publish dry-run/未公開確認）も終了0で回収した。証跡: `tmp/pr68-payload-budget-release-resume.log`。最終ASTも `tmp/pr68-payload-budget-final-ast.log` で成功。read-only最終レビューで具体的な不具合なし。

容量guardは標準40GiB（約2build分）、閾値以下では整理しない。別checkoutの任意検査は通常Cargoへ分離し、明示CARGO上書きを保持する。Windows文字コードをUTF-8へ固定し、Git Bashを明示した。23件のlocal回帰は成功、最終HEADのWindows実CIは未確認。
