<!-- subagent-spark-harness-strict-start -->

## 1. 明示契約と依存

- [ ] 1.1 current Typography/Diagrams source・binary・locks・full PNG/geometry を照合し、crop/declaration不足と未評価を区別する。delegation-exception: `直列のクリティカルパス`。
- [x] 1.2 registry 公開境界・全依存更新候補と自然次版を確認し、版番号/lock/release契約を整える。証跡: file: `evidence/dependency-audit.md`、互換lock更新/target-check/契約self-test・実契約/V8 inverse tree PASS。Hayro0.8はrenderer二重化・未実測移行のため不採用。新graph全gateは3.1で未完。delegation-exception: `直列のクリティカルパス`。

## 2. 評価入口

- [x] 2.1 native fixture 結合・Typography 正常 decode と両方向 mismatch 拒否を実ファイル回帰で検証する。証跡: file: `evidence/native-fixture-binding.md`、標準RED1FAIL→GREEN6PASS、Python28PASS、入力非変更。依存更新後の全検証は3.1で別追跡。delegation-exception: `直列のクリティカルパス`。
- [x] 2.2 明示 sample/diagrams recipe と利用手順を追加し、既存 reference/95/geometry/破損拒否を維持する。証跡: file: `evidence/native-fixture-binding.md`、file: `../../../docs/current-preview-crop.md`、Justfile専用recipe dry-run/差分確認PASS、旧reference diff0。delegation-exception: `直列のクリティカルパス`。
- [x] 2.3 部分ゲート再開時のRUSTFLAGS漏れを専用workspace-test recipeで防ぎ、通常testの既存本体・対象・順序を保持する。証跡: file: `evidence/workspace-gate-resume.md`、dry-run三段維持/契約RED→GREEN1/最終recipe23suite2229PASS5ignored exit0。delegation-exception: `直列のクリティカルパス`。

## 3. 品質・公開・後処理

- [x] 3.1 fmt/AST/strict Clippy/全test/coverage100%/score/semver/strict release-check/V8 singleton を新sourceで通す。証跡: file: `evidence/quality-progress.md`、check81537 exit0、semver196/196、score全recipe段階成功、strict release-check23938実exit0/KDV_RELEASE_CHECK_EXIT=0、functions3742/3742・lines30730/30730、package/dry-run成功。標準checkの機械入力省略は実回帰RED→GREENで修正。file: `Justfile`、file: `scripts/subagent-spark-harness-lib.sh`、file: `scripts/check-subagent-spark-harness-tests.sh`、file: `openspec/changes/v0-5-12-current-typography-crop/evidence/harness-machine-output.md`。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.2 Draft PR/current-HEAD review/P0P1修正/個別reply resolve/fresh全thread/Ready/required全成功/通常merge/自動公開を完了する。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.3 GitHub/crates.io/checksum/VCS/fresh exact registry worker/consumer・局所Issue証跡・merged local branch cleanup を完了する。delegation-exception: `直列のクリティカルパス`。

## 4. 元DoDを保持

- [ ] 4.1 #58 current四分類独立95/採用、#59 元HTML正常close/全packaged-cleanmachine/残差帰属を根拠付きで確定してから対象IssueをCloseする。局所公開だけでClose/未完archiveしない。delegation-exception: `直列のクリティカルパス`。

## User Review Phase

- [ ] 残Issueの対応・必要公開・後処理までという人間指示を維持する。delegation-exception: `直列のクリティカルパス`。

禁止: stash/autostash/新worktree/clone/master編集/sibling編集/remote削除/no-verify/admin/path-git override/私有入力公開/品質・reference・geometry・deadline緩和。
