# 外部crop/full PNGの実デコード回帰

後続strict gate80241は新test factoryのtuple戻り値へtype_complexityを出してexit101。名前付きCurrentPngFixtureがTempDirと四pathを所有する形に整理し、strict lint96111 exit0。実decoder/拒否対象/scoreは非変更。下記ae5cd096…と12580/08f641…は型整理前の実epochとして保持し、新source全gate62926と現在binaryの回帰を別証拠として記録する。

既存Python入口はSHA/PNG IHDR/寸法/frame/geometry/clean stateのprovenanceを検証し、画像デコードをRust scorerへ委ねていた。しかしRust側ではcropだけが後段でdecodeされ、fullは実画素検査に到達していなかった。

公開synthetic PNG/geometry/manifestの既存factoryをnative回帰へ共用し、破損PNGの宣言SHAも更新した。修正前のnative入口は正常1PASS/破損full3FAILで誤受理を再現（session75132 exit101）。既存imageのPNG decoderで両入力をRGBAまでdecodeする最小修正後、正常1件と拒否3件を合わせた4native testがPASS（22157/12580 exit0）。各拒否testはfull/crop双方を試すため、header-only/IDAT CRC/zlib破損の6ケースを実際に検証している。12580ではエラーがPNG pixel decode由来であることもassertし、単なるhash/宣言不一致による偽の成功を防いだ。全4入力のbytes非変更も検証している。

検証: `rtk proxy env CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTFLAGS='-D warnings' cargo test -p kdv-storybook --locked current_crop_provenance_ -- --nocapture --test-threads=1`。

Pythonの既存27self-testは `rtk proxy just current-preview-crop-verifier-test` でPASS。AST台帳の証跡ラベル修正後、`rtk proxy env JOBS=1 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 just ast-lint` は1PASS。OpenSpec strict validateとdiff checkもPASS。ただし完全release-checkは新sourceで未終端のため全quality成功とはしない。

| 新source | SHA256 |
| --- | --- |
| native scorer/回帰 | ae5cd096c3cc908ef3e5c73958ad858eb0147852cec84b13e119f0bc6d0cde9b |
| Python公開fixture factory | 810affcfce5bd055cba715eee0523b6633e7fe2e12018f2865dade28cd89a3e6 |
| 現lock | 90d89a556a81d8523d9c63d642f6b1661ff366c22935f76eb1f93eac329e33d1 |

既存PNG codec/標準decoder制限を使い、新しいpip依存・独自PNG codec・threshold/reference/geometry変更は導入しない。Storybookのtest-only tempfile登録以外に解決package/versionは変わっていない。現lockのStorybook所有tempfile行だけを除いたreadonly再計算は旧2171a915…に一致する。

PNG実体の完全性は四分類の採点結果ではない。新current producerの独立semantic/interaction/OSclipboard/performance証拠・採点方式・元packaged受入が不足したままIssue58をCloseしない。旧referenceと旧診断/失敗ログを保持する。
