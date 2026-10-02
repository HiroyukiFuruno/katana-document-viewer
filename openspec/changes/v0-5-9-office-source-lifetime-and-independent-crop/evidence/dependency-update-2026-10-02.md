# 依存更新の証拠台帳（2026-10-02）

## 最新追記（KUC0.4.1）

下記の元監査はKUC0.4.0時点の記録。後続でmainがGitHub Release `v0.4.1`（2026-10-02T07:38:30Z）と `cargo info katana-ui-core@0.4.1` をライブ確認し、workspace/coreのexact pinを `=0.4.1`へ更新した。

`cargo update -p katana-ui-core --precise 0.4.1` はexit0。registry checksumは `f8fd7856249d96c0ce65e68151a327d82511af70a8fbe7416058a1a0ba8e9d5c`、更新後lock SHA256は `3b28f987041f275a5e296930330e0971a27275a7d3ddcb5563ffe804d2d75ff8`。path/git overrideはない。

依存契約、document boundary、回帰fixtureも同exact版へ追従し、release-contract/recovery-DoD self-testとBash構文検査はPASS。KUC0.4.0 graphの結果は履歴として別記し、新KUC0.4.1へ流用しない。最新graphを別途再検証し、通常check、strict Clippy/AST/fmt、semver196pass/58skip、release-contract-check、V8 singleton152.2.0、Office fidelityがexit0。厳格coverageはfunctions3739/3739・lines30683/30683で100%。実5Office cold141248KiB、最大PPTX11反復cold98496/warm4336KiB。完全release-check/公開/fresh consumer/下流受入とIssue58の独立4評価は未完了。

以下は元監査時点の記録であり、そこで「現状」「未検証」とある項目はこの最新追記による再検証状況を優先する。

完全 `rtk proxy env CARGO_BUILD_JOBS=1 JOBS=1 ... just VERSION=v0.5.9 JOBS=1 release-check` はexit0。通常check・再coverage・semver・package verification・publish dry-run・0.5.9未公開確認まで通過した。raw: `issue59-rss.oBZVZI/release-check-v059-kuc041-final.log`。macOSのdebug package worker linkには`__eh_frame section too large (max 16MB)`の`linker_messages`警告が1件あり、compilerによるとこのlintは`-D warnings`の対象外である。これを警告なしとは報告せず、allow/除外も追加していない。package linkは成功し、製品のrelease worker実測は別の固定binaryで実施した。

mainの `rtk proxy cargo update --dry-run` は「latest compatibleへの更新0件」、lock SHA不変を確認した（`dependency-compatible-final-dry-run.log`）。Wegener（01a0fbe8-5d45-79e0-a8ed-b7b60d634346、gpt-6-luna/low）は指定4crateと既知generic-array制約だけをreadonly照合した。全直接依存のmajor監査完了という広い結論には採用せず、mainが別途workspace/root-deps監査を継続している。

mainが `rtk proxy env CARGO_BUILD_JOBS=1 JOBS=1 cargo outdated --workspace --root-deps-only --format json` を実行してexit0を確認。kdv-linter/katana-document-viewer/kdv-storybookの3crateともdependencies=[]であり、この直接依存監査では追加candidateは0件。lock SHA3b28f987は不変、manifestのversion差分も既存候補のまま。raw: `dependency-major-final-audit.json` / `.log`。推移依存generic-arrayの既知exact境界は解消したと扱わず、path/git overrideも加えていない。

公開参照: https://github.com/HiroyukiFuruno/katana-ui-core/releases/tag/v0.4.1 、https://crates.io/crates/katana-ui-core/0.4.1 。

## 対象と判定

対象は `release/v0.5.9` の KDV workspace 依存更新候補。ここではソース、registry package、既存ログ、現在の Cargo manifest/lock を照合した。依存更新の採用完了やリリース可否を示す台帳ではない。更新後 graph に対する全 gate、新 graph の RSS、4項目の独立スコアは未検証である。

## 直接依存の候補評価

| 依存 | manifest / lock の現状 | 根拠と評価 |
|---|---|---|
| `katana-render-runtime` | caret `0.4.22`。lock は registry `0.4.22` | `Cargo.toml` と `Cargo.lock` が一致。caret 指定は0.4系列の互換patch解決を許すが、今回のlockで固定される実体は0.4.22。Issue #59 の製品A/Bログは従来のlocked graphで得た結果であり、この更新候補graphのA/Bへ外挿しない。 |
| `katana-ui-core` | exact `=0.4.0`。lock は registry `0.4.0` | manifest と lock は一致。今回の候補では現行exactを維持している。 |
| `v8` | exact `=152.2.0`。lock は registry `152.2.0` | manifest と lock は一致。mainの `rtk proxy cargo tree -i v8 --locked` はKDV0.5.9とKRR0.4.22が同じ152.2.0を参照することを確認。singleton checkerもPASS。公開後のfresh registry consumerは別の未完了工程。 |
| `office2pdf` | exact `=0.8.0`。lock は registry `0.8.0`、checksum `f8e88ceb4a5cb9095d2d7200368d52532090e06ed33fc3c2a94e4cf2a80f76f7` | `office2pdf-v0.8-evaluation-lock.log` は0.7.0から0.8.0への更新を記録し、`office2pdf-live-monitor.json` は upstream/crates.io 0.8.0を `eligible` と記録する。registry source `office2pdf-0.8.0` のREADME、config、PPTX slide parser/testsを読んだ。hidden slides は opt-in のみで出力され、既定では除外。0.7.0 source も既定で除外し、README/APIにも同じ挙動が示されるため、既存の既定動作と一致する。 |

