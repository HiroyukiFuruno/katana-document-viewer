## 1. OpenSpec

- [x] 1.1 CLI入口とapp/API入口のtheme責務を分けて記述する
- [x] 1.2 `katana-light` をlight presetの基準として記述する
- [x] 1.3 `--theme` JSON入口を別changeへ分離する

## 2. Theme contract

- [x] 2.1 `KdvThemeSnapshot` を追加する
- [x] 2.2 `KdvThemeSnapshot` の配色fieldを全てnot nullにする
- [x] 2.3 `katana-light` presetを追加する
- [x] 2.4 `katana-dark` presetを追加する
- [x] 2.5 app/API向け `ExportRequest` でthemeを必須にする

## 3. CLI contract

- [x] 3.1 `CliThemeMode::Light` / `CliThemeMode::Dark` を追加する
- [x] 3.2 CLI export入口でtheme modeを省略可能にする
- [x] 3.3 CLI theme未指定を `Light` に正規化する
- [x] 3.4 export debug exampleに `--light` / `--dark` を追加する
- [x] 3.5 `--thema` を受け付けないことを明示的に検査する

## 4. Export適用

- [x] 4.1 HTML export stylesheetをtheme snapshotから生成する
- [x] 4.2 HTML rootへ `data-kdv-theme` を出力する
- [x] 4.3 diagram theme markerをtheme modeから決定する
- [x] 4.4 既存の固定light CSSをtheme field参照へ置き換える
- [x] 4.5 app/APIから渡されたcomplete theme objectをKRR `RenderContext.theme` へ渡す

## 5. Verification

- [x] 5.1 app/APIでthemeなしの `ExportRequest` を作れないことを型で固定する
- [x] 5.2 CLI未指定が `katana-light` になるUTを追加する
- [x] 5.3 CLI darkが `katana-dark` になるUTを追加する
- [x] 5.4 HTML exportのCSS変数がtheme snapshotから出るUTを追加する
- [x] 5.5 diagram build時にKRRへcomplete theme objectが渡るUTを追加する
- [x] 5.6 OpenSpec strict validationを通す
- [x] 5.7 `cargo fmt` / 対象UT / `cargo clippy` を通す

## 6. Feedback

- [x] 6.1 diagram背景をcode block背景から分離し、transparentとしてtheme契約へ追加する
