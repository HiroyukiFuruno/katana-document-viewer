# 現sourceの厳格品質検証

現lock90d89a556a81d8523d9c63d642f6b1661ff366c22935f76eb1f93eac329e33d1、native scorer source541836d673450e186f9cbe3cda4a6d19d22496ab6d5f68522c291e2dc9e44b90に対し、標準`just VERSION=v0.5.11 release-check`をJOBS1/CARGO_BUILD_JOBS1/CARGO_INCREMENTAL0で実行した。外側pipefail+teeで各stageの実結果を保全している。

## 実完了

- release target/contract、semver196、fmt/strict Clippy/AST/boundary、core1960 PASS/1ignored、実worker wrapped title3、通常Storybook644+isolated1、新PNG4、V8 singleton152.2.0/実native link、全harnessと通常check終端PASS。
- 最終scorer c599ea7b…の追加23score/ignoredexport2/外部crop1は各exit0。sample99/diagrams100、外部visual100/average99/95を維持。外部producerは旧凍結candidateであり、current root四分類受入にはしない。詳細はsupplemental-quality-2026-10-04.md。
- 同sourceの厳格coverageが既存条件のまま成功し、recipeは後続packageへ進んだ。functions3742/3742、lines30730/30730で各100%、uncovered functions/lines各0。regionsは40836/41331（98.80%）で、既存のfunctions/lines必須条件と混同しない。
- RTKがsummary返却を省略するため、元の全coverage集計logを独立copyへ保全し実SHA一致を確認した。SHA bc04f3fae824aa8538c43414b4b6bd7c7821e9a6de7c685b206a355bf2b685f3。通常target/coverage targetの再生成cacheは標準recipeがcleanし、固定binary/原本/参照/旧FAILは保持した。

## 完全gate終端

73836は終了コード0。package verificationはdev profileで7分33秒、publish dry-runの再verificationは17.32秒で成功。916files/15.0MiB/圧縮7.3MiB、版検査と未公開検査もPASS。macOSの既知非fatal linker `__eh_frame section too large`警告は両buildの生logに保持し、隠蔽やlint無効化をしない。主log SHA6b92ad786a55881fc7a1f4b21cbbf66fe798c11d0fe450dc844870979a57231e、完全coverage集計は前述bc04f3fa…の別logへ保全した。

生成candidate archiveのSHA2d38bf5a44d48ecf60468b86091dd4599f918d7e676c3fb1fe890105cc11bdd3は未commit候補の値で、公開crateのchecksumには転用しない。実archiveにはprivate tmp/source-referenceの入口なし、正規化manifestのoffice2pdf exact0.8.1/KUC exact0.4.1/V8 exact152.2.0を確認した。

## 未完

commit/push、Draft current-HEAD review/required CI/merge/公開/fresh consumerは未完。元Issue58/59の全packaged/clean-machine/HTML正常close/非visual採点は別途未完。これらを上記成功から推定しない。
