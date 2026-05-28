# Tasks: v0.1.3 Karui export optimization evaluation

## Definition of Done

- [x] Karui適用前後のPDF file size、生成時間、quality scoreを同じfixtureで比較できる
- [x] Karui適用後PDFが `ExportQualityGate` を通過する
- [x] link annotation、footnote destination、図形、数式、コードブロック、GitHub alertが劣化しないことを自動検査する
- [x] 既定のHTML/PDF/PNG/JPG export挙動を変更しない
- [x] KaruiをPDF生成backendとして使わないことが設計とテストで確認できる
- [x] 実際に生成したPDF artifactをユーザーが目視確認し、releaseへ進めてよいことを確認済みである
- [x] PDF artifact の目視確認で見つかった表示劣化が解消済み、またはKarui不採用としてrelease判断から除外済みである

## 1. 評価準備

- [x] 1.1 Karuiのlibrary/CLI利用形態、license、binary配布、build影響を調査する
  - `docs/karui-postprocess-evaluation.md` に記録した。現時点ではKDVが安定して呼べるpublic crate / CLI / library APIは未確認のため、adapterは未提供診断を返す。
- [x] 1.2 評価対象fixtureを `sample.ja.md` と `katana/README.md` 相当の英日混在Markdownから選ぶ
  - 評価fixture方針を `docs/karui-postprocess-evaluation.md` に固定した。
- [x] 1.3 通常PDFのfile size、生成時間、quality scoreをbaselineとして記録する
  - `ExportPostprocessEvaluationRequest` / `ExportPostprocessMetrics` でbaseline PDF size、baseline生成時間、通常/最適化後quality reportを同じreportに保持する。

## 2. postprocess adapter

- [x] 2.1 Karui呼び出しをKDV PDF生成後のpostprocess adapterとして切る
- [x] 2.2 Karui失敗時に通常PDFを壊さず、diagnosticsへ失敗理由を残す
- [x] 2.3 既定ではKarui postprocessを無効にする

## 3. 品質ゲート

- [x] 3.1 Karui適用後PDFに `ExportQualityGate` を再実行する
- [x] 3.2 link annotationとfootnote destinationが残ることを検査する
- [x] 3.3 図形、数式、コードブロック、GitHub alert、tableが劣化しないことを検査する
  - PDF page image を失わないことと既存surface/PDF品質テストを条件にし、native surface rasterの劣化を採用不可にした。
- [x] 3.4 file size削減率と処理時間をscore reportへ含める
- [/] 3.5 実際のPDF artifactを出力し、絶対パスをユーザーへ提示して目視確認を依頼する
  - failed artifact: `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tmp/kdv-export-debug/v0-1-3-user-pdf-review-20260527-112528/exports/sample.ja.pdf`
  - fixed candidate artifact: `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tmp/kdv-export-debug/v0-1-3-user-pdf-review-20260527-114730/exports/sample.ja.pdf`
  - dependency-updated artifact: `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tmp/kdv-export-debug/v0-1-3-release-pr-20260527-132739/exports/sample.ja.pdf`

## 4. 採用判定

- [x] 4.1 fatal failureが増える場合は不採用にする
- [x] 4.2 file size削減が小さい、または処理時間が大きい場合は不採用にする
- [x] 4.3 採用する場合は明示オプションとしてCLI/API設計を別changeへ切り出す
  - v0.1.3ではKaruiを採用しないため、CLI/APIの明示オプションは作らない。

## User Feedback

- [/] PDF容量削減は機械検査だけでrelease判定しない。実際に生成したPDF artifactを提示して目視確認を依頼する。
- [/] ユーザーが `approve` を返したため、release PR作成準備を再開する。
- [/] 2026-05-27の目視確認で、READMEヘッダー再現箇所のHTML imageが画像ではなく文字列としてPDFに出力され、コードブロック周辺の見た目にも劣化疑いが出た。Karui適用結果なら調整または採用見送り、通常PDF export結果ならrelease blockerとして原因を切り分けてから再提示する。
  - root cause: KDV native PDF surfaceで、HTMLタグ解析がquoted attribute内の `>` をタグ終端として扱っていたため、fixtureのSVG data URIが画像ではなく文字列化された。
  - root cause: code block描画で、本文のX座標へ `box_x` ではなく `box_y` を渡していたため、縦位置に応じてコード本文が右へずれた。
  - fixed candidate: `readme-header-crop.png` と `code-block-crop-2.png` で当該崩れの解消を確認した。
- [/] 同じ壊れ方を `just check` で検知できるよう、HTML data URI画像、quoted attribute内 `>`、sample fixtureのraw HTML漏れ、code block本文X座標の回帰テストを追加する。
- [/] 既存の品質ゲートは、PDF artifactの見た目を保証するoracleとして浅すぎた。`just check` / `release-check` を必要条件に下げ、成果物変更では実artifact出力、絶対パス提示、ユーザー目視確認、再発可能な劣化の回帰テスト化を release gate として扱う。
- [/] 修正版PDF artifactをユーザーが目視確認し、releaseへ進めてよいことを確認する。
