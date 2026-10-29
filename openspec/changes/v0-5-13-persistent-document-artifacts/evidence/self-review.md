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

追加reviewの任意directory名とCargo `--target-dir`両形式を修正。repo root・外部・外部symlinkの拒否を維持し、test binaryの `--` 以降はtarget optionとして扱わない。実ephemeral Cargoのbuild→閾値超過clean→再buildを両形式で検証。`tmp/pr68-target-dir-guard-tests.log` 27件・`tmp/pr68-target-dir-guard-ast.log` ASTはいずれも終了0。Rust source/dependency graph変更なし。

任意名targetを許可する安全レビューで、source/.gitにCargo tagがある場合の誤整理リスクを検出した。Git管理領域・tracked sourceを含むtarget・crate manifest・nongit crateのsrcを拒否する境界を追加。実Git tracked source拒否と、tagged nongit srcのwrapper拒否/源ファイル保持を確認。標準justのpatternに新helper回帰を含め、`tmp/pr68-source-ownership-tests.log`32件と`tmp/pr68-source-ownership-final-ast.log`はいずれも終了0。Rustsource/graphは変更なし。


## PATH指定Office workerの追加レビュー対応

裸のworker名はwhich 8.0.6で実行可能ファイルへ解決し、明示相対pathはcwdへ束縛する。解決した同一の絶対pathをhashと実worker起動へ渡し、探索失敗はNotFoundとして返す。XLSX Unsupported判定の順序は維持した。既存の絶対path契約に加えて、実DOCX workerのPATH指定cold変換・独立process restart hitを確認した。cache単体54件、実process契約3件（child専用entrypointは親から起動）、ASTがPASS。手動検査の初回はRUSTFLAGS未指定で中断したため、正しい既存profileで再実行した成功ログを根拠にする。

最終source/graphの `just VERSION=0.5.13 JOBS=2 release-check` は終了0。全既存lint/AST/test/score/semver196項目、3799関数・31286行100%、version/package/publish dry-run/未公開版確認がPASS。証跡: `tmp/pr68-path-worker-release-check.log`、RTK tee1791289754_just_coverage.log。PATH単体/実processは `tmp/pr68-path-worker-unit.log` / `tmp/pr68-path-worker-process-correct-profile.log`。read-only自己レビューに具体的な追加欠陥なし。公開KRR0.4.23での準備検査であり、最終KRR graphのリリース証跡とはしない。

容量guard32件は8afdd69aのUbuntu/macOS/Windows全CIで成功した。40GiB（約2build分）を超える場合だけ通常build前に整理する契約を維持。KatanAのmaintenance/coverage導線を読み、専用coverage targetの分離を参考にした。KatanA側に容量閾値の仕組みはなく、KDVで所有範囲・source保護・排他・使用中保護を追加した。


## fe1c349b追加レビュー対応（全体ローカル検証PASS）

- ネストしたtarget: Cargoが子だけへ生成した所有タグを維持し、タグがなく明示指定されてもいない既定親は子の整理対象へ統合しない。明示された親や所有タグのある親は個別に検査する。全targetに共有inode集計を適用し、inode 0では実pathを使って同じファイルの二重計上を避ける。実ephemeral Cargoで、子の古いartifact削除・再build成功・親の無関係なファイル保持を確認した。親/子のexact容量でも二重計上により整理しない。標準guard35件PASS: `tmp/pr68-nested-target-final-tests.log`。
- 既存entryのsave: destination種類判定後に総使用量を検査し、既存entryには追加容量0、新規entryだけ実封筒容量を加算する。異なる容量ownerで総使用量が縮小上限を超える同key再利用を拒否し、exact容量の既存内容保持・clear後復旧を確認した。保存場所に不正directoryがあれば既存regularの再利用もUnsafeDirectoryとする。最初の単体検査はこの安全検査を迂回する旧期待値で1件失敗し、正常状態と不正状態を分けて検証し直した。修正後cache単体55件PASS: `tmp/pr68-save-quota-unit-resume.log`。
- ASTと自己レビューを確認。補助レビューの「regular再利用はUnsafeにならない」という指摘は、同directory内の不正entryをused_bytesで拒否する実呼出しと上記55回帰を根拠に不採用とした。他に具体的な追加欠陥なし。
- 最終source/graphの `just VERSION=0.5.13 JOBS=2 release-check` は終了0。既存全lint/AST/test/score/semver196項目、3800関数・31292行100%、version/package/publish dry-run/未公開版確認がPASS。証跡: `tmp/pr68-nested-quota-release-check.log`、RTK tee1791295423_just_coverage.log。公開KRR0.4.23での準備検査であり、最終KRR graphは別途検証する。

