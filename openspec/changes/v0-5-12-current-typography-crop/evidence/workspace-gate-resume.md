# Workspace gate 部分再開の標準設定

18:46Z heartbeatでlive空き7.5GiB/owned buildなしを確認し、未完workspace/all-target/all-feature検査を再開した。初回のdirect起動18050はRUSTFLAGS/CARGO_BUILD_JOBS未設定を確認し、exact ownedCargo2761だけSIGINTで中断exit130。private raw SHA `884b93622a94964b98468032f07ea704fcf4e7a9bf944a03efb68c80605a88b8` を保持し、PASS/RED/完了にしない。

標準 `RUSTFLAGS=-D warnings` と `CARGO_BUILD_JOBS=1` を明示した同workspace検査を62081で開始した。既存Justfile第一段と同じworkspace/all-targets/all-features/locked/exclude kdv-storybookであり、新source全gate未完を維持する。

再発防止としてJustfileに `workspace-test` を併設し、export RUSTFLAGSを継承、`-j {{JOBS}}` を明示する。通常testの既存本体は非変更で、dry-runはworkspace→既存skip付きStorybook→既存mouse_click単独の三段順序/対象を保持した。今後の部分再開は `rtk proxy just JOBS=1 workspace-test` を使い、独自に既存recipeを転記しない。

readonly Einstein (`gpt-6-luna` / low、agent01a10842-daa9-7c51-bf4f-a70ea005a3c2) は5対象条件・三段順序・export RUSTFLAGS/JOBS維持・並列化追加なしを差分照合し終了/close済み。workerは旧HEADに未追跡tasks.mdがないため2.3との整合を未判定とした。mainは現tasks.mdの2.3を直接読み、recipe/dry-runに対応することを補完した。旧HEADのtasks不在を現物不在としない。

実62081はexit101/core1959PASS/1FAIL/1ignored。依存recipeへ移した旧案は既存Justfile契約に反し、通常testを元本体へ戻した。追加workspace-testを前置すると既存recipe_bodyの`test:`部分一致がその名前へ先に一致するため、追加入口を通常testの後ろに配置した。parser/契約testを緩和していない。focused86351はFAILを保持し、配置修正後53907は1PASS/exit0/raw SHA `b50ce6a10d4f4aba46810cf646c72329d41c16a7b36a23a975ef2dfb2a4ad2f4`。

最終専用recipe実行91840はexit0、workspace/all-targets/all-features/locked/exclude kdv-storybookの23suite/2229PASS/0FAIL/5ignored。private raw SHA `3e93d1127bb5dbdfbe655cb668b0e4f4c5d71c1e821ea76dbcec57007744e55d`。62081/86351/53907/91840は終了、再pollしない。全workspace成功をcoverage/未実施score/semver/strictreleasecheck/PR/公開/元Issue完了へ昇格しない。原本/参照/既存rawは非変更、Cargo安全拒否をmarker修復/別削除で迂回しない。
