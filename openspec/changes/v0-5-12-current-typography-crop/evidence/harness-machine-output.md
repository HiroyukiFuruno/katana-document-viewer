# 機械入力を表示用に省略しない

公開KRR0.4.23採用後の標準checkはfmt/strict Clippy/AST/全test/契約/V8実linkまで進み、subagent証跡ハーネスの正常fixtureでexit1となった。終了したcheckをPASSや進行中として扱わない。

原因は `scripts/subagent-spark-harness-lib.sh` の機械入力に `rtk rg --files` の表示用圧縮が入ったこと。実際の完全パス2件がディレクトリ見出しとbasenameへ変わり、後続の存在判定がtasks/handoffを認識できなかった。レンダラーやKRRの実行失敗ではない。

`scripts/check-subagent-spark-harness-tests.sh` に実ファイル/実検索の回帰を追加した。2changeの4完全パス・archive除外と60行全ての証跡/行番号を同一文字列として検査する。追加回帰は旧実装でRED exit1を確認済み。機械消費される検索とsortだけを `rtk proxy` へ変更する。証跡要件・拒否条件・対象change・閾値は変更しない。

追加回帰RED exit1/raw SHA `ce5f8c4ccd6ffe23a0bd213c67cf2fc5e8affadea604972970e3199959fcbdf9`。修正後の同テストGREEN exit0/raw SHA `286d473f938bc1b12ad6a3dc6754ff2b14cebf14cb0914e44c8a6d996eb36e2e`。既存recipe全ハーネス9検査/実本体とcrop Python28はsession65500 exit0/raw SHA `0ee534ecda065aee3168e4fe0fec70020af25bd19eb3e4cb734b85edce79f2e5`。既存negative controlを維持し、Bash構文/diff-checkもexit0。標準全checkは旧FAILのままで再実行待ち、focused GREENを全gate完了にしない。

readonly Herschel `gpt-6-luna` / reasoning `medium`、agent `01a10b9b-491f-7040-87a4-72d5618ae6b5` は対象2fileの差分を独立レビューしfinding0、mainが実diff/結果を確認してclose済み。元Issueの外部crop採点/意味・操作・OSclipboard/配布性能をこの検査へ置き換えない。

追加互換回帰: 実RTK/find/grep/sortのみをPATHへ提供し、外部rgがない環境で既存find/grep経路を検査した。proxy化だけの候補はRED exit1/raw SHA `d1c539ed4a812164445571518f24ece14002683a626a23aeba9c9c31d6e7ecd0`。RTKと外部rgの双方が存在するときだけproxy rgを選ぶように修正し、既存fallbackを保持。同回帰/既存mainテストは18250 exit0/raw SHA `286d473f938bc1b12ad6a3dc6754ff2b14cebf14cb0914e44c8a6d996eb36e2e`。これは独立レビュー後の追加差分でmainが確認、最終全ハーネスは後続で再検証する。
# 最終全ハーネス追補

外部rgなし互換修正を含めた最終9検査と実本体は4922 exit0。raw harness-final.log SHA `a2b52d8d1757f4c203a733312a9a8c6c828b0f2fb2b6623af4030d93bc4824d1`。追加差分前の65500成功とは別epoch。新graph全体checkの再実行は別途必要。