fe1c349bの3OS CI37469059929は成功（PATH worker解決を含む直前source）。今回の2件修正後のcurrent HEAD review・各thread reply/resolve・3OS CIは結果回収まで未完で継続する。

## 32d1d909の追加レビュー・CI対応

Linuxで実行中imageのatomic置換後も旧processから指紋を読めるよう、/proc/self/exeから同じ実行imageを取得する。更新後pathへのfallbackはしない。他OSの取得経路と明示IOerrorは維持。実行binaryのhardlinkを起動し、そのpathをatomic置換する独立子process回帰を追加した。macOSのcache単体55件とfmtは終了0。Linux実回帰は次のexact HEAD CIで回収するまで未完。

全release-check14641は終了0、3800関数/31293行100%、全既存品質/semver196/package/publish dry-run/未公開確認PASS（tmp/pr68-running-image-release-check.log、RTK tee1791301114_just_coverage.log）。その実行後半にPython/CIのみ追加修正: Ubuntu32d CIのcoverage開始時、復元targetに所有タグがなくguardが終了2。Cargo既知issue12441の既存directoryタグ未生成に一致するため、新規/空のsource-free targetだけを初期化し、CIではcache復元前に実行する。非空の未知directoryは所有扱いしない。新標準37回帰とAST終了0（tmp/pr68-empty-target-final-tests.log / tmp/pr68-running-image-empty-target-ast.log）。Python最新差分は通常pushの全checkと次の3OS CIで再確認する。元32d CIはfailureであり成功と扱わない。

## 239a0757追加レビュー対応（macOS実回帰PASS）

- Macimage: dyldのロード済み起動Mach-OからLC_UUIDを読みSHA256指紋にする。Apple linkerは既定で出力内容hashからUUIDを生成し、静的描画依存を含む再buildを識別する。ASLR/path置換に依存せず、別imageへのfallbackなし。UUID欠損・ゼロ・不正load commandはIo InvalidData。SDK公開C ABIのopaque headerから64bit load commandsを読む。libcの古いMachヘッダ型はdeprecatedでstrict警告になったため使わず、依存追加・allowは行わない。
- root: 構築時のstd::path::absoluteへ固定する。独立子processでcwdを変更し、load/save/clearが元rootへ作用し、移動先の同名directoryの無関係なregular fileを保持することを確認。
- 実binaryをコピーして起動後にatomic置換するMac/Linux回帰。nativeMac単体58 PASS、子専用2 entrypointは親から実起動し成功を確認。final tmp/pr68-mac-image-root-final-tests.log EXIT0。Windowsのcanonical path prefixを考慮し、テスト比較はabsolute属性とcanonicalized physical directoryの一致を確認する。最初のnative58 PASS後、full gate終了後にこのtest-only比較を整えて再検証した。
- full68512 release-check EXIT0: 全品質/semver196/3805関数31333行100%/package/publish dry-run/未公開確認。tmp/pr68-mac-image-root-release-check.log / RTK tee1791305860_just_coverage.log。この後の変更は上記test-only比較で、production source/graph/coverage対象は不変。最新test/fmt/ASTと通常push全check、次のexact3OSで追跡する。
- 239a0757 CI37494294516全3OS SUCCESS。容量37件すべて成功、UbuntuでLinux atomic置換回帰がworkspace/coverage両方で成功（3808関数31313行100%）。Linuxthread4196940044へreply4198304556/resolve、fresh18threadの未対応はMac4197800571とrelative root4197800580の2件のみ。Mac/rootの最終HEADは公開・freshreview/CI回収まで未完。

## 2026-10-15 root祖先symlinkと動的worker版

PR68 thread4283771319: prepare後canonicalizeし構築時の物理rootへ固定。祖先symlink付替え後もload/save/clear対象と無関係ファイル保持を実Unix回帰。最終要素symlink拒否は維持。thread4283771339: Office page keyへ描画時workerSHA256を追加、描画前後に変更を検出するとI/Oエラーで保存拒否。実workerを起動するwrapperをatomic置換し更新/rollback時miss、安定版restart hit、描画中変更時の保存なしを検証。キャッシュ単体59 PASS（子専用2ignoreは親から実起動）、新実worker回帰2PASS。rootテスト行数AST失敗はhelper抽出で修正し、全release-check92549 EXIT0、3806関数31344行100%、semver196 PASS、package952files/publish dry-run/未公開確認成功。ログtmp/pr68-root-worker-release-check-final.log、coverage原文RTK tee1792073898_just_coverage.log。KRR0.4.23準備graphであり最終KRR再検査は別。

## live worker無効化 / Cargo -C