## 汎用依存の更新可能性

Cargo.lock の現在の graph には `crypto-common 0.1.7` と `generic-array 0.14.7` が存在する。registry source `crypto-common-0.1.7/Cargo.toml` は `generic-array = "=0.14.7"` を直接固定している。従って generic-array 0.14.7 を独立に更新することはできない。

`generic-array-update-evaluation.log` と `generic-array-version-boundary.log` は旧office2pdf0.7.0 graphでの失敗を記録している。mainが新office2pdf0.8.0 graphで `rtk proxy cargo update -p generic-array@0.14.7 --precise 0.14.9 --dry-run` を再実行し、同じexact境界によるresolver失敗を確認した（exit101、`generic-array-office080-evaluation.log`）。0.14.9への更新候補が要求 `=0.14.7` を満たさないという結果であり、0.14.7がregistryから取得不能という意味ではない。依存経路は `crypto-common 0.1.7 → cipher 0.4.4 → aes 0.8.4 → umya-spreadsheet 2.3.3 → office2pdf 0.8.0`。patch/overrideや閾値緩和は加えていない。

最新互換lock更新には `glam 0.33.11 → 0.33.12` と `lazy_static 1.5.0 → 1.5.1` を含む。manifestに旧KRR0.4.20、lockに0.4.21があったため、今回両方を公開0.4.22へ進めている。

同じ評価ログに含まれる既存依存ツリーは重複version（例: `alloc-no-stdlib 2.0.4/3.0.0`、`crypto-common 0.1.7/0.2.2`、`base64 0.22.1/0.23.1`）を記録している。これらは依存ごとの互換境界が異なる証拠であり、重複だけを理由に一括統一しない。`v0.5.9-tree-duplicates.log` は v0.5.9 tree の重複一覧を別途記録する。全面的な汎用依存更新が成立したとは判定しない。

## Manifest / lock の照合

読み取り時点の `Cargo.toml` は package/workspace version 0.5.9、KRR caret 0.4.22、KUC exact 0.4.0、V8 exact 152.2.0、office2pdf exact 0.8.0。`Cargo.lock` の workspace package version と該当 registry package version はこれらに一致している。lock の office2pdf checksum は上記の値。

同時点の既存 `git diff -- Cargo.toml Cargo.lock` には manifest の0.5.8→0.5.9、KRR 0.4.20→0.4.22、office2pdf 0.7.0→0.8.0、lock上の関連package更新がある。これはユーザー指定により既に存在していた作業差分の記録であり、本台帳作成で manifest/lock は変更していない。Cargoコマンドによる再解決・ビルドは実施していないため、最新 manifest/lock の再生成整合性は独立には検証していない。

## 未検証・未完了

- 全品質 gate: fmt、AST lint、strict Clippy、全test、coverage 100%、semver、score gate、release-check。
- office2pdf 0.8.0を含む新locked graphでの依存再評価、実製品A/B、RSS測定と元受入。
- 新graphを使った Issue #59 のRSS再計測。既存のA/B値は以前のlocked graphに対する測定であり、新graphの証拠ではない。
- Issue #58 の4独立スコア各95点、既存reference非上書き、採用判定。
- 公開crateのchecksum、fresh registry consumer、配布版・下流受入。local graphのV8 singletonは上記のとおり確認済み。

従って本記録の結論は「候補versionとその根拠、および既知の汎用依存境界を記録した。候補依存の採用完了は未確認」である。

## 参照した証拠

- `Cargo.toml`、`Cargo.lock`（manifest指定、lock version/checksum/dependency tree）
- `issue59-rss.oBZVZI/office2pdf-v0.8-evaluation-lock.log`
- `issue59-rss.oBZVZI/office2pdf-live-monitor.json`
- `issue59-rss.oBZVZI/generic-array-update-evaluation.log`
- `issue59-rss.oBZVZI/generic-array-version-boundary.log`
- `issue59-rss.oBZVZI/generic-array-office080-evaluation.log`
- `issue59-rss.oBZVZI/v0.5.9-tree-duplicates.log`
- registry source: `office2pdf-0.7.0`, `office2pdf-0.8.0`, `crypto-common-0.1.7`, `generic-array-0.14.7`（Cargo metadata、source README/config/parser/tests）
- `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/tasks.md`（未完了gateと独立評価のDoD）
- `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/evidence/product-ab-2026-10-02.md`（既存graphの製品A/Bと残DoD）
