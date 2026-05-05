---
name: openspec-apply-change
description: katana-document-preview の OpenSpec change を読み、tasks.md に沿って実装する。仕様、設計、タスクがある変更の実装開始や継続で使う。
---

# OpenSpec Apply Change

OpenSpec の artifact を一次情報として読み、`tasks.md` の順番で実装します。
実装は kdp の文書プレビュー基盤（document preview runtime）に閉じます。

## 実行入口

専用入口があればそれを優先します。なければリポジトリルートから次を使います。

```bash
npx -y @fission-ai/openspec <command>
```

## 手順

1. change を選ぶ。

   名前が指定されていない場合は、active change を確認します。

   ```bash
   npx -y @fission-ai/openspec list --json
   ```

   active change が複数ある場合は、勝手に選ばずユーザーに確認します。

2. 状態を確認する。

   ```bash
   npx -y @fission-ai/openspec status --change "<change-id>" --json
   ```

3. 実装指示を取得する。

   ```bash
   npx -y @fission-ai/openspec instructions apply --change "<change-id>" --json
   ```

4. `contextFiles` に含まれる artifact を読む。

   - `proposal.md`
   - `design.md`
   - `specs/**/spec.md`
   - `tasks.md`

5. 未完了タスクを 1 つずつ実装する。

   - タスクの意図を先に確認する。
   - 変更範囲を最小にする。
   - バグ修正では先に再現テストを置く。
   - 完了したタスクだけ `- [x]` にする。
   - 不明点が仕様判断に関わる場合は止めて質問する。

6. タスク単位で検証する。

   - 変更に対応する unit test
   - 必要な integration test
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - 既存の抽象構文木検査（AST lint）があれば実行

## kdp 固有の確認

- library と egui implementation の責務が混ざっていない。
- neutral interface と egui implementation の境界が保たれている。
- version と boundary 検証を曖昧にしていない。
- 外部プロセスの失敗を握りつぶしていない。
- UI state、WebView、React、editor/preview の都合を入れていない。

## 完了時

実装後は `/lint-and-ast-lint` と `/self-review` を通します。
OpenSpec 完了確認は `/openspec-verify-change` に渡します。