thread4284455689: memory hitより前にworkerSHA256を照合し、変更時はPdfPageCacheを破棄してresource metricsも解放。描画missでは検証・保存が終わるまでrevision未確定として、失敗した描画を次回LRUで誤採用しない。実同session旧→新→rollbackを2件の実worker回帰で検証、cache59件/AST成功。thread4284455700: Cargo -C/attached指定を解釈し相対CLI/env targetを変更後cwdへ結合、所有範囲検査は維持。42件成功、モックは単体テスト専用、実Cargo統合テスト維持。統合後release-check99145 EXIT0、3809関数31360行100%、semver196 PASS、package952files/publishdryrun/unpublished成功。ログtmp/pr68-live-cargo-c-release-check.log / RTK tee1792077201_just_coverage.log。KRR取込後の最終検査を代用しない。

## PDF保存失敗再試行 / Office変換中worker更新

thread4285094776: PDF disk保存失敗時にlive LRUを破棄し元のerrorを返す。容量1byte不足で同session2回Capacity、clear後missで保存しrestart hitとなる実filesystem回帰を追加。thread4285094789: 変換出力取得後（cache hit/miss両方）にworker依存キーを再計算し、開始時と異なれば保存/返却前にIoエラー。実worker変更後のrollbackも旧keyを汚染しない回帰を追加。実worker3件/キャッシュ60件成功。ASTのfile200行制限に合わせretry専用テストmoduleへ分離しAST成功。全release-check57473 EXIT0、3809関数31365行100%、semver196PASS、package953files/publishdryrun/unpublished成功。ログtmp/pr68-retry-conversion-release-check.log、coverage原文RTK tee1792080591_just_coverage.log。KRR0.4.23準備graphであり最終取込後の検査は別。

## root実体の置換検出

thread4285688056: 構築時のdirectory handleをArcで保持し、操作前・lock取得後にpathnameの非symlink性と実体を照合する。Unixはdev/inode、WindowsはFILE_ID_INFOのvolumeと128bit IDを使用し、保持handleにより旧inodeの再利用も避ける。別directory・symlinkへの置換をload/save/used_bytes/clearすべてUnsafeDirectoryで拒否し、無関係ファイルと元entryを保持、元directory復元後は再利用する実filesystem回帰を追加した。Windows API/feature/rename共有は独立静的レビューでも確認。レビュー提案の「saved identity + 各操作で検証」を採用した。操作途中にhost自身がnamespaceを並行置換するTOCTOUまで原子的に防ぐdir-handle相対I/Oは今回実装しておらず、その保証は主張しない（host所有の専用directory契約）。

cache62件/AST成功。全release-check99551 EXIT0、3813関数31390行100%、semver196PASS、package955files15.2MiB/publishdryrun/unpublished成功。ログtmp/pr68-root-identity-release-check.log、coverage原文RTK tee1792084010_just_coverage.log。KRR0.4.23準備graphであり最終取込後の検査は別。修正公開後にthread返信/resolve・fresh review/3OS CI/preflightを回収する。

## root操作中の置換に対するDir相対I/O

thread4286251959: 51a534a2の直前identity検証だけでは操作中のroot置換を防げないため、cap-std4.0.3の保持Dirへ全load/save/lock/used_bytes/clearを固定した。Unixでlock取得後にrootをrename・別directoryへ置換し、同じlocked処理の読込/保存/列挙/clearが元実体だけへ作用して無関係keepを保持する決定的な実回帰を追加。WindowsのDirはFILE_SHARE_DELETEなしで開くためrename自体を拒否し、cache継続利用を確認する回帰を追加。元root復元とpublic入口のUnsafeDirectory拒否も維持する。

一時名生成だけtempfile Builderを使い、実作成はDir相対create_new・Unix0600。write/sync後に同Dirのhard_linkで既存先を上書きせず公開、file close後にDir相対cleanupする。既存先への公開失敗で内容とentry数が不変、readonly lock/Unix readonly directoryでI/O errorかつ内容不変・残骸なしを確認。staticレビューは実I/Oのambient pathname残存なし、Windows share/cleanup・可視性整合を確認。初回ambient capability取得より前のhost側namespace選択はconstructorの入力取得であり、構築後のroot差替えに追従する処理は残さない。

全通常check・semver196はtmp/pr68-anchored-directory-release-check-final.logで成功。最初のcoverageは新I/O error2行未到達で停止し、production/graphを変更せず上記の実権限回帰だけを追加した。cache65件EXIT0(tmp/pr68-anchored-permissions-tests.log)、AST74286 EXIT0、厳格just coverage68445 EXIT0で3823関数31449行100%に復旧(tmp/pr68-anchored-directory-coverage-final.log)。残りのrelease-verify工程を同じproduction/graphで再開し、package957files15.2MiB、publishdryrun、未公開確認43881 EXIT0(tmp/pr68-anchored-directory-publish-precheck.log)。ゲート/除外/閾値の変更なし。最新testsを含む通常pushの全checkとfreshreview/3OS CIは次工程で回収する。

