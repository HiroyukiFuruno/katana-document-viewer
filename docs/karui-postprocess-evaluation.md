# Karui postprocess 評価メモ

## 結論

v0.1.3 では Karui を PDF 生成 backend として採用しない。
KDV native export で生成した PDF に対して、明示的に選ばれた後段処理だけを評価する。

Karui は製品ページ上で PDF 圧縮を提供しており、Smart mode は PDF 内画像の再圧縮、Render mode はページ再描画として説明されている。
一方、KDV が安定して呼べる public crate / CLI / library API は今回確認した範囲では見つからない。
Zenn 記事では Karui PDF エンジンを要望次第で MIT 切り出し可能と説明しているため、現時点では実験 adapter を「未提供」として扱う。

- Karui: https://karui.app/pdf-compress/
- Zenn: https://zenn.dev/ikora/articles/b50ca6275eddc9

## KDV 側の境界

- HTML/PDF/PNG/JPG の既定 export は変更しない。
- PDF 生成、pagination、link annotation、surface 描画は KDV native export の責務として維持する。
- Karui は生成済み PDF の postprocess 候補としてだけ扱う。
- Karui が未導入または失敗した場合、通常 PDF bytes を返し、diagnostics に理由を残す。

## v0.1.3 採用条件

候補 PDF は次をすべて満たす場合だけ採用可能にする。

- `ExportQualityGate` を通過する。
- page tree、page image、link annotation、footnote destination の個数が通常 PDF より減らない。
- file size が 5.00% 以上削減される。
- postprocess 時間が 30,000ms を超えない。

図形、数式、コードブロック、GitHub alert、table は KDV native surface の raster image として PDF に入る。
そのため v0.1.3 の自動検査では、最適化後 PDF が page image を失っていないことと、既存の surface/PDF 品質テストが通ることを採用条件に含める。

## 評価 fixture

比較対象は同じ Markdown から生成した通常 export と postprocess 後 export に限定する。

- `sample.ja.md` 相当の日本語 fixture
- `katana/README.md` 相当の英日混在 Markdown

Karui の安定した呼び出し口が公開されるまでは、KDV 側では `KaruiPdfPostprocessAdapter` が未提供診断を返し、実測 benchmark は採用判定に使わない。
