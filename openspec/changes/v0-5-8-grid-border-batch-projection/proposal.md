## Why

`DocumentSurfaceFrame::grid_cell_borders` は座標ごとに格納Vecを線形探索する。
KatanA が可視セルごとの四辺罫線を描画すると探索が二乗化するため、公開KDV
`0.5.7`を採用した実受入でhost側罫線描画を安全に追加できない。

## What Changes

- 既存の単一セル `grid_cell_borders` を維持したまま、座標と四辺罫線を一度だけ
  走査できる読み取り専用batch accessorを公開する。
- batch accessor が格納済みの罫線を複製・再計算せず、スタイル、色、`none`、
  欠損、merged-cell座標をそのまま返すことを契約化する。
- 大きいgridとviewport投影を対象に、利用側がセル数と罫線数の和に比例して
  罫線を取得できる回帰を追加する。

## Capabilities

### New Capabilities

- `grid-border-batch-projection`: DocumentSurfaceFrameからセル座標と四辺罫線を
  線形時間で投影する公開読み取りAPI。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-document-viewer` の公開 `DocumentSurfaceFrame` APIと契約テスト。
- KatanAは新APIを一回投影して描画キャッシュを構築できるが、本変更ではKatanAの
  描画実装、fixture、reference、品質閾値を変更しない。
- 依存追加は行わない。