途中の共有空き9.8GiBに対し、通常テスト完了後のown justを工程境界で一時停止し子processなしを確認、KDVの使用済みdev生成物だけcargo cleanして34.3GiB回収、空き41GiBで同processを再開した。他repo/source編集なし、通常40GiB閾値は不変。tmp/pr68-disk-pressure-clean.log。

## 使用容量のinterprocess snapshot

thread4286986682: public used_bytesもload/save/clearと同じ.lockを取得してからused_bytes_lockedへ委譲する。内部locked処理は再取得せずdeadlockを避ける。親の実OSlock保持中に一時ファイルと公開hardlinkを作り、独立handleのreaderが容量計測を返さず待つことをchannelの期限付き受信で確認。一時名削除・lock解放後にstable容量となり、reader join後にTempDirを解放する実filesystem回帰を追加した。初回test alias/expectのcompile/AST失敗はgeneric Result/明示error処理へ修正し、最終cache66件43368 EXIT0・AST71442 EXIT0を根拠とする。

全release-check15994 EXIT0、3823関数31449行100%、semver196PASS、package958files15.2MiB/publishdryrun/unpublished成功。ログtmp/pr68-used-bytes-lock-release-check.log、coverage原文RTK tee1792091503_just_coverage.log。除外・閾値・契約の変更なし。current HEAD公開後の返信/resolve・freshreview/3OS CI/preflightは結果回収まで未完。KRR0.4.23準備graphであり最終取込後の再検査は別。

## 2026-10-20 IronCalc0.9入力互換性と責務分割

公開済みIronCalc0.9.0/office2pdf0.9.1への先行移行で、結合セルの型変更に加え、不正・1x1・重複範囲を上流が省略する差異を検出した。実XLSXの不正ref回帰は修正前に失敗を確認した。既存XML passでmodel経路だけraw範囲を検証し、従来のエラーとgeometryを維持する。streamingでは取得しない。シート名/件数を実modelと照合してからgeometryをmoveし、builderでも件数を照合する。filterもtakeで移動するため不要な複製を増やさない。

既存AST規約に沿ってsession生成・XML filter/merge処理をprivate moduleへ分離した。公開API、入力期待値、品質閾値、coverage除外は変更しない。主担当と独立レビューで所有権・streaming・深さ/名前/件数・エラー経路を確認し、未対応のactionable findingはない。allocation懸念は不要clone除去後に旧0.8のraw文字列保持と比較し、今回固有の退行でないとして撤回した。

途中の全検査44211はClippy条件式、5625はAST行数で停止したため、最終成功の証拠にしない。等価な条件式整理と責務分割後のASTは成功。容量整理と重なった限定検査3286も失敗として回収し、再実行する。最終sourceの限定回帰・全release-check・current HEADレビュー/CIの結果は実際の完了後に追記する。旧KRR0.4.23/旧ZIPを含むgraphの準備検査であり、公式修正版取得後の最終再検査を代用しない。


### 2026-10-20 現行sourceの先行検査

private session/filter/merge分割後、通常checkのRust/Storybook/AST/strict lint、semver196項目、残るprofiling/performance/fidelity/monitor/V8/harness検査は成功した。計測verifierの参照先を新session moduleへ修正し、release契約・計測self-test/検査も成功。実ZIP/実workerの不正worksheet・結合ref・欠損relationshipsと、属性値の不正/欠落・blank trueの回帰を追加し、canonical coverage全テストは成功した。

ただしcoverage20832は終了コード1で、3839関数100%、31593行中1行未カバーのため、全品質ゲート成功とは扱わない。元ログは `tmp/kdv-upstream09-coverage-attribute-edges-final.log`。閾値・除外・商用処理をテスト都合で変更せず、計測根拠の診断を継続する。独立した最終差分レビューでactionable findingはなかった。パッケージ検査・公開HEADの通常検査/review/CIは別途、実結果を回収する。


パッケージの先行検査98034は終了コード0。Rust1.97.1でversion検査、package962 files/15.2MiB(圧縮7.3MiB)、配布packageのbuild、publish dry-run、v0.5.13未公開確認が成功した。`tmp/kdv-upstream09-publish-precheck-final.log`を根拠とする。macOS linkerのcompact-unwind関連warningは残るが、検査終了コードは0。

調査でローカルstable aliasがRust1.97.1/LLVM22.1.6のままと判明し、公式rustup checkは現行stable1.99.1への更新有りを示した。CIのstableと揃えるため、既存の1.99.1(LLVM23.1.1)を検査processだけに指定して再検証する。global defaultの変更、商用処理のcoverage都合変更、閾値緩和は行わない。


