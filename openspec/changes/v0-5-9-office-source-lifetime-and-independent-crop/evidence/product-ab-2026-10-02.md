# 製品A/Bと責任境界の追加実測

## 固定条件

既存HEAD6ae11c2、version0.5.8、同一locked registry graph、同じ実PPTX3入力/worker/scale bits1067776592・1068265927。私有原本・絶対pathは公開しない。閾値・deadline・画質・allocator・既存referenceは変更していない。

- lock SHA256: `0cd6ea02c5568ffd3e26d384062fd44cc45962bd1564d541908fb92186f8dd5e`
- worker SHA256: `be0283a71ce5e41cca487b46bd9818b4a6ef0dce2f9921907c81286595ca5a2d`
- baseline rlib SHA256: `32c66d407b1754c90eea98247396635c5a58ad4b14c26c7489ded0d39c6325a9`
- source-lifetime候補rlib SHA256: `a14b4f2f8acb3ae02dd7e8e43578c4b38364d73cc7a2adc8b1f4ded1813e18e4`

## 原本寿命の製品回帰

実workerによる変換・描画が完了した同じtestで、baselineはPDF decode完了traceより原本dealloc markerが後にあり1pass/1fail。候補はmarkerが先になり2pass/0fail。コードの所有権移動はPDF解析開始前のdropを保証するが、trace自体はdecode完了点であり開始点とは呼ばない。

通常Cargoの`office_static_adapter::tests`は5pass、`--test office_source_lifetime`は2pass。metadata/capabilities/diagnostics/page countと、3入力各2倍率の実RGBA寸法・診断FNV64（非暗号学的digest）がbaseline/candidateで一致した。全closeでKDVの8counterは0。

## RSS効果は未確認

3入力cohortのtraced実測（KiB）:

| source lifetime | cold | final | delta | final live malloc |
|---|---:|---:|---:|---:|
| baseline | 7024 | 294592 | 287568 | 70944 byte |
| candidate | 7040 | 290544 | 283504 | 70944 byte |

両方とも既存196608KiB基準を超える。wrapper無しの単一最大入力fresh A/B4組のdeltaは、baseline194336–196480KiB、candidate194288–196592KiBと重なり、candidateが高い組もある。原本寿命だけの再現性あるRSS改善は確認できず、以前の代理A/Bの36–42MiB差を製品効果とは扱わない。

候補の実最大入力11回では、初回close205568→11回目210544KiB、warm増分4976KiB。全closeのmalloc in-use69856byteと8counter0は維持。cold6976→final210544、delta203568KiBはcold196608KiB基準未達。元XLSX2/PPTX3・mixed HTML/PPTX・配布版の全受入をこれで代替しない。

## VM残差の実製品確認

同候補3入力を同じ親PIDでread-only vmmap計測: RSS6960→301248KiB。finalのPhysical footprint287.7MiB、`Malloc Large (empty)` virtual279.9MiB/resident・dirty269.6MiB/22regions。malloc側はin-use70944byte、allocated41943040byte。生存KDV ownerのリークと、解放済みVM residentを区別する。

同backendのJPEG復号・色変換・縮小で大型割当が発生することは独立hayro/sample診断でも確認済み。ただしそれだけでKDVの実行境界を除外しない。

## process隔離は診断prototypeのみ

同じ製品候補rlib・実worker・3原本・2倍率で、変換/描画を診断childで実行し、実RGBAを親へ返して保持/dropした。3childはexit0、各childのKDV8counterも0。6frameの寸法/RGBA byte数/FNV64とmetadataは同一。

親RSS6880→80288KiB（delta73408KiB）。親final `Malloc Large (empty)` resident69.3MiB。比較対象の同候補非隔離3入力はRSS6960→301248KiB（delta294288KiB）。これは隔離という機序を支持するが、製品コードではなくprototypeであり元5Office/配布版の合格とは報告しない。prototypeはOffice全sessionをchildへ移したのに対し、実装候補は既存`DocumentSession`のOffice rasterだけをsandboxed workerへ移すため、改めて製品A/Bが必要。

KatanAの公開KDV callerは`DocumentSession::open`。既存public API/型/エラーを変更せず、同高水準Office経路の描画だけをchildへ隔離し、親はPDF metadata・制限付きframe cacheを保持する方針。低水準`OfficeStaticViewerSession::render_item`と直接PDF APIは互換動作を維持する。worker失敗は既存`DocumentSessionError::Office`で伝え、in-process fallbackは追加しない。

## DocumentSession製品のraster隔離A/B

