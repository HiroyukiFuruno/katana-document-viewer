## ADDED Requirements

### Requirement: 公開converterのautofit契約

KDV SHALL exact registry office2pdf0.8.1を採用し、中央寄せで折り返す文字列をautofit未指定だけで縮小させない。検証 SHALL PDF合成変換に基づく文字サイズ、行数、中央配置と完全な文字列を確認する。

#### Scenario: autofitなしの中央折返し
- **WHEN** 公開synthetic26pt見出しをKDVの実Office workerへ渡す
- **THEN** 宣言サイズと中央アンカーが維持され、CTMによる隠れた縮小を回帰が検出する

#### Scenario: 明示autofitの維持
- **WHEN** 同じ折返し構造でnormAutofitを要求する
- **THEN** 要求された縮小を保持し、autofitなしだけの修正で既存挙動を破壊しない

### Requirement: 元入力と公開品質の継続

新graphの検証 MUST 原本/参照/SHA/既存資源制限/既存品質条件を保持し、元入力の受入と局所診断を区別する。HTML正常close、全配布受入、独立非visual評価の未完をこのchangeの公開成功から推定してはならない。

#### Scenario: 新依存の元入力再検証
- **WHEN** registry更新後のworkerで元原本を変換する
- **THEN** 当該graph/workerの実PDF変換を確認し、上流の回帰成功や旧worker結果を新実測として転用しない

#### Scenario: 全公開工程
- **WHEN** 0.5.11を公開する
- **THEN** 全build/test/lint/coverage/score/semver/release-check、Draft review/指摘reply resolve/Ready/required、通常merge、自動公開、fresh registry consumerと局所cleanupを完了する
