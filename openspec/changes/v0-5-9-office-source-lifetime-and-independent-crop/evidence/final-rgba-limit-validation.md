# RGBA互換修正後の最終再検証

対象は未commitのrelease/v0.5.9候補。旧完全gateの成功を流用しない。
KUC exact0.4.1/KRR0.4.22/office2pdf0.8.0/V8152.2.0のregistry graph、lock SHA256 `3b28f987041f275a5e296930330e0971a27275a7d3ddcb5563ffe804d2d75ff8`。

## 実行済み

- 通常全件check session82463 exit0: core1956pass/0fail/1ignored、実IT、Storybook通常経路640+1pass/0fail、provenance27件、fmt/strict Clippy/AST/境界/V8 singleton。
- coverage初回は未カバー1行でexit1。寸法エラーを親の応答処理から伝播するunitを追加し、既存製品コードと閾値・除外を変えず再検証した。
- coverage再実行exit0: core1957pass/0fail/1ignored、functions3740/3740、lines30702/30702、未カバーfunctions/lines0。regions98.80%（495未カバー）は別の指標で、既存gateはfunctions/lines100%と未カバー0を要求する。
- release build session61430 exit0。新worker/rlibを通常Cargo target外へ凍結して、診断driverを再compile（session86850 exit0）。製品worker SHA256 `ba7d19d186101a1de33616e7e66f0c505cd8515edfa38d490443138d8b2bcd60`、rlib `94fece11008f3b7545ac0bc28486c7f34475616b9555ee6e1c6f55398985022c`、driver `37102567f20a7956b3e0911a12337dea842e00d4b2ecedec9ae209afd3fdca19`。
- 同実5Office・one mode: session13197 exit0、cold7008→final142672KiB、増分135664KiB、最終live malloc71808bytes。全5closeで8counter0。旧KUC0.4.1候補とmetadata/frame10行のdiff exit0。
- 最大PPTX・two mode11反復: session80548 exit0、cold7008→初回close101104→final103616KiB、cold96608/warm2512KiB、最終live malloc69760bytes。全11closeで8counter0、旧KUC0.4.1候補とmetadata/frame33行のdiff exit0。
- 追加の5Office・two mode: session57760 exit0、cold7040→final154160KiB、増分147120KiB、各close8counter0。この15行の出力はone modeの10行とは同じ測定条件でなく、追加倍率の確認として区別する。
- 元のcold196608/warm65536KiB上限を変更していない。RSS測定中は自己所有のCargo build/coverageを実行していない。
- Office fidelity session83791 exit0。固定外部oracle/fixture/tolerancesに対してtolerances_met=true、failures0。DOCX2ページのmean MAE0.02525295/RMSE0.10328595、XLSX2sheetのtext/font/fill/border/merge欠落・不一致0、row track delta0.0037456584413520628を維持。
- compatible dependency dry-run session95279更新0、Cargo.lock不変。Git config/index SHAも前後不変、stash5OID不変。
- 全Storybook registry consumer session46100 exit0: core1957pass/1ignored、Storybook664pass/0fail/22ignored。通常checkでskipする24件も含む。外部4入力を同じrunへ明示してcrop採点1pass/0fail、visual100/95・average99。raw `storybook-current-crop-v059-rgba-limit-final.log`。旧producer7b5224d4のimmutable artifactを現在のscorerで再評価した証拠であり、未生成のroot新HEADや未公開0.5.9由来artifactの評価ではない。

## 証跡の訂正

以前の台帳の「5Office metadata/frame13行」は件数の誤記だった。対象rawログの`^cycle=`は10行（5metadata+5frame）、warm11反復は33行。行数の訂正は既存入力・期待画素・上限の変更ではない。追加two modeは15行あり、one modeと混同せず改めて同条件で再測定した。

## 未完了

完全release-check session55336のsemverは196pass/58skip。ただし続くAST検査でtestファイル216行を検出しexit101。寸法検証3testを期待値を維持して専用fileへ分割し、fmt/AST session26648 exit0。製品処理や検査条件を変えず、session97206・`release-check-v059-dimension-split-final.log`で完全gateを再実行した。

session97206は分割後のsemver196pass/58skip、通常check core1957pass/1ignored・Storybook640+1pass/0failと全harness/provenance/V8を完了した。strict coverageもfunctions3740/3740・lines30702/30702・未カバー0、regions98.80%を維持して成功。全summaryは`coverage-v059-dimension-split-final-summary.log`。package914files/15.0MiB（compressed7.3MiB）の検証ビルドとpublish dry-runも成功し、完全release-checkはexit0。KDV0.5.9未公開の確認まで完了した。macOS debug linkの既存`__eh_frame`容量警告は残るが、検査条件は変更していない。生成crate内に新raster source/testsを確認し、私有scratch/worker/driver/AGENTS混入のpatternは0件だった。公開/下流受入の成功とは区別する。

current-HEAD Draft review、3OS/preflight、公開・fresh registry consumer、元GUI/mixed HTML-PPTX受入、Issue58の独立三項目と採用、Issue closure・stash/branch整理は未完了。

rawはprivate `issue59-rss.oBZVZI/`の`*-rgba-limit-final*`へ保持し、私有入力・binary・生ログはcommit/packageへ含めない。
