## 1. OpenSpec

- [x] 1.1 `--theme` を正式option名として固定する
- [x] 1.2 `--thema` を不採用として記述する
- [x] 1.3 JSONもcomplete theme objectにする方針を記述する

## 2. Implementation

- [ ] 2.1 `--theme <json>` をCLI parserへ追加する
- [ ] 2.2 JSONから `KdvThemeSnapshot` を読む
- [ ] 2.3 欠落fieldをエラーにする
- [ ] 2.4 `--light` / `--dark` との同時指定をエラーにする
- [ ] 2.5 `--thema` を未知optionとして拒否する

## 3. Verification

- [ ] 3.1 完全JSONがexportへ反映されるUTを追加する
- [ ] 3.2 部分JSONが失敗するUTを追加する
- [ ] 3.3 `--thema` が失敗するUTを追加する
- [ ] 3.4 OpenSpec strict validationを通す