wrapper無しの同一DocumentSession診断binary source、同locked graph、実PPTX3入力と各入力の同scale bitsで比較した。旧workerは上記SHA、候補は新private modeを含むworkerへ変更する必要があるため、同一worker binaryとは記述しない。

- raster隔離candidate rlib SHA256: `7c5a9d49b2e0a56677ff909e313b7ada4073fa464ca1d355d825f98799666e73`
- raster隔離candidate worker SHA256: `203810e8735aba638207058c75c04998e9bd43f734de84d807da4d03fd9ebb13`

| DocumentSession | cold RSS KiB | final RSS KiB | delta KiB | final malloc in-use byte |
|---|---:|---:|---:|---:|
| baseline | 6912 | 298384 | 291472 | 70480 |
| raster-isolated candidate | 6976 | 157472 | 150496 | 70688 |

全6frameの寸法・RGBA byte数・非暗号学的FNV64・page count・diagnostics・capabilitiesが一致し、各closeで8counterは0。同じ親PIDのVM証跡もrawログへ保持した。候補はこの3PPTX subsetで既存cold196608KiBを下回ったが、元XLSX2/PPTX3、mixed HTML/PPTX、warm、公開版採用後のGUI受入は別途必要。

通常Cargoの`--test office_raster_worker_contract`は1pass/0fail。実DOCX/PPTXについて低水準直接描画と高水準worker描画のRGBA・寸法・fingerprint・metadataが一致し、倍率1/1.25/1のcache再利用で実worker完了4件、全close8counter0を確認した。構造分割後の全library/gateはまだ再検証中。

元の実XLSX2/PPTX3を同順序でDocumentSessionから開き、XLSXの実Grid frameと各PPTX first pageを取得した。viewport640×480で、PPTXは928px相当の同倍率、XLSXは既存APIどおりzoom commandなし。これはKDV製品経路の同入力A/Bであり、KatanA GUI/配布受入そのものではない。

| 5Office DocumentSession | cold RSS KiB | final RSS KiB | delta KiB | final malloc in-use byte |
|---|---:|---:|---:|---:|
| baseline | 6912 | 269056 | 262144 | 71600 |
| raster-isolated candidate | 6992 | 143248 | 136256 | 71808 |

同じ10行のmetadata/frame出力が一致し、XLSX2のGrid diagnostic digestとsheet count、PPTX3のRGBA/dimensions/page countが一致した。全close8counter0、両run exit0。候補最大PPTX11反復はcold6992→初回close100960→final107312KiB、cold増分100320KiB、warm増分6352KiBで、各close8counter0・malloc in-use69760byteを保持した。

構造分割後はfmt/AST PASS、library1950pass/0fail/1ignored、実worker契約1passと原本寿命2passを再確認した。既存cold196608/warm65536KiB基準は変更していない。新公開KRR0.4.22採用後のgraphと元下流GUIの受入は再検証が必要。

## 同一workerを用いた5Officeの追加対照

旧locked graphの固定baseline/candidate診断binary双方に、同じ`worker-isolation-candidate`（SHA256 `203810e8735aba638207058c75c04998e9bd43f734de84d807da4d03fd9ebb13`）を指定した。実原本5件・順序・倍率・viewport・上限を変えず、baselineの通常Office変換とcandidateの通常変換/private rasterを同workerで実行した。以前のworker差分による混同を避ける追加対照であり、最新registry graphやroot GUIの対照とは扱わない。

| 同一worker・5Office | cold RSS KiB | final RSS KiB | delta KiB | final malloc in-use byte |
|---|---:|---:|---:|---:|
| baseline | 6912 | 268224 | 261312 | 71584 |
| raster-isolated candidate | 6992 | 146544 | 139552 | 71744 |

双方exit0、metadata/frame10行のdiff exit0、全closeの8counter0、同親PIDのcold/final vmmapを取得した。baseline増分は元196608KiBを超え、candidateは下回る。mallocの生存量はほぼ同じであり、live ownerの恒久リークを解消したという主張ではない。新たなworker間差ではなく、KDV親raster実行境界の寄与を支持する。

- baseline診断binary SHA256: `d82060d655170cb8d6fb596eeb6f65aacd96d1a86d67c6a7e68c3234f6167491`
- candidate診断binary SHA256: `6d8e2ab6832ee23221e200852bfae813b764e57b64ae8b30ff03210cb316b794`
- raw: `unified-same-worker-baseline-five.log` / `unified-same-worker-candidate-five.log`

## 残DoD（更新）

