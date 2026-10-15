# KDV局所回帰 2026-10-06

## 確認済み

- strict Clippy `just lint`: PASS（全workspace/all-targets/all-features）。
- AST `just ast-lint`: PASS（追加した計測/永続cache/testを200行/30行制約内に分割）。
- subagent harness標準suiteと実契約: PASS。実際の委譲モデルは利用可能なgpt-5.6-luna/mediumで、主担当が結果を検証し、推測のZIP修正を棄却した。
- 統合paged session通常回帰: 5 PASS、私有入力指定の3件は通常実行でignored。
- 合法PPTX descriptor: direct office2pdf Stored、KDV worker Stored/Deflatedが3 PASS。通常とDeflatedのRGBA一致。streaming ZIPを一律に拒否する仮説は実測で棄却。
- PDF計測: 実child processでdecode/render/rasterize/image_encode/frame_decodeを確認、1 PASS。
- 永続cache: 容量/破損/clear/内容・scale・environment・limits変更時miss、独立9process（PDF/DOCX/PPTXそれぞれcold/restart/revision変更）を含む2 PASS、child入口はparentから実行。restartのconversion/raster再実行がないことをtraceで確認。

## 計測範囲

匿名fixture、scale 0.5、source取得から最初のRGBA page artifactまで。native画面表示やOS clipboardを含まない。coldは新しい独立process/cache miss、restartは別process/cache hit、warmは同じprocessでdrop/reopen。

| 入力 | cold取得〜frame | 別process restart hit | 同process warm reopen |
|---|---:|---:|---:|
| PDF | 0.074秒 | 0.053秒 | 0.053秒 |
| DOCX | 1.341秒 | 0.096秒 | 0.099秒 |
| PPTX | 1.305秒 | 0.104秒 | 0.099秒 |

この入力では暫定約2秒以内を維持し、restartを1秒未満へ改善。DOCX/PPTX coldは理想1秒未満に未達。任意入力や実アプリの初回表示を保証する結果ではない。PDF geometry/outlineのdecodeはcache hit時も実行する。

## 未検証

新しい最終source/graphのfull test/coverage100%/score/semver/strict release-check、KRR修正版の公開取り込み、Draft review/required CI/公開/fresh consumerは未完。原本PPTX3件が現在見つかったため、別のlocal-only corpus測定を実行中。報告エラーと合法fixtureの成功を混同しない。

## 私有DOCX/PDF corpus（原本は公開しない）

同じ計測入口へ環境変数で原本を指定し、PDF 4件・DOCX 2件で各cold/独立process restartと各process内warm reopenを実行した。全12processが成功し、restart時のconversion/raster省略と終了後のresource counterゼロを確認した。raw logと内容SHA256はignored tmpにだけ保存する。

| 種別 | cold取得〜frame | restart hit | warm reopen |
|---|---:|---:|---:|
| PDF 4件 | 0.034–0.061秒 | 0.012–0.015秒 | 0.012–0.014秒 |
| DOCX 2件 | 1.567–3.753秒 | 0.153–0.160秒 | 0.152–0.155秒 |

最も遅いDOCXではoffice.conversionが2.913秒、最初のrasterが0.029秒。coldは暫定約2秒にも未達の入力を含むため、全入力での性能合格とは扱わない。測定はscale 0.5のKDV RGBA artifactまでで、実画面の表示時間は含まない。入力名・内容・画像・私有絶対pathは公開artifactへ記載しない。
