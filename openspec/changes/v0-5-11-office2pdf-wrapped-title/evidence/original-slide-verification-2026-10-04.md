# office2pdf0.8.1 元46slideの局所再検証

公開依存office2pdf0.8.1/KRR0.4.22/KUC0.4.1/V8152.2.0/libc0.2.190の候補workerで、元3PPTXの9/11/26slide、全46pageを既存memory2GiB/CPU46秒/wall45秒/output128MiB/env空のまま変換した。原本・独立LO参照・旧PDFは非変更。collector15982 exit0、別pypdf/実bytes verifier3209 exit0。PNGや目視だけで実変換を成功にしていない。

| 証跡 | SHA256 |
| --- | --- |
| 採取時worker | d0c5314f855750dfb821d3f8acab72b584075e5b963b9fef1eeeb221d0fef453 |
| 採取時lockfile | 2171a915db6201f8c1466c03a6a00bfa3c9b7c55bd258a7409d2c73531c861cf |
| 独立参照manifest | e36a18c78601ed2105a696a55a5141218456b766d3646e44de0b2266ca5e985f |
| 新candidate manifest | 2532a435dcdd568725ba89a5e13956618a0df40ba0f88edd694e7425b53524ab |
| 独立照合receipt | fa67ed96f9d73d7a442b5b58ef37aaf9320c37e1a46b3e963295410727f53067 |

原本見出しの自然PDF変換は指定26ptを保持。全pages/MediaBox/主relation順序/有限word矩形と字句対応差を記録し、6不正矩形を拒否する検査を実行した。画像化slide・文字multiset・自然PDFの矩形差を意味的欠落数やfidelity95点へ読み替えない。geometry tolerance、packaged main、host viewport、全clean-machine、HTML正常close、独立非visual三分類は未評価。

後続PNG native回帰のためStorybookへtest-only tempfileを直接登録し、現lockは90d89a556a81d8523d9c63d642f6b1661ff366c22935f76eb1f93eac329e33d1へ変化した。旧生成epochのlock/binary/resultを新hashへ書き換えない。この登録は既存tempfile packageを再利用し、新しいpackage/versionを解決していない。元46slideのworker normal graphとは別のtest-only所有関係である。

全strict release-checkは最初の68397でAST台帳の証跡ラベル不足によりexit101。semver196とstrict Clippyの実通過を全gate成功に拡張しない。ラベル修正とPNG回帰追加後に新sourceで全入口を再実行する。原本内容・PDF/PNG・私有pathはGitHubへ掲載しない。
