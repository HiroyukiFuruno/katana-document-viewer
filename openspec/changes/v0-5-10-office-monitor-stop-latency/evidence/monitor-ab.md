# Issue #59 Office監視待機の局所A/B

対象Issue: https://github.com/HiroyukiFuruno/katana-document-viewer/issues/59

## 条件

親はdev/opt-level=0、同0.5.10候補/同lock/同rustc/同probeを用い、controlだけ監視を公開0.5.9のsleep実装へ戻してcompileした。candidate compile後にcontrolをbuildし、両probeを凍結してから製品sourceをcandidateへ復元した。workerは両方で公開0.5.9のrelease worker（SHA256 `2af9765859bf9be1839143115d6ac00cccae5f6f08a6b807c93f846b9437e214`）を固定した。これは親監視だけの局所比較であり、配布mainや新worker全体の受入ではない。

親lock SHA256 `52443bbe3b42ffb535cd07dc4e8ebd61a84a5ee41e2ec44dd2e050c19c80f796`、同registry KUC0.4.1/KRR0.4.22/office2pdf0.8.0/V8 singleton152.2.0。互換cc/font-types/uuid更新も両親で同一。元6Officeと代表XLSX、代表DOCX/PPTXの8caseを各control/candidateで実行した。全16run exit0、測定前後の競合build/runtime/containerは0。途中競合した初期代表runはこのA/Bとは別に保全し、性能比較へ含めない。

## 観測

| 入力区分 | control監視終了ms | candidate監視終了ms |
| --- | --- | --- |
| 代表DOCX/PPTX（変換/raster計6回） | 2,81,73,106,78,68 | 0,0,0,0,0,0 |
| 原本PPTX A（変換/raster計4回） | 80,70,93,85 | 0,0,0,0 |
| 原本PPTX B（同4回） | 80,95,29,8 | 0,0,0,0 |
| 原本PPTX C（同4回） | 0,16,46,16 | 0,0,0,0 |

0msは整数ms計測の表示であり絶対的なゼロではない。wall時間の差には変換時間の揺らぎもあり、全Office速度の改善率をこの単発比較から算出しない。

7caseのframe/metadata/config/close計77fileはbytes一致、元6OfficeのPage9実RGBA/PNG・Grid9typed結果を含む。Gridは描画済み画素とは呼ばない。close全8counter0、代表DOCX/PPTXの既存PDF/RGBA容量分離・6frame/cache契約も成功した。元RSS予算、原本要素fidelity、root packaged/HTML正常close、Issue #58の独立三分類は別の未完DoD。

## 回帰と制限

同sysinfo rlibを使って公開registryの実monitor sourceと候補sourceをstandalone compileした。最終test構成で旧実装は通知3件FAIL/実process2件PASS（session70783 exit101）。候補は5件PASS（standalone、製品Cargo gateは別途必要）。通知前の制御にmpsc内部parkを使う初期testはtokenを消費し得たため採用せず、実threadのAtomicBool/yield同期へ修正した。agent Galileoの最終testにも参照名誤りとunused importがあり、mainが実compileで検出・修正した。subagentの未実行報告をPASSへ転用していない。

macOSの既存linker `__eh_frame` warningはdev worker buildに残った。-D warningsでもlinker_messagesは対象外であり、警告なしとは扱わない。

## 残るOffice段階

候補release lib/workerの実buildはsession29009 exit0、workspace監視5testはsession28161 exit0。ready通知だけではpark済みを保証しないことを自己レビューで確認し、testを待機開始側への通知という名称へ整合した。全品質ゲートは別途実行する。

候補release親/workerで同8caseを実実行したsession22024は全exit0。原本6件と代表XLSXのframe/metadata/close70fileは同graph親A/B候補の結果とbytes一致（worker pathを含む設定7fileは比較対象から分離）。大きいXLSXのrelease実段階はZIP完全性515ms、AutoFilter catalog2503ms、streaming判定/openは整数0ms。全体約3.39秒の局所probeであり元配布性能の代わりにはしない。後方filterと不正XML/CRC拒否を維持するため走査省略は採らず、新規最適化は実証済み監視に限定する。

公開workerの大きいXLSXはpackage_parse約2709ms、session_open約8244ms。worksheetのZIP展開後サイズは514,994,280bytesだった。親preflightとworkerのfilter/import/streaming/evaluateを細分計測し、CRC/ZIP安全検査やfilter動作を省かず原因と最小修正を判断する。監視修正のみでOffice全体改善・Issue Closeとはしない。