### 最終coverageの回収

1.99.1でも集計不足が続いたため、保存したprofdata/binaryからLLVMの関数別report/show-instantiationsを取得した。未カバーはfinishの構造式ではなく、`parse_worksheet`の空filterColumnを完了する32行目で、ZIPのBufReader経路だけ未実行だった。Cursor経路の既存テストだけでは不足していた。compiler不具合という推測は採用しない。

実representative ZIPから空filterColumnを読む肯定回帰と、両workerバイナリで同経路を通って不正merge refを拒否する回帰を追加した。補助93210は終了コード0。続くRust1.99.1の標準 `just JOBS=2 coverage` 70732も終了コード0で、3839関数/31593行100%、未カバー0となった。`tmp/kdv-upstream09-canonical-coverage-complete.log`を根拠とし、商用処理・閾値・除外は変更していない。

複数mergeCellsで空になる従来挙動は、公式IronCalc0.8.3のimport/worksheets.rsのload_merge_cellsが直下mergeCellsの件数1の場合だけrefを収集する実装で再確認した。新parserも同じ挙動を維持する。最新HEADの全通常検査/review/CIは次段階で実結果を回収する。


### 通常検査の容量復旧とworkspace結果

Rust1.99.1の通常検査10774はfmt・容量guard42回帰・strict Clippy・AST・境界/入口検査まで成功したが、workspace testのlink時に容量不足で終了101となった。private既存clean-allを直列で再実行した27435は終了0、空き49GiBへ回復し、source/graph18ファイルのhash不変を確認した。検査対象・閾値を変えず、incremental cacheを無効にしjobs1で失敗段階以降を再実行している。

再実行6955のworkspace段階は28スイート、2352成功・失敗0・既存ignored8で完了した。Storybookと後続script gateは現在実行中で、全通常検査成功とはまだ扱わない。証跡は `tmp/kdv-upstream09-normal-check-resumed-final.log`。並行して別repoのbuildが終了した後、全Cargo cwdとlsofで未使用を確認したKUC cacheのみ既存just cleanで整理し、95657終了0、空き24GiBを確保した。ソースや過去測定証跡は削除していない。

Storybook本体は646成功・失敗0・既存ignored23/filtered24で終了した（実行803.59秒）。6955は既存recipeで分離するmouse1件と後続script gateが未完であり、全検査成功とは扱わない。分離実行は依存から再buildしている。08:22頃の共有成果物消失報告もあるため、原因をKDVのcleanと断定しない。主担当の最後のclean95657は07:31:14Z終了、KDVlogにclean/Removedなし。KDVのDoDや品質基準は変えない。


### 全通常検査の回収結果

6955は実終了コード0。workspace28suite/2352成功、Storybook646成功と分離mouse1成功、release-target/scorecard/descriptor/profiling/performance/fidelity/monitor/V8 singletonリンク契約/subagent harness/crop28回帰が成功した。10774で成功済みのfmt/容量guard42/strict Clippy/AST/入口・境界と合わせて、現行sourceの全通常gateを回収した。18件のsource/graph SHA256不変を再確認した。単一invocationの全check終了0は通常push hookで別途回収する。

既存release-preflight workflowはrelease PRの最新HEADでacceptance artifacts後に完全release-checkを実行し、契約/semver/通常gate/coverage/package/publish dry-run/未公開確認まで含む。残る重複したローカル配布buildを増やさず、この最新HEADのCI実終了も回収する。品質・除外・閾値を省略しない。

### clean/sweepの使用中保護

KatanAのclean保護を参考に、KDVの明示clean/sweepを既存build guardの同一lockへ統合した。実行中なら非blockingで拒否し、所有・使用中検査の失敗も非0で返す。独立レビューで判明したsweepの `|| true` による失敗隠蔽を除去し、cleanの不要なsweep前提も外した。通常buildの40GiB閾値は維持する。実OSのopen-file回帰は、macOS lsofが検出行を出力しつつ終了1を返す場合を発見したため、出力があれば使用中と判断するよう修正した。実OS lock/open-file回帰とWindows未知errno単体検査を含むPython25件が成功した（`tmp/kdv-maintenance-python-checks.log`）。Rustのソース・依存graphは変更していない。その後、実Cargo fixtureを含む標準 `just build-cache-script-test` が48件成功・実終了0となった（`tmp/kdv-maintenance-full-script-check.log`）。最新HEADの全品質ゲートは通常push hook/CIで引き続き回収する。

### Issue参照不足の早期検出

