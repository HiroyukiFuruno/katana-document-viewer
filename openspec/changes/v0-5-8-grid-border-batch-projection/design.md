## Context

`DocumentSurfaceFrame` はgrid cellごとの四辺罫線を内部Vecに保持する。既存の
`grid_cell_borders(coordinate)` は単発照会に適する一方、hostが可視セル全体を描画
する場合は各座標で同じVecを再探索する。KatanAは一度のprojectionを描画キャッシュへ
渡す責務を持ち、KDVはhost固有の描画またはcacheを持ち込まない。

## Goals / Non-Goals

**Goals:**

- 座標と四辺罫線の格納順を、allocation・再計算・個別探索なしに一括走査可能にする。
- 既存の単発照会API、罫線style/color/`none`表現、merged-cell座標を維持する。
- 実XLSX由来の罫線と大きいprojectionで、batch結果が単発APIと同じ完全な値を返す
  回帰を固定する。

**Non-Goals:**

- KatanAの描画、viewport cache、reference PNG、fidelity閾値の変更。
- `HashMap`等の重複indexをKDV frameへ永続保持すること。
- single-cell lookupの計算量または既存public APIの削除。

## Decisions

### 読み取り専用sliceを公開する

`DocumentSurfaceFrame::grid_cell_border_entries` は
`&[(DocumentGridCoordinate, DocumentGridCellBorders)]` を返す。内部Vecへのborrowであるため、
hostは一度の反復で座標と四辺罫線を投影でき、コピー・再計算・検索を発生させない。

`HashMap` indexをframeへ追加する案は、lookupを高速化するが、すべてのframeで追加の
allocation/メモリを負担し、順序も変える。本件は全件projectionが必要なので採用しない。

### 単発APIを維持する

`grid_cell_borders` は互換性のため残す。batch APIは単発lookupの代替ではなく、全件描画の
入口である。両APIの値が同じであることをテストする。

### 罫線を欠落から除外しない

batch accessorはgridが持つすべてのcell entryを返す。styleが`none`または四辺が欠損した
entryも除外しないため、hostは既存の投影契約どおり可視/非表示を判断できる。

## Risks / Trade-offs

- [sliceが内部順序をobservableにする] → 順序はgrid input順として仕様化し、hostは座標を
  keyにして必要なら自らindex化する。
- [hostがsliceをframe寿命より長く保持する] → Rust borrowによりコンパイル時に禁止する。
- [大きいgridの回帰が見逃される] → 複数style・欠損・merged座標を含む入力でentry数と
  内容を固定し、個別lookupの繰返しを必要としないconsumer形をテストする。
