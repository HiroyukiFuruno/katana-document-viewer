# PR62 計測二重加算の回帰

指摘: https://github.com/HiroyukiFuruno/katana-document-viewer/pull/62#discussion_r4172140862

## 原因と範囲

最新の人間指示「二重計上直すで良いです！」により、この最小修正の採用は明示承認済み。採否待ちとして停止しない。

既存親preflight_diagnosticsが全体をoffice.preflightで囲むのに、archive inspectへ同名inclusive traceを追加したため、同session/sourceで2行出る。parse_stage_elapsedは同名を加算する。nested archiveも同名で再帰し、合計を正しい残差へ使えない。P2というラベルだけで見送る対象ではなく、このPRの既承認「正確な独立段階計測」を満たす局所バグとして修正する。以前のP2見送り指示を新PRの計測誤りにも一律適用して追加確認で停止した判断を訂正した。他taskのメッセージを新たな編集許可とは扱っていない。

外側office.preflightは非変更。内側は独立名office.package_inspectionへ分離しdepth0だけ計測する。root inclusiveの内部にnested時間は既に含まれるため、同名の再帰traceを追加しない。ZIP各archiveの完全性検査、処理順、安全上限、DEBUG/session/source、公開API/表示/95点/元DoDは非変更。

## 実回帰

新しいnative testは自身の実test binaryを子processとして実行し、DEBUG=trueのstderrを取得する。Mock、固定待ち、timeout変更、私有原本は使わない。

- preflight_trace_has_one_outer_stage_and_one_root_inspection: 実代表DOCXを公開sessionの事前検査へ渡す既存missing-worker typed failure経路で、外側1/内側1行を検証。旧source10154 exit101、同session/sourceのoffice.preflight実2行。候補18184で1PASS。
- nested_preflight_trace_counts_root_inspection_only: 実ZIPの既存nested depth拒否経路で、直接public preflightのroot inspection1行・外側preflight0を検証。旧source44377 exit101で同名4行、候補18184で1PASS。既存ResourceLimitExceeded期待は非変更。

コマンド: `rtk proxy env RUSTFLAGS='-D warnings' CARGO_TARGET_DIR=tmp/issue59-rss.oBZVZI/cargo-target cargo test -j2 --locked --no-fail-fast -p katana-document-viewer --test multi_format_office_worker_contract --test multi_format_office_preflight_contract preflight_trace -- --nocapture`。

Pauliのsidecarはprofiling checker一ファイルのみ。owner重複/親欠落/depth gate逆転/新stage欠落のmutationを追加、mainが実self-test exit0と旧source validator exit1、候補just office-profiling-stage-check exit0を確認した。これはsource marker契約でありRust AST完全証明とは呼ばない。実native trace回帰で併せて検証する。agentは終了済み。

## 旧計測と公開境界

旧原本の全raw/recipe/producer/hashは保持。同名preflight集計は以後原因判断に使わない。監視A/Bのmonitor_finish、独立wall、frame/資源、XLSX filter_catalog2503ms/zip_integrity515msの個別行を二重preflightとの差引へ転用していない。新しい速度改善値をこのtrace修正から作らない。

## 修正後の全入口

製品修正commitは53e6af5。新source全release-check82524はexit0、raw SHA256 `c315a6be6ee1a225ea6dd68e2de67295ed1fcd4c1d46a226615568765ecc0108`。functions3742/3742・lines30730/30730で既存必須100%、regionsは41331中495未到達の98.80%であり全region100%とはしない。通常core1960PASS、Storybook640+isolated1、native trace回帰2件、strict Clippy/AST/OpenSpec/semver196/196が通過。台帳verifyの許可コマンド形不備は実行済みrtk just office-profiling-stage-checkへ整合し、harnessの規約を変更せず実検査89901 exit0。台帳補完後AST4426/OpenSpec77101もexit0。

現candidate scorer SHA256 `3c87893bbf342ac6f769e81cbf98db9ce2baa8d5181c17e6f0c1e9765165c464`で通常除外する23scoreを実行し全PASS、独立visualはsample99/diagram100・threshold95、ignored export parity2PASS、外部current crop1PASS/100/95/average99。crop参照パスの初回誤指定exit101と、短縮名へexact指定した0testの実行は検証成功に数えずログを保持。実在入力と正しい名前に整合した別logで必要各testの実行数とexit0を確認した。原本/reference/producer identityは変更しない。

新source親/凍結dev workerの代表Office fidelity48909はexit0、verify-recordもexit0、record SHA256 `0e6ada7fc10e9e219293d374d9040936070f064a4103ea9a033146ca8d309d56`、tolerances_met=true/failures空。これは代表fixtureの既存fidelity契約であり元6原本・packaged-main・速度/RSS受入ではない。915fileのpackageに実stage修正が含まれsource cmp0、publish dry-run/未公開確認PASS。非fatal __eh_frame linker警告を保全。既存recipeは通常target生成物28.1GiBだけcleanし、ソース/原本/参照/凍結binary/rawは保持した。

通常push・同thread reply/resolve・current-HEAD再レビュー・fresh全thread・新HEAD3OS/preflight・公開/fresh consumerは未完。旧HEAD CIの成功を代用しない。HTMLはKRR公開後のroot元入力受入、Issue58非visualは独立証跡と採点規約の未完を維持する。