通常push77570は既存品質delegateを通過した後、コミット本文のOpen Issue URL欠如で拒否された（push全体は終了1）。Issue #69へ現在の公開依存・API移行・manifest/lock・検証・未解消の推移ZIP経路を記録し、参照をコミット本文へ補完した。単独の実remote入力によるIssue検査は終了0だが、通常pushと最新HEADの公開側レビュー/CIはまだ未完である。

この不足を重い検査の後で検出しないよう、delegate前にも同じIssue検査を追加した。既存のdelegateと最終Issue検査は残し、検査中のIssue状態変化も最終段で拒否する。旧hookでは、実Git fixtureの参照欠如でも記録delegateが起動してしまう回帰が終了1となった。修正後は、malformed入力/参照欠如で未起動、正常default branch入力で終了0とstdin保持、delegateがfixtureのoriginを書き換えた場合の最終拒否、既存delegate失敗伝播を確認した。標準 `just release-governance-check` は終了0（`tmp/release-governance-check-current.log`）。独立レビューでも認証・scope・品質/final guardの弱体化は指摘されていない。Rust source/manifest/lockの18件は検査済みhashを維持する。

### 最新HEADのCIと自動clean後の再生成

23bb9f65の通常push55713は全hookを通過し終了0、Codex receipt6292503608は新規指摘なし。全レビューを取得し27thread未解決0、CI39432744843のmacOS/Linux/Windowsが成功した。Linuxの関数3843/行31575はともに100%・未カバー0で、region98.71%と222関数のprofile mismatch警告もログに保持した。標準native coverageの3839関数/31593行100%と対象OSを区別する。

公開前検査39432745015はsemver202成功・58skip後、自動cleanの次に起動した補助ツールが入れ子出力だけを生成し、rootの所有タグが失われたため後続Storybook入口で安全停止した。完全release-check・配布検査は未完であり、3 OS CI成功で代用しない。clean前の所有・使用中確認は維持し、成功した自動clean直後の空targetへタグを準備し直す修正と実入れ子Cargo回帰を行う。未所有の非空directoryへタグを付けて削除可能にする対処は採用しない。

修正は自動clean成功後の `prepare_targets` 再実行1行に限定した。実Cargoの依存なしfixtureをoffline/1jobで入れ子targetへbuildし、旧実装では初回終了0でもroot tagなし・次回終了2を確認した。修正後は2回の閾値clean/buildとも終了0、root tagとbinaryを確認した。Windowsではbinaryに.exeを付ける。標準49件は終了0（`tmp/pr68-cache-repair-final-script-check.log`）、OpenSpec strictも終了0。独立レビューで、同一lock内・clean成功時だけの再準備、例外時のsafe failure、非空未所有directory拒否、40GiB閾値、wrapped commandの終了値伝播に問題なしと確認した。Rust source/graph18件は不変。通常pushの全品質検査と修正後HEADのreview/CI/full preflightは引き続き実結果を回収する。

### 所有タグsymlinkの拒否

b834の通常pushは終了0。正式レビューでCACHEDIR.TAG symlinkの追従が指摘された。実Cargoによる旧maintenance経路は終了0で未所有出力を削除した一方、旧automatic経路はCargo側がsymlinkを拒否して終了2となった。guard自体をlstat＋通常ファイル判定へ変更し、両経路を所有検査で拒否する。タグsymlink・外部署名ファイル・無関係出力のbyte内容を保持する回帰を追加し、標準51件が終了0。fixtureの外部署名も各一時container内に隔離した。Rust source/graph18hash不変。current formalreview/threadsを全取得し未解決1件を確認、修正push後に該当threadへ返信して解決する。

### 所有タグの同一ハンドル検査

dbd5の通常pushは全hook終了0、symlink指摘へreply/resolve済み。最新Codexレビューはlstatとread_text間の置換を新たに指摘した。実lstat結果を取得した直後に署名ファイルをsymlinkへ置換すると、旧maintenanceは終了0で無関係出力を削除した。POSIXではNOFOLLOW/NONBLOCKで開き、同fdのfstat/署名読取を行う。Windowsのinode比較fallbackは、元ファイルをrenameして同inodeへのリンクにすると通るため不採用とし、CreateFileWのOPEN_REPARSE_POINT・同HANDLEのdisk種別/非reparse/非directory属性を確認してFDへ移管する。未移管HANDLEはfinallyで閉じる。主担当がWindows責務をmoduleへ分離し、実open前のNONBLOCK検査と外部bytes保持assertを補完した。標準56件は終了0、MacではWindows native専用1件をskip。Windows実CIと修正後全品質ゲート/新review/各threadreplyresolveは未完であり、旧HEADの完走を代用しない。
# 2026-10-22 公開IronCalc更新とtarget実体固定

局所レビューはPASS。IronCalc/baseを公開0.9.1へ揃え、古いbaseを拒否する公開契約回帰を追加した。最初のコンパイル失敗は公開済みbaseへのlock更新で解消し、上流依存下限はIronCalc #1578へ報告した。

