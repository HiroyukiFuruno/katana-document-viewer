## Context

公開 0.5.11 の external crop native test は diagrams と verifier expected-fixture が固定されている。Python 側は sample.md を既に受理できる。#58 の current Typography 入力を正しい render fixture と比較する入口が不足している。

## Goals / Non-Goals

Goals: 明示 fixture ごとの専用入口、取り違え拒否、実画素 decode と immutable 入力、95 点・既存 geometry/reference の維持。

Non-Goals: runtime/public API の変更、current でない入力の自動代入、非visual数値式の独断定義、元 Issue の未達受入の完了扱い。

## Decisions

既存 verifier helper に expected fixture を渡し、専用 sample/diagrams test が同じ値を provenance と描画に使用する。manifest から自動推定すると、利用者が意図した fixture の取り違えを見逃すため採用しない。既存 diagrams recipe は維持し、Typography recipe を追加する。

取り違え回帰は実 JSON/file と Python subprocess を通し、両方向の mismatch 拒否・sample 正常 PNG decode を検証する。既存 CRC/zlib/header-only 拒否も維持する。通常 test がこれらを実行し、実 current visual test だけ四つの外部 artifact 指定を要する ignored test とする。

KRR0.4.23公開後は実registry artifactとmerge/tag/VCS/checksumを独立照合してからcaret最低版を0.4.23へ更新する。release-contractは0.4.23以上の0.4.xを許可し、fresh consumerは現公開候補の実0.4.23/V8一版/全registryを検証する。旧最低版・旧checkerや旧graphのgate成功を転用しない。依存公開は元HTML/packaged性能の受入ではなく、元計測の所有境界と既存budgetを維持する。

作業証跡ハーネスが読むファイル一覧/検索行/sortにはRTK表示用圧縮を入れず、rtk proxyで生の完全パス・全証跡行・行番号を渡す。2change/archives除外/60行の実ファイル回帰を追加して漏れを検出する。ハーネスの証跡要件とnegative controlは維持する。

## Risks / Trade-offs

- [新 producer provenance 不足] → 実 binary/locks/fixture と生成宣言を確認し、crop 未提供や非visual未評価を残す。旧得点は転用しない。
- [共有 disk/CPU] → 重 build・性能計測の競合を確認し、元 raw/参照を保持する。旧 binary 成功を新 source へ流用しない。

## Migration Plan

単一 release/v0.5.12 で依存確認・完全 gate・Draft/current-HEAD review・通常 merge/自動公開・fresh exact registry consumer を行う。既存 diagrams 呼出しに互換性を保つ。

## Open Questions

#58 非visual算式・同 producer 独立 oracle/OS clipboard/性能入力は別判断待ちであり、今回の visual 入口追加の条件にはしない。#59 元HTML/packaged受入は別未完。
