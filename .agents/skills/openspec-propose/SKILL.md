---
name: openspec-propose
description: katana-document-preview の OpenSpec 変更案を、提案、設計、仕様差分、タスクまで一括で作る。仕様が曖昧な新機能、API 変更、文書プレビューの責務変更を始めるときに使う。
---

# OpenSpec Propose

kdp の変更を実装前に言語化し、後続の実装者が迷わない OpenSpec artifacts を作ります。
生成する文書は原則として日本語で書きます。

## 実行入口

リポジトリに `Justfile`、`Makefile`、`scripts/openspec` がある場合は、その入口を優先します。
現時点で専用入口がない場合は、リポジトリルートから次を使います。

```bash
npx -y @fission-ai/openspec <command>
```

## 入力

ユーザーの依頼から、変更名と目的を決めます。
変更名がない場合は、内容から kebab-case の change id を提案します。

例:

- `v0-1-0-document-preview-interface`
- `v0-2-0-multi-format-preview`

判断できない場合は、次の一点だけ質問します。

```text
今回固定したい変更は、公開 API、neutral interface、preview backend、egui implementation のどれに関するものですか？
```

## 手順

1. 既存の active change を確認する。

   ```bash
   npx -y @fission-ai/openspec list --json
   ```

2. 同じ責務の active change がないか確認する。
   近い変更がある場合は、新規作成ではなく継続するかをユーザーに確認する。

3. change を作る。

   ```bash
   npx -y @fission-ai/openspec new change "<change-id>"
   ```

4. artifact の状態を確認する。

   ```bash
   npx -y @fission-ai/openspec status --change "<change-id>" --json
   ```

5. `instructions` の順序に従って artifact を作る。

   ```bash
   npx -y @fission-ai/openspec instructions <artifact-id> --change "<change-id>" --json
   ```

6. 次の観点を必ず入れる。

   - 公開 API と crate 境界
   - neutral interface と egui implementation の責務
   - 中立 interface と egui implementation の境界
   - egui implementation が library の薄い利用者であること
   - UI state に依存しないこと
   - テスト、静的検査（lint）、抽象構文木検査（AST lint）

7. 作成後に検証する。

   ```bash
   npx -y @fission-ai/openspec validate "<change-id>" --strict
   ```

## 出力

最後に次だけ報告します。

- change id
- 作成した artifact
- 検証結果
- 実装に進めるかどうか

## 禁止

- katana UI、翻訳、アイコン、changelog、release 手順を前提に書かない。
- `tasks.md` を実装メモの羅列にしない。
- OpenSpec のテンプレート見出しを崩さない。
