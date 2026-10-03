## Context

公開KDV0.5.10/office2pdf0.8.0の実worker PDFで、原本26ptの中央寄せ折返し見出しがCTM0.6/effective15.6ptだった。owner PR1965はautofit guardと中央overflow配置を修正し、native PowerPointと公開synthetic fixtureのcompiled PDF回帰を追加した。0.8.1をregistryから実取得済み。KRR修正版は別の公開待ちである。

## Goals / Non-Goals

**Goals:** exact registry0.8.1を採用し、KDV公開worker経由で文字サイズ/アンカー/明示autofitが維持されることを検証する。元原本の再検証と全品質ゲート、公開後consumerまで0.5.11として完了する。

外部PNGでは既存PythonのSHA/寸法/geometry/frame契約を維持し、native scorerのprovenance段階でcropとfull両方の実画素decodeを要求する。再hashされた破損画像も画像codecで拒否する。

**Non-Goals:** KDV側のgeometry補償、原本/独立referenceの上書き、上限/閾値の緩和、未公開依存、非visual採点式の無承認実装。Office局所改善を元HTML正常closeや全配布受入の代替にしない。

## Decisions

- ownerの公開修正を採用し、KDVにconverterと重複する縮小補償は加えない。private原本はignored診断に保持し、tracked回帰は公開synthetic入力を使用する。
- PDFのTfだけでなく合成CTM/text matrixを検査する。描画スクリーンショットだけでサイズを合格にしない。
- baseline0.8.0/new0.8.1の同入力・既存memory2GiB/CPU46秒/wall45秒/output128MiB、font設定、元PDF/参照SHAを記録し、異なるproducer epochへ成功を転用しない。
- 直接/推移/lockfileを調査し、最新互換更新と必要なmajor移行を評価する。libc0.2.190/minifb0.29の候補もcontract/API影響を調べ、満たせない更新は理由と残存版を記録する。
- HTMLは公開KRR修正版後rootが元正常closeを再受入する。Office局所作業をその待ちだけで止めない。
- PNG完全性は既存imageのPNG decoderへ委ね、Pythonに独自の画像codecや追加pip依存を作らない。両PNGの全画素decodeをnative入口へ追加し、score計算前に不正入力を拒否する。既存Python公開synthetic PNG/geometry/manifest factoryをnative回帰から使い、hash不一致だけの拒否を誤って成功としない。点数式・既存reference・95閾値は不変。

## Risks / Trade-offs

[上流の他のfont/layout変更による原本差] → 新graphで全fidelity/score/既存reference/意味保持を検証し、不合格を緩和しない。

[全lock変更と同binaryの誤結合] → source/binary/両lock/原本/frameを生成epoch別に結合する。旧manifest/BINDINGSを新lockで上書きしない。

[共有host容量/性能競合] → 起動直前の空きと実processを確認し、凍結証跡を残して再生成可能な自repo成果物だけを安全に整理する。他taskを終了/編集しない。

## Migration Plan

自然次版0.5.11/単一release/v0.5.11で公開依存と回帰を統合し、全gate→Draft current-HEAD review/全thread対応→Ready/final required→通常merge/自動公開→fresh registry worker/consumer→局所cleanup。未合格の候補は公開しない。

## Open Questions

原本別承認tolerance、非visual採点規約、current packaged/clean-machineの真受入は旧Issue58/59の未完条件であり、このchangeで変更しない。
