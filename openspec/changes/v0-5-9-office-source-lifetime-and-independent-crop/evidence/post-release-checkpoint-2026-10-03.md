# v0.5.9 公開・整理の状態訂正

旧tasksの4.2/4.4が未完表示のままだったため、実公開と整理の証跡に一致させた。これによってIssue #58/#59の未達条件を完了へ置換しない。

- PR61: `MERGED`、head `71597736c9a968eded1cbe54f65b4b102fd9ef4a`、通常merge `a5e9802288312a2bbf407dec27c8e0ea71c23ed3`。2026-10-03のfresh GraphQL `reviewThreads(first:100)`はnodes空/hasNextPage=false。
- CI run37017695647、preflight37017695831、Release37028672331の2026-10-03再取得は各status completed/conclusion success。
- GitHub Release/crates.io公開およびregistry VCS/tag merge一致は公開時に照合済み。cached registryの実`.crate` SHA256を再計算して `6e0ad33a740afd7a98a62aa3adbf6379eea530db53764305b7488a207602d9c0`と一致した。
- 公開時fresh Linux registry link/tree、Mac公開worker DOCX/PPTX計6frame・全close8資源0は成功。これを元GUI/配布性能・独立三分類の受入へ転用しない。
- 旧stash5件は正式採否/必要差分統合・不採用根拠/回帰/公開後consumerの照合後、各OID guard付きでdrop済み。2026-10-03 `git stash list`は空。新stash/autostashなし。
- masterはmergeへFF済み、2026-10-03 `master...origin/master`は0/0。旧merged local release/v0.5.9は削除済み。remote branchと既存root worktreeは保持。現在の新しいrelease/v0.5.10は監視待機修正の作業中なので不要branchとして削除しない。

元HTMLの公開KRR修正版後の正常close、元配布/clean-machine受入、Office残差判断、Issue #58の独立非visual三分類各95点は未完。旧changeをarchiveせず4.3/2.3/3.2を未完で維持する。
