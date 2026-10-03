# v0.5.10候補の検証境界

## 差分限定レビュー

公開型/処理順/100ms周期/RSS上限/kill/join/CRC/後方filterを維持する。readonly Hookeは再現可能P0/P1なし、mainが差分と実ゲートを照合した。sysinfoの現workspace featureはsystemのみ、実KatanA consumerもsystem/diskのみでmultithreadなしをcargo treeで確認した。新しいallow/exclude/mock/sandbox上限緩和はない。待機開始testのready送信がpark済みを保証しない制限は名称/設計へ反映した。

コード差分限定レビューと候補の全release-checkはPASS。ただしcurrent-HEAD cloud review/CI/公開/元受入は未完なので、リリースReady・全DoD達成の判定ではない。

## 実行済み

- current release lib/worker build、focused監視5test、fmt/strict Clippy/AST/OpenSpec/段階契約、semver196/196が成功。
- 通常core1960PASS/1ignored、Storybook通常640PASS/22ignored/24filteredとisolated mouse1PASS。
- 現候補の同じStorybook test binaryを通常target clean前に凍結しSHA256 `5430a501f99448f3a5f1b9513307577296953e1748ed5c30a7fa49933ddf1405`で追加検証した。旧0.5.9 scorerの結果を流用しない。
- visual reference3test: sample99/diagram100、既存threshold95を維持、export参照もPASS。raw SHA256 `837e2aa52fa2b36d12c471cf1344c6f0d950ecd512b2b84e1b11dec409e8c9b2`。
- 通常testで除外する他のscore/metrics/typography/overlay20testは20PASS。raw SHA256 `1b9ba69f0242e4b347da80feb2ab619a874e07b30a022a6a4f6800690746c48b`。
- ignored export parity2test: sample/diagrams各PASS。raw SHA256 `ef5d710e10b91604aba5961b41646b5f6b597955841be0a5d096697901cdeb64`。
- 外部current crop4入力を同候補scorerへ実投入し1PASS、visual100/threshold95/average99。producerは公開0.5.9の既存root生成時identityを維持し、後commit/新graphへ書換えない。raw SHA256 `f3e46f2a10b59f78abdd9a1fd37b228af12b34446aa3458c356cc87bea2be865`。非visual三分類は未評価のまま。
- sample/diagrams reference PNGのSHA256は各 `7183c95d24e7910dfb088f837cb8b37c8faa98c4f01ad51441c1287009ee91dc` / `c6b6326f76374ea2712ca634b877ece7162acf7ea949ec1c1c4ee23b0fc31577`。reference編集なし。
- 現候補親/同release workerで代表DOCX/XLSXの独立LibreOffice/Poppler原本比較を実再生成し、既存tolerance検証とverify-recordがexit0。record SHA256 `4e11d8a9a4e8ef4b9934bbe29b54d2878885f24f33e229601af8e9e245ba57ed`。代表fixture/既存toleranceの比較であり元6原本/packaged-mainのfidelity代替ではない。DOCX 2page一致、XLSX 2sheet一致/欠落要素0。XLSX print page対continuous_gridやDOCXの既存寸法差を解消したとはしない。

## 失敗と再実行

最初のrelease-checkはhandoff.md不足でexit1、二回目は旧完了tasksへ追加した公開項目の証跡ラベル不足でAST exit101。単独harnessではverifyコマンド形とrepo相対file参照の不備も検出した。handoff・実行済みrtk bashの再現コマンド・実在file参照・証跡ラベルを補完し、単独harness/ASTはGREEN。ゲート自体の変更や除外で通していない。旧FAILログを保持し、同じ全release-check入口を再実行する。

## 全入口の終端確認

同じ全入口のsession23701はexit0。コマンドは `rtk proxy env CARGO_TARGET_DIR=tmp/issue59-rss.oBZVZI/cargo-target CARGO_LLVM_COV_TARGET_DIR=tmp/issue59-rss.oBZVZI/coverage-target just JOBS=2 VERSION=v0.5.10 release-check`。raw SHA256 `864a2e7fb237890b28865ab1c9af1e5881ed512cf31d8025af4e050cf8c642d4`。

coverage原表TOTALはfunctions3741/3741・lines30728/30728で各100%、uncovered0。regions41329中495未到達（98.80%）は既存gateの対象外であり、全region100%とは報告しない。既存functions/lines必須100%を変更していない。package/publish dry-run/未公開確認はPASS、915fileに新model moduleを含みprivate scratchを含まない。macOS package worker linkerの非fatal `__eh_frame` compact-unwind warningをrawに保持し、無警告とはしない。

既存release-verifyの通常target cleanで再生成可能な38.5GiBを整理した。原本/参照/ソース/凍結worker/probe/scorer/rawは保持。追加cleanupで消していない。

依存はdirect outdated全3package deps空、lock互換cc1.6.0/font-types0.12.6/uuid1.27.0を採用。KMM0.2.3/KAL0.5.2の下限は既存lock解決版に合わせた。lock SHA256 `52443bbe3b42ffb535cd07dc4e8ebd61a84a5ee41e2ec44dd2e050c19c80f796`、registry KUC0.4.1/KRR0.4.22/office2pdf0.8.0/V8 singleton152.2.0。KRR PR105はDraft OPEN/HEADf636ee97で全6check SUCCESS、GitHub最新公開は0.4.22。未公開修正をoverride採用しない。

commit/push/Draft current-HEAD review/CI/公開/fresh consumer/元DoDは未完。HTML元closeは公開KRR修正後のroot担当境界で重複せず、必要なKRR版の公開境界を再確認して採用する。