targetの全体整理はPOSIXの全ancestor no-follow/FD相対操作、Windowsのancestor/target/tag HANDLE保持と同HANDLE削除へ変更した。root/tagを維持し、選択clean/sweepを全体削除へ読み替えず、target作成前に拒否する。40GiB閾値は維持。実target差替えとPOSIX中間ancestor差替えのREDを再現し、修正後の標準script suiteは64件成功（MacではWindows専用等4件skip）。直接guardを呼ぶ既存fixture2件は、通常入口と同じく一時pathを先にresolveする。判定や期待値は緩和していない。独立レビューの追加指摘は0。

修正graphの先行通常検査はfmt/strict Clippy/AST/公開境界/全workspace・Storybook/Office検査まで成功（記録された36 suiteで3003 PASS、失敗0）。最後のV8実リンク中に容量不足となり、自分のPGID23220だけを中断したため全体はexit130であり、通常検査完了とは扱わない。その後private直下の既存clean-allは最終exit0、Rustプロセス0、空き約34GBを確認した。最終sourceの通常push hook、3 OS native CI、release-preflightのcoverage/semver/package/dry-runを引き続き回収する。Windows nativeの成功も未確認。

証跡: `tmp/kdv-ironcalc091-base091-anchor-check.log`、`tmp/kdv-anchor-final64-python.log`、`tmp/kdv-anchored-cleanup-independent-review.md`、`tmp/kdv-capacity-recovery-verified-20261022.log`。KRRとoffice2pdf/forksの修正版公開待ちは維持し、古いKRRで公開しない。

追加レビューで、明示`--target-dir`へ既定targetも併合する経路と、manifest指定を無視する経路をRED再現した。通常pushは自分のhook PGID22733だけを中断し（push終了1、remote HEADは01fbのまま）、CLI単一target > 環境target > repo既定targetに限定した。manifest/global/重複target/guard専用指定は安全に拒否する。help/dry-runはtarget/tagを作らず委譲する。標準script suite71件成功（Macで4件skip）、独立レビュー追加指摘0。旧tag競合回帰は、未対応manifest指定を外した通常cleanで同じ差替えを検証するため、期待値を変えずに維持した。最終sourceの通常push/native CI/全公開前検査は引き続き未完であり、今回の局所成功で代用しない。証跡: `tmp/pr68-explicit-target-red.json`、`tmp/kdv-target-selection-full71.log`。

## 2026-10-22 coverage専用target刷新の整合

現guardが選択cleanを拒否する一方、coverage-v8-refreshがclean -p v8を呼ぶ既存矛盾を確認した。専用targetを明示した全体cleanへ変更し、通常target・両方の所有タグを保持する実filesystem回帰を追加した。標準72回帰はexit0（Mac限定skip4）、公開契約self-test/staticはexit0、旧選択recipeを拒否する契約回帰も成功。coverage関数/行100%・未カバー0を変更していない。証跡: tmp/kdv-coverage-refresh-script72.log、tmp/kdv-coverage-refresh-contract.log、tmp/kdv-coverage-refresh-contract-regression.json。43f21303の通常push全ゲートはexit0だが、この追加修正後の最新HEAD CI/preflightは別途回収し、過去の成功で代用しない。

## 2026-10-22 fresh review: POSIX lsof依存

PR68 thread PRRT_kwDOSTfBFs6u5iVd（discussion_r4340478042）のP2を採用。Linux共通依存導入へlsofを追加し、最小Ubuntu/DebianとmacOS PATHの準備をdocsへ明示した。検査ツール不在の安全側拒否は維持し、導入要求の診断を追加。PATHから実ツールを除く回帰で終了3と出力/タグ保持を確認、標準73回帰exit0（Mac skip4）。証跡: tmp/kdv-lsof-dependency-script73.log。最終commit push後に当該threadへreply/resolveし、fresh review/CIを再回収する。

## 2026-10-22 Windows native失敗の修正・再検証待ち

f496 CI Windows nativeは2件失敗（既存writerがあってもmetadata-only pin成功、reparse buffer invalid4392）。新4d通常push52075は、この実失敗回収後に親gitpushを確認した自身hook PGID75568だけSIGINTしexit1、成功に数えない。pinのdelete=FalseへLIST_DIRECTORYを加えて共有検査へ参加させ、子削除は既存DELETEで参加する。失敗時もcontextでHANDLEを閉じるテストへ修正。junction bufferはsubstitute/print両名とNUL/byte offsetを構成しoutside正例を先に確認、同一bufferの属性handle拒否と実再帰DELETEhandle中のwriter拒否を確認する。macOS標準73件はexit0（native等skip4）だがWindows成功を代用せず、native CI再回収待ち。容量回帰をCIのharnessより先行し、省略なしで失敗を早期検出する。Microsoft一次資料: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea 、https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-iochecklinkshareaccess 。