### 公開依存候補での追加実測（KUC0.4.0時点）

KDV0.5.9/KRR0.4.22/KUC0.4.0/office2pdf0.8.0/V8152.2.0をregistryから解決した候補で、実5Officeはcold7008→final146528KiB、増分139520KiB、全close8counter0・最終malloc in-use71808byteだった。最大PPTX11反復はcold7008→初回close100672→final105248KiB、cold98240/warm4576KiB。既存cold196608/warm65536KiB基準は維持した。

同じ実frame行は旧隔離候補と一致した。ただしoffice2pdf0.8.0では1入力のCalibri→Yu Gothic fallback警告が消えたため、metadata/diagnostics全行が同一とは報告しない。

- このgraphのlock SHA256: `51b97d2f0ed5388ab719082627736ce4176b36215b919d2b70ffb50d8dbb6452`
- worker SHA256: `ad6e5cbe6728d2ffa81381435fa8d1fbf8f3ffd011828efc61a588f9801c86c6`
- rlib SHA256: `1722f8b286d3fbaafa95463a5dc1a7b16bb721934295eac4f7c1c5ccd9a12dcc`
- `unified-product-v059-office080-five.log` / `unified-product-v059-office080-warm11.log`に実入力と数値を保持。

既存の独立LibreOffice oracleと変更していないfixture/reference/許容値に対するOffice fidelityはPASS。DOCX2ページ、mean MAE0.02525295/RMSE0.10328595、XLSX2sheetのtext/font/fill/border/mergeのmissing/mismatch0、row track delta0.0037456584。semverは196pass/58skip。これらはKUC0.4.0の候補の証拠である。

この後、bounded file reader/JSON encodingの実異常系を補強し、公開KUC0.4.1をexact registryとして採用した。従って上記binary/hash/測定を最終HEAD、新しいKUC0.4.1 graph、GUI配布受入の証拠へ流用しない。再ビルドと再測定を残す。

### 最新reader/JSONと公開KUC0.4.1での製品再測定

KDV0.5.9/KRR0.4.22/KUCexact0.4.1/office2pdf0.8.0/V8152.2.0のregistry graphを再ビルドした。実5Officeはcold7056→final148304KiB、増分141248KiB、全close8counter0・最終malloc in-use71728byte。最大PPTX11反復はcold7024→初回close101184→final105520KiB、cold98496/warm4336KiB、最終malloc in-use69680byte。既存cold196608/warm65536KiB上限は維持した。

- lock SHA256: `3b28f987041f275a5e296930330e0971a27275a7d3ddcb5563ffe804d2d75ff8`
- worker SHA256: `05edb527f0b60533aaf1eeb7cbf0217a56df573b0f0d535e432f440181d2e6e6`
- rlib SHA256: `2d1f720106031d0d0c6d1195a2594d0ae01aeb8e208eb93a2b5a8900578014aa`
- raw: `unified-product-v059-kuc041-final-five.log` / `unified-product-v059-kuc041-final-warm11.log`

10行のmetadata/frameは直前のoffice2pdf0.8.0/KUC0.4.0候補とbyte一致した。旧0.7.0 baselineと最新graphの全metadata同一とは報告しない。これはKDV DocumentSessionの製品測定であり、root GUI、配布版、fresh registry consumerの受入ではない。

同じimmutable fixture/reference/閾値で最新graphのOffice fidelityを再実行しexit0。DOCX2ページのmean MAE0.02525295/RMSE0.10328595、XLSX2sheetのtext/font/fill/border/merge欠落・不一致0、row track delta0.0037456584を維持した。raw: `office-fidelity-v059-kuc041-final.json` / `.log`。通常`just check`、semver196pass/58skip、release-contract-check、V8 singleton152.2.0がexit0。厳格coverageはfunctions3739/3739・lines30683/30683で100%（`coverage-v059-kuc041-final-gate.log`）。worker実引数不正→exit64を含む実異常系5件を追加し、coverage閾値・除外は変更していない。完全release-check・公開・下流受入は未完了。

- [ ] sandbox/deadline/output/validation/cache/typed-errorを保った製品実装と回帰。
- [ ] 原本の異なる5Office、mixed HTML/PPTX、cold/warm、親子PID・全counter・実画素の製品A/Bと元受入。
- [ ] 全品質gate、依存再確認、Draft current-HEAD review、正規公開、fresh registry consumer、Issue終了、過去stashの意味的整理。

rawログは`issue59-rss.oBZVZI/`のproduct-*とfocused-*に保持し、私有入力を含む領域は公開成果物へ含めない。
