# Current KatanA crop の visual 評価

既存 reference を上書きせず、同じ current producer に由来する四つの外部ファイルを明示する。入力は評価中に変更しない。

- `KDV_CURRENT_CROP_MANIFEST`: producer/fixture/依存版/SHA/寸法/frame/geometry 宣言。
- `KDV_CURRENT_CROP_PNG`: 固定 geometry から用意した 1280x2400 crop。
- `KDV_CURRENT_CROP_FULL_PNG`: 同じ capture の 2565x4774 full PNG。
- `KDV_CURRENT_CROP_GEOMETRY`: 同じ completed frame の geometry/clean interaction JSON。

上記環境変数を設定後、Typography (manifest fixture `katana/sample.md`) は以下で評価する。

```bash
rtk proxy just JOBS=1 current-preview-crop-sample-score-check
```

Diagrams (manifest fixture `katana/sample_diagrams.md`) は従来の入口を使う。

```bash
rtk proxy just JOBS=1 current-preview-crop-score-check
```

専用入口は宣言 fixture と描画 fixture の一致を必須化し、両 PNG を実画素まで decode して既存 visual 95 点で判定する。入力不足・取り違え・hash/geometry 不一致・壊れた PNG を拒否する。score は指定 crop に対する KDV 描画との比較であり、別 producer の旧成功を新候補へ転用しない。

provenance 検証は宣言と実ファイルの整合検査であり、生成時 source/binary/locks の独立証明を自動で完了させるものではない。生成者の raw/immutable identity と採用判断を別途追跡する。visual 成功を semantic/interaction/OS clipboard/performance の得点へ転用しない。四分類の不足が残る Issue #58 を、この入口追加・visual 成功だけで Close しない。

fixture 結合・入力非変更・PNG 破損拒否の回帰は通常 native test、宣言/CLI 境界は `rtk proxy just current-preview-crop-verifier-test` で検査する。実 current 採点入口は環境依存のため ignored test として明示実行する。
