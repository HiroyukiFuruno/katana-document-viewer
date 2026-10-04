# 0.5.11 公開依存の採用候補

## 実取得

office2pdf0.8.1をcargo info --registry crates-ioで取得exit0。実crate SHA256 b816a251df46819b282610723877ec6c5fa9a97a953c2068f83367e7d804d512、registry VCS9ca03fecab22a6b4828736b003ee1c5f9551e0dcはGitHub Release target一致。最初のnot foundは公開処理中の観測として保持する。

PR1965のauto_fit guard/center overflow/compiled PDF回帰をmainが直接source差分で確認。upstreamのnative比較・回帰成功はowner証跡であり、元PPTXの新KDV worker受入はまだ実施していない。

後続で元11page新worker実変換と同入力title26pt回復を直接確認した。公開合成回帰3件は候補devでPASS、同実公開0.5.10 workerでは暗黙autofit回帰1件がFAILした。全原本/配布受入の成功には読み替えない。

## 直接・推移依存監査

manifest版0.5.11/office2pdf exact0.8.1。cargo upgrade --dry-run --incompatible allow --pinned allow --recursive falseで候補libc0.2.190/minifb0.29/office2pdf0.8.1を検出。全workspace3crateの監査を行い、候補採用後の同dry-runは追加変更なしexit0（latest36 packages）。

just update-safeでTokio1.53.1→1.53.2とminifb0.29のWayland依存更新を反映。libc exact0.2.190に合わせてrelease contractと自己検査fixtureの固定値を更新した。seccompiler/skarn-sandboxやsandbox上限を変更していない。

minifb0.29のsource/CHANGELOGを読み、WindowOptionsの追加fieldとMouseButton追加variantを確認。KDVはdefault struct updateを使い、MouseButtonの網羅matchはない。公開methodの削除なし。Wayland GPU自動有効化を従来動作へ混ぜないためStorybookはUseGPU::Disabledを明示する。macOS/Windowsはこのfieldを読まない。3OSのcompile/実表示/品質gateでの採否は未完了。

Cargo.lock SHA2560665298bd67e04d82e5e944f97543e1a40948d4ac25afe96321064a8856190ef。KRRはregistry0.4.22、KUC0.4.1、office2pdf0.8.1、V8 singleton152.2.0。cargo tree -dの重複一覧にV8なし、inverse treeはKDV/KRRが同一152.2.0を使用する。これはfresh linkの代替ではない。

後続のテスト専用pdf-extract0.12.1（調査時公開最新）追加後、現lockは2171a915db6201f8c1466c03a6a00bfa3c9b7c55bd258a7409d2c73531c861cf。PDF全文と合成glyph変換を独立に読む目的でdev-dependenciesのみに追加し、通常の公開runtime graphには追加しない。13推移依存をlockへ記録、全workspace dry-run追加変更なし（latest37 packages）。旧原本診断epochのlockを改ざんしない。各候補のstrict full gatesを開始中で、まだ採用完了ではない。

## 完了・未完の区別

2026-10-04追補: PNG実デコード回帰の一時fixture所有に既存workspace tempfileをStorybook dev-dependenciesへ登録した。cargo update --offline -p kdv-storybookはLocking0packages/exit0。現lock90d89a556a81d8523d9c63d642f6b1661ff366c22935f76eb1f93eac329e33d1から当該owner登録一行だけをreadonly投影で除いたSHAは2171a915…と一致し、選択package/versionは変化していない。旧worker/元46slide診断のlock epochを現在値へ改ざんしない。

最初の完全gate68397はsemver196件/strict Clippyまで成功後、tasks.mdの証跡label不足をASTで拒否しexit101。label修正後のASTはexit0。現sourceの全workspace test/coverage/score/package/publish dry-runを含む完全gateは再実行待ちで、旧部分成功を全採用へ読み替えない。

- just fmt-check exit0、git diff --check exit0。
- strict OpenSpec validate exit0、release-target-check0.5.11 exit0（公開line0.5.10の次版）。最初の旧manifest0.5.10での版不一致は変更前の診断で、成功へ読み替えない。
- release contract Python self-test/新graph validation exit0。
- release-contract-check全入口はbrowser_session実buildに進んだため、KRR実測競合を避けてsession51461をSIGINT/exit130。静的部成功だけで全入口PASSにしない。
- V8 checkerは実link testを含むため、同様にsession85747をSIGINT/exit130。tree/self-testは成功だが実link未完。
- 更新中の旧lockに対するlibc contract不一致も保存し、cargo update完了後に同静的contractを再検査してexit0。
- 自repoのcoverage専用targetだけcargo cleanで12.9GiB再生成可能cacheを整理。原本/参照/raw/固定binaryを保持し、旧coverage成功値を新graphへ転用しない。

新worker回帰・元PPTX・strict Clippy/全test/coverage/score/semver/release-check・commit/push/Draft/current-HEAD review/公開は未完。KRR実測中に重いbuild/変換/性能を重ねない。新source/全lock/workerが結び付く元受入が必要で、旧binaryを新graphと宣言しない。
