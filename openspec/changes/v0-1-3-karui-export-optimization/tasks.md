# Tasks: v0.1.3 Karui export optimization evaluation

## Definition of Done

- [ ] Karui適用前後のPDF file size、生成時間、quality scoreを同じfixtureで比較できる
- [ ] Karui適用後PDFが `ExportQualityGate` を通過する
- [ ] link annotation、footnote destination、図形、数式、コードブロック、GitHub alertが劣化しないことを自動検査する
- [ ] 既定のHTML/PDF/PNG/JPG export挙動を変更しない
- [ ] KaruiをPDF生成backendとして使わないことが設計とテストで確認できる

## 1. 評価準備

- [ ] 1.1 Karuiのlibrary/CLI利用形態、license、binary配布、build影響を調査する
- [ ] 1.2 評価対象fixtureを `sample.ja.md` と `katana/README.md` 相当の英日混在Markdownから選ぶ
- [ ] 1.3 通常PDFのfile size、生成時間、quality scoreをbaselineとして記録する

## 2. postprocess adapter

- [ ] 2.1 Karui呼び出しをKDV PDF生成後のpostprocess adapterとして切る
- [ ] 2.2 Karui失敗時に通常PDFを壊さず、diagnosticsへ失敗理由を残す
- [ ] 2.3 既定ではKarui postprocessを無効にする

## 3. 品質ゲート

- [ ] 3.1 Karui適用後PDFに `ExportQualityGate` を再実行する
- [ ] 3.2 link annotationとfootnote destinationが残ることを検査する
- [ ] 3.3 図形、数式、コードブロック、GitHub alert、tableが劣化しないことを検査する
- [ ] 3.4 file size削減率と処理時間をscore reportへ含める

## 4. 採用判定

- [ ] 4.1 fatal failureが増える場合は不採用にする
- [ ] 4.2 file size削減が小さい、または処理時間が大きい場合は不採用にする
- [ ] 4.3 採用する場合は明示オプションとしてCLI/API設計を別changeへ切り出す
