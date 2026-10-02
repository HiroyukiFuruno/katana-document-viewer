## Context

Issue #59の公開0.5.8診断では、全counterとmalloc in-useがcoldへ戻ってもRSSが残った。KDVなしのhayroでもRSS+225088KiBが再現する一方、KDVは変換成功後も原本をPDF解析完了まで保持している。第三者backendの再現だけではKDV原本寿命の寄与を除外できない。

## Goals / Non-Goals

Goals: 原本の不要な高水位重複をなくし、実製品A/Bで効果と残差を測る。#58は外部artifactのSHA・寸法・frame provenanceを照合し、四項目各95点を独立に評価する。

Non-Goals: allocator置換、OS memory reliefによる受入代替、backend/他repoの無許可編集、参照上書き、画質・deadline・coverage・安全上限の緩和。

## Decisions

- 既存`static_artifact`へsourceを先にmoveする。同helper終了でbytesがdropされ、PDF解析前にはmetadataだけが残る。単なる`bytes: _`のdestructureはdrop時点を保証しないため採用しない。
- conversion keyは従来どおり変換前に計算し、診断・mime・identity・format・capabilitiesを保持する。PDF解析後にitemsとcountを確定して返す。
- RenderCache再利用は診断で大型割当/RSSを解消しなかったため、根拠のない追加変更をしない。
- 原本寿命だけではRSS効果が再現せず、実VMの主残差は解放済み大型malloc領域だった。同実frameを親へ返す隔離prototypeを測定してから、KatanAが使う`DocumentSession`のOffice rasterのみを既存sandboxed workerへ移す。PDF metadata/制限付きframe cacheは親、復号の大型一時割当はchildとし、workerの通常終了・timeout・強制終了で親へresidentを残さない。
- 既存public API/エラー型は変更しない。高水準Office frameはprivateなworker-render入口を使い、既存`DocumentSessionError::Office`で具体的worker失敗を返す。直接PDFと低水準Officeの既存render APIは互換動作を維持し、隠れたin-process fallbackを追加しない。
- 公開`OfficeWorkerConfig.max_output_bytes`は従来の変換PDF上限のまま維持する。private rasterのRGBAは既存PDF描画pixel上限から導いた別の内部上限で親子検証する。小さなPDF上限がRGBA成功を拒否する互換性回帰を実workerで確認する。
- self-testだけnative Git repository-local環境を隔離し、hookの`GIT_DIR`等が一時fixtureから呼び出し元metadataを変えないことを実Git回帰で検証する。通常の製品Git操作と既存hook委譲は変更しない。過去のbare設定破損経路の確定とは区別する。
- crop候補を明示入力とし、provenance不一致は失敗する。PNGの類似度だけからsemantic/interaction/performanceの点数を推定しない。
- release/v0.5.9一つで作業する。stash/autostash、新worktree/cloneは禁止。元ファイルと過去stashは意味的照合する。

## Risks / Trade-offs

- [寿命短縮だけでは全RSS基準を満たさない可能性] → 同実入力のcold/warm/異なる入力を再測定し、残るbackend/allocator責務をIssue-firstで引き継ぐ。満たさないまま解消扱いにしない。
- [disk空きが少ない] → 既存cacheを優先し、生成物の所有と使用状況を確認してから必要な限定cleanupを行う。品質ゲートを省略しない。
- [公開cropに挙動証跡が不足] → 不足分を未対応として残し、自己比較で埋めない。

## Migration Plan

回帰・実A/B→依存調査/完全gate→Draft current-HEAD review→個別reply/resolve→Ready/required checks→merge/自動公開→fresh registry consumer/元下流受入→Issue終了/安全cleanup。

## Open Questions

- 製品原本寿命短縮はmetadata/frameを保つが、RSS改善は確認できていない。同locked graphのraster隔離では元XLSX2/PPTX3のRSS増分262144→136256KiBを実測した。最新registry graphでも同5Office141248KiB、最大PPTX11反復cold98496/warm4336KiB、全close8counter0を確認した。mixed HTML/PPTX・公開版での元下流GUI受入はまだ未検証であり、Office単体成功で代替しない。
- #58の四項目独立採点と現行artifact採用判断は未完了。
