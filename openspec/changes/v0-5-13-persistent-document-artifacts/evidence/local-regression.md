# KDV局所回帰 2026-10-06

## 確認済み

- strict Clippy `just lint`: PASS（全workspace/all-targets/all-features）。
- AST `just ast-lint`: PASS（追加した計測/永続cache/testを200行/30行制約内に分割）。
- subagent harness標準suiteと実契約: PASS。実際の委譲モデルは利用可能なgpt-5.6-luna/mediumで、主担当が結果を検証し、推測のZIP修正を棄却した。
- 統合paged session通常回帰: 5 PASS、私有入力指定の3件は通常実行でignored。
- 合法PPTX descriptor: direct office2pdf Stored、KDV worker Stored/Deflatedが3 PASS。通常とDeflatedのRGBA一致。streaming ZIPを一律に拒否する仮説は実測で棄却。
- PDF計測: 実child processでdecode/render/rasterize/image_encode/frame_decodeを確認、1 PASS。
- 永続cache: 容量/破損/clear/内容・scale・environment・limits変更時miss、独立9process（PDF/DOCX/PPTXそれぞれcold/restart/revision変更）を含む2 PASS、child入口はparentから実行。restartのconversion/raster再実行がないことをtraceで確認。

## 計測範囲

2026-10-29訂正: 旧harnessの`first_frame_seconds`は、初回renderだけでなく再描画の検証とsession closeまでを含むcycle時間だった。以下の過去値は改変せずcycleとして保持し、正確な初回frame到達時刻や旧版からの速度改善には代用しない。初回render直後の時刻とcycleを分離し、末尾の公開入力で再計測した。

匿名fixture、scale 0.5、source取得から初回render・再描画・closeまでのcycle。native画面表示やOS clipboardを含まない。coldは新しい独立process/cache miss、restartは別process/cache hit、warmは同じprocessでdrop/reopen。

| 入力 | cold cycle | 別process restart cycle | 同process warm reopen cycle |
|---|---:|---:|---:|
| PDF | 0.074秒 | 0.053秒 | 0.053秒 |
| DOCX | 1.341秒 | 0.096秒 | 0.099秒 |
| PPTX | 1.305秒 | 0.104秒 | 0.099秒 |

この入力の旧cycle値は2秒以内、restart時は1秒未満だった。初回frame単独の正確な時刻を測れていないため、旧指標のまま性能合格とは扱わない。任意入力や実アプリの初回表示を保証する結果でもない。PDF geometry/outlineのdecodeはcache hit時も実行する。

## 未検証

KRR修正版の公開取り込み後の最終検査・公開/fresh consumerは未完。文書反映後の準備HEAD結果はPR68と最新の物理receiptへ記録し、まだ回収されていない結果は成功と扱わない。前回台帳から再利用できる私有入力の対応を確認できず、追加私有再測定は実施していない。全ディスク上に原本が存在しないとは断定しない。報告エラーと合法fixtureの成功を混同しない。

## 私有DOCX/PDF corpus（原本は公開しない）

同じ計測入口へ環境変数で原本を指定し、PDF 4件・DOCX 2件で各cold/独立process restartと各process内warm reopenを実行した。全12processが成功し、restart時のconversion/raster省略と終了後のresource counterゼロを確認した。raw logと内容SHA256はignored tmpにだけ保存する。

| 種別 | cold cycle | restart cycle | warm reopen cycle |
|---|---:|---:|---:|
| PDF 4件 | 0.034–0.061秒 | 0.012–0.015秒 | 0.012–0.014秒 |
| DOCX 2件 | 1.567–3.753秒 | 0.153–0.160秒 | 0.152–0.155秒 |

最も遅いDOCXの3.753秒は当時のcold cycle値で、office.conversionが2.913秒、最初のrasterが0.029秒だった。変換だけでも暫定約2秒を超えるため、全入力での性能合格とは扱わない。表のcycleから初回frame単独の正確値は算出しない。測定はscale 0.5で、実画面の表示時間は含まない。入力名・内容・画像・私有絶対pathは公開artifactへ記載しない。

## 2026-10-29 計測区間修正後の公開入力

初回render APIの成功直後に時刻を捕捉し、再描画とsession closeを含む`cycle_seconds`を別に残した。既存のhit、再描画一致、終了後resourceゼロ検査は維持した。実childのREADY→REPEAT→MEASURE順序と取得時間≦初回frame≦cycleを検査する回帰は、旧childで失敗し、修正後のstrict実行で成功した。

同じ公開PDF/DOCX/PPTXを通常（DEBUG未設定）/診断（DEBUG=true）各2回、cold/restartの24独立processで再測定し、全件成功した。各process内のwarm reopenも成功した。

| 入力 | 通常cold初回frame | 通常restart初回frame | 診断cold初回frame | 診断restart初回frame |
|---|---:|---:|---:|---:|
| PDF | 0.033–0.036秒 | 0.003秒 | 0.031–0.045秒 | 0.003秒 |
| DOCX | 2.427–2.506秒 | 0.279–0.289秒 | 2.228–2.319秒 | 0.280–0.286秒 |
| PPTX | 1.811–1.917秒 | 0.276–0.279秒 | 1.689–1.780秒 | 0.283–0.284秒 |

DOCX coldは暫定約2秒を超えている。これはdebug profile・scale 0.5・OS cache未resetの公開fixture測定であり、release profile、native表示、旧版からの改善、DEBUGによる速度差の因果関係は検証していない。KatanA原本で観測された13–21秒の原因や改善を、この小規模入力から断定しない。診断coldで変換/raster trace、restartで両traceの省略を確認した。

実行入口は`persistent_cache_contract`の実childで、私有資料を使っていない。binary/worker/入力/両test source/lockの前後SHA256一致、24件のログ・各初回frame/cycleはignored `tmp/kdv-true-first-frame-paired-measurement/receipt.json`に記録した。RED/GREENは`tmp/kdv-first-frame-clock-red.log`と`tmp/kdv-first-frame-clock-strict-green.log`。最終source/graphの全品質・公開前検査はtasks.mdで別途追跡する。

## 2026-10-29 前回私有入力の利用可否

Issue 65の計測記録12件（6種類の内容SHA）とIssue 67の原本記録3件（3種類の期待SHA）をローカルで照合した。両記録のSHA重複は0件で、corpus内の文書ファイル、参照パスから利用可能なファイルへの解決、利用可能ファイルとのSHA一致はいずれも0件だった。前回入力の再利用を確認できないため、追加私有測定は行っていない。全ディスクからの不存在やIssue 67歴史原本の取得・修正を主張しない。匿名番号と件数のみの証跡: ignored `tmp/kdv-current-private-first-frame/receipt.json`。
