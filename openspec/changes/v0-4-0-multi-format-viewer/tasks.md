# Tasks: katana-document-viewer v0.4.0 multi-format viewer

## Definition of Ready (DoR)

- [ ] `v0.2.0` のviewer input / command / KUC bridgeが利用可能である
- [ ] `v0.3.0` のpagination previewとPDF export境界が完了している
- [ ] KRRのpublic APIで扱えるrender-runtime責務を確認できる
- [ ] KUCのtable、page viewport、image viewportの利用条件が確認できる

## Definition of Done (DoD)

- [ ] `rtk ./scripts/openspec validate v0-4-0-multi-format-viewer --strict --no-interactive` が通る
- [ ] `rtk just check` が通る
- [ ] PDF / CSV / Office / image系の責務判断表がdocsまたはdesignへ残っている
- [ ] 責務判断表には目標品質、入力契約、出力契約、採用owner、却下ownerと理由、KRR handoff要否が含まれている
- [ ] owner未確定のformatは実装対象から外し、未対応diagnosticsへ落としている
- [ ] KRR責務と判断した処理はKRR public APIまたは別changeへのhandoffとして表現されている
- [ ] KDV内の暫定rendererを正規経路にしていない

---

## 1. Boundary decision を確定する

- [ ] 1.1 KRR public APIで扱えるrender-runtime入力と出力を棚卸しする
- [ ] 1.2 PDF / CSV / DOCX / XLSX / PPTX / SVG / WebP / AVIFごとに目標品質を可読性優先 / layout faithful / pixel faithfulから選ぶ
- [ ] 1.3 formatごとに入力契約、出力契約、parse/extract owner、render owner、viewer owner、command ownerを分類する
- [ ] 1.4 採用ownerだけでなく、却下したownerと理由を記録する
- [ ] 1.5 owner未確定のformatを実装対象から外す
- [ ] 1.6 KRR側拡張が必要な処理はKDV実装に混ぜず、別changeまたはhandoffとして記録する
- [ ] 1.7 ユーザーへ境界判断を提示し、承認後にformat別実装へ進む

---

## 2. neutral ViewerSource contract を追加する

- [ ] 2.1 `ViewerSource::Pdf`、`ViewerSource::Csv`、`ViewerSource::Office`、`ViewerSource::Image` を追加する
- [ ] 2.2 source identity、revision、MIME、diagnosticsを持つ
- [ ] 2.3 KUC型やKRR内部型をpublic APIへ露出しない
- [ ] 2.4 unsupported formatはraw sourceとdiagnosticsを保持する

---

## 3. PDF viewer を追加する

- [ ] 3.1 PDF page renderingをKRRへ置くかKDV adapterへ置くかを境界判断に沿って決める
- [ ] 3.2 KDVはpage navigation stateとhost commandを持つ
- [ ] 3.3 KUCはpage viewportとnavigation controlsを表示する
- [ ] 3.4 text layer selectionは将来対応としてdiagnosticsまたはcapabilityへ明記する

---

## 4. CSV viewer を追加する

- [ ] 4.1 CSV parserとtable modelをKDV側に追加する
- [ ] 4.2 header detection、column width hint、cell diagnosticsをmodelへ入れる
- [ ] 4.3 KUC tableへ表示する
- [ ] 4.4 CSVをKRRへ渡す前提にしていないことをテストする

---

## 5. Office viewer を追加する

- [ ] 5.1 DOCX / XLSX / PPTXを可読性優先の抽出modelにするか、layout-faithful renderingをKRR候補にするかを決める
- [ ] 5.2 可読性優先の場合はテキスト、表、画像の抽出modelをKDVで定義する
- [ ] 5.3 layout-faithful renderingが必要な場合はKRR側handoffを作り、KDVでは未対応diagnosticsを保持する
- [ ] 5.4 KUC viewerは抽出済みmodelまたはrendered artifactだけを表示する

---

## 6. Image viewer を追加する

- [ ] 6.1 SVG / WebP / AVIF decodeまたはrasterizationをKRRへ置くかKDV adapterへ置くかを境界判断に沿って決める
- [ ] 6.2 KDVはzoom、fit、copy、open commandを持つ
- [ ] 6.3 KUCはimage viewportを表示する
- [ ] 6.4 decode不能時はraw sourceとdiagnosticsを保持する

---

## 7. Final Verification

- [ ] 7.1 `rtk just check` を実行する
- [ ] 7.2 `rtk ./scripts/openspec validate v0-4-0-multi-format-viewer --strict --no-interactive` を実行する
- [ ] 7.3 境界判断表、KRR handoff、KDV実装範囲をユーザーへ報告する
