# #67 PPTX/ZIP切分け 2026-10-06

## 出典の確認

報告元の添付画像ではPNGを表示しており、そのPNG自身に過去のPPTXエラー画面が含まれていた。ローカルの資料PNGを確認し、記載された過去のPPTXパスは現在存在しないことを確認した。現在の実workerから同じエラーが新たに発生した証跡ではない。原本・PNG・内容・private URIはGit/公開Issueに添付しない。

## 現行実装の解析経路

KDVはzip 8.6のseekable `ZipArchive::new(Cursor<&[u8]>)`でcentral-directoryを読み、local-header/data-descriptor/payload CRC・path/entry/size等を検査する。Office変換器もzip 0.6.6のseekable archiveを使用する。問題の文言は両zip版のstream readerに存在するが、現行KDV/変換器のseekable呼出しはそのstream readerへfallbackしない。

合法descriptor PPTXを直接変換器へ渡すと成功したため、想定していたworker ZIP正規化は棄却し、製品コードには残していない。Stored/Deflated descriptorのlocal bit3/CRC-sizeゼロ/central-directoryを検査し、実workerで非空frameを生成する回帰を追加した。Deflated入力は通常入力とRGBA完全一致。

## 現在入手可能なprivate PPTX corpus

資料directoryのPPTX3件（約40.9MB、5.8MB、18.7MB）を、既存producerで各2独立processずつ開き、全6操作が成功。同一sourceの反復frame/closeも既存受入で検証し、元のpreflight、安全制約、worker timeout/メモリ上限を維持した。raw logs/JSON/path/hashはignored local tmpにだけ保存した。実行sourceは0.5.13作業版で、ZIP/変換処理に変更はない。

初回frameは2.437–10.978秒で、2秒目安を満たさないprivate PPTXもある。今回のZIP解析の成功を性能改善の証拠にしない。計測には同時build/OS負荷の影響があり、数値はこの観測窓の結果として扱う。

## 判定とowner handoff

過去画像のPPTX原本は未入手で、当時の失敗の再現/修正を主張しない。現行KDVの合法descriptorおよび入手可能な3入力にZIP拒否は再現せず、推測修正や入力破損の断定は行わない。再発時は実入力と実際に起動したworkerの版/hashを結合して調査する。下流の実worker選択/実アプリでの原本受入はKatanA owner Issue #345へ、この要求とKDV局所証跡を引き渡す。KDV自身の局所検証/公開は下流の採用完了待ちで保留しない。