## 2026-10-22 Windows属性専用handleの実変更を受けたcapability化

bb78のnative再回帰では旧writer拒否は成功した一方、FILE_WRITE_ATTRIBUTESだけの既存handleによる有効FSCTL_SET_REPARSE_POINTが成功した。共有制御だけで属性変更も防げるという仮定を撤回し、外部非削除というDoDを維持して方式を修正する。volume rootだけ絶対openし、祖先/target/tag/子すべてをNtCreateFile RootDirectory+一成分名+leaf no-followで開く。NtQueryDirectoryFileで保持HANDLEから列挙し、削除/readonly解除も同HANDLEに限定する。所有tagは相対openした保持HANDLEから署名を読み、前後に祖先/root/tagのreparse化を検査する。native回帰は同一valid bufferのoutside正例→live変更成功→stale directory属性での実削除又は安全拒否→外部sentinel保持を確認し、静的junctionのfull-purge成功も追加した。macOS75件exit0（Windows専用等skip6）、x64構造size16/48/16と主要offsetを確認。native成功の代用にはせず再CIで回収する。証跡: tmp/pr68-bb78ec0f-windows-job.log、tmp/kdv-windows-relative-io-script75.log、tmp/kdv-windows-relative-io-layout.json。

root自体のreparse変更もnative回帰へ追加した。OSが5/32/145で防止した場合は通常整理成功、変更が実成功した場合は保持rootの属性検査でELOOPを返して整理を拒否し、どちらも外部sentinelを保持する。通常の有効junction full-purge正例は別回帰で必須とするため、invalid bufferや常時拒否を安全成功として代用しない。

## 2026-10-29 公式Office ZIP9修正版の先行採用

局所自己レビューPASS、追加指摘なし。利用側の観点: Office/派生の公式修正を採用し旧ZIPを除去、既存emoji fallbackを0.37のfeature明示で維持する。自己否定の観点: 以前の4aebゲートを新graphの証跡に流用せず、check99637のactual EXIT0と新consumer契約self-testを回収した。悪魔の代弁者の観点: 未公開patchやforkの直接追加を避け、公式parent0.10.0をexact選択する最小変更とした。consumerは旧ZIP・旧派生の併存も拒否する。ユーザー依存更新を保持し、Rust本体・品質閾値・reference・skip条件は変更していない。

検証: `tmp/kdv-office010-check-feature-fix.log`、`tmp/kdv-daily-publication-20261029.json`、registry consumer self-test EXIT0、`git diff --check`成功。新HEADの全CI/公開前検査/reviewは別途回収し、KRR0.4.23のまま公開しない。

## 2026-10-29 Linux PPTX行送り検査の測定原因修正

HEAD537393c9のUbuntu CI job123188634257は、代表PPTXの黒画素帯が[(136,146),(160,168),(182,190),(198,211)]となり、最終の画素中心間隔18.5ptが19–24pt条件を外れて失敗した。実PDFを比較すると、LinuxのCarlito/NotoSansCJK SCとmacOSのCalibri/MicrosoftYaHeiで字形は異なるが、baselineは双方393/371/349/328pt、行送り22/22/21ptで完全に一致する。Linux公式engineの独立実行53949はEXIT0。証跡: `tmp/pptx-office010-linux-macos-baseline-diagnostic.json`、`tmp/office010-linux-layout-diagnostic.log`、`tmp/pr68-537393c9-ubuntu-failed.log`。

字形の黒画素中心をbaselineと同一視した検査の誤判定を修正した。従来の38–48（2倍値）と同じ19–24ptの許容幅を、既存の実worker/PDF glyph probeで直接測定する。4段落の全テキストと4 baselineを必須とし、実PDFの第三段落だけを上へ移動して行間を1ptへ圧縮する入力も拒否する。既存のpixelによる4行非重複・日本語3字の可視性/字形区別・タイトル26pt/明示autofit/anchor検査は保持した。商用コード・reference・閾値・skip・coverage条件は変更していない。

追加2回帰72638 EXIT0、関連実worker契約7352 EXIT0（12+7件）、strict Clippy/AST80168 EXIT0、diff check成功。局所自己レビューは利用側の行送り契約、測定の誤判定、実PDFを使う最小変更の3観点でPASS。旧HEADの残CI/preflightは修正済みの旧測定を実行するため停止し、watch37892/53139はexit1、成功に数えない。新HEADで全通常hook・3OS CI・coverage/semver/package/dry-run・current reviewを再実施する。
