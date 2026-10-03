## Context

公開0.5.9の既存DEBUGトレースと原本6件を用いた非競合診断で、監視終了待ちを切り分けた。親consumerはdev、workerはreleaseであり配布性能の証明ではない。監視はmacOSだけでOffice変換/rasterとXLSX持続workerに共有される。

## Goals / Non-Goals

**Goals:** 監視停止時の固定poll待ちを解除し、worker終了までjoinする契約と安全監視を維持する。同入力A/Bで監視終了段階・metadata/frame一致・全資源0を確認する。

**Non-Goals:** 変換/layoutそのものの高速化、HTML同期KRRの割込み、監視周期・制限・qualityの緩和。今回の局所改善をIssue #58/#59全体の完了に置き換えない。

## Decisions

- `thread::park_timeout(100ms)`と`Thread::unpark`を使う。stopをReleaseで設定してからunparkしjoinする。loopはAcquireでstopを再確認する。通知がpark前に届いてもtokenを保持でき、Condvar/Mutexや新依存を増やさない。
- 超過判定・kill・process refreshは既存のままとする。spurious wakeでは再確認するだけで100msより長い監視休止を導入しない。
- testは待機前と待機開始側への通知を実thread/channelで制御する。ready送信は実際のpark済みを保証しないため、待機開始側testを厳密なpark中同期の証明とは呼ばない。性能診断はtestの小さい時間閾値で代用しない。終了しない回帰にはfallback wakeでthreadを回収してからFAILする。
- 公開0.5.9と修正版の各監視sourceを同sysinfo rlib・同compilerで独立compileし、実process監視のA/Bを先に確認する。続いて製品build/全gate/consumerで確認する。standaloneだけでリリース完了としない。
- 改善を主張するのは監視終了待ちだけ。元XLSXの2.7s parse/全診断8.7sなど別段階の残差は未解消として保持する。

## Risks / Trade-offs

- [unpark通知の順序・lost wake] → stop Release→unpark、Acquire loop、park前とpark中の回帰。
- [監視安全性の退行] → 既存実process上限超過kill回帰とOffice/XLSX consumer資源検査を維持する。
- [診断と製品受入の混同] → producer/profile/graph/raw SHAを保存し、元packaged受入・RSS・fidelityを別の未完DoDとして残す。
- [他担当の負荷] → 実測前後のprocess/container確認、競合runは保全し性能比較から除外。

## 細分計測後の採否

同原本6件と代表Officeを候補release親/workerで実行した8caseは全exit0。大きいXLSXは展開後worksheet514,994,280bytesに対するAutoFilter catalog走査2503ms、ZIP完全性検査515msで、streaming判定/openは各整数0ms。終端までXMLを読む既存契約は後方autoFilterと不正XML拒否を維持するため、走査省略を採らない。計測だけで安全性を維持する追加最適化が実証されたとは扱わない。PPTXのworker全体は1365–4737msだが、外部変換/layout段階をこの修正の成果やKDV全体除外証明に読み替えない。最新の「明確な改善点なしに無理に改善しない」という報告を新しい数値DoDとしない。

追加readonly監査（Pascal、main照合）: quick-xml0.42.0のread_to_end_intoもread_event_implを回すため全XMLを解析する。check_end_namesを無効化せずに使えば整形式検査は維持できるが、FilterParserは要素の親位置を制限せずautoFilter/filterColumn/criterionを認識する。sheetData区間を丸飛ばし・無条件dispatch抑止すると、その区間で従来認識したfilter情報が消える。既存の後方filter・不正XML拒否テストと認識結果を維持する根拠がないので不採用。要素名判定を残すdispatch軽量化も、最適化済みコードで2503msの有意改善が測れたとは扱わず追加改修しない。readonly調査を新たな性能実測やwhole-issue解決の証拠にはしない。

## Migration Plan

既存cwdの単一release/v0.5.10で変更する。公開依存監査・全gate・Draft review/reply/resolve・Ready・required checks・通常merge・自動公開・fresh registry consumer・局所Issue証跡更新・cleanupまで追跡する。0.5.9のartifact/referenceは保持する。
