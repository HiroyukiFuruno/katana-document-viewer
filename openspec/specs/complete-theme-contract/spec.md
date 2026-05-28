## Purpose

KDVのCLI入口とapp/API入口でtheme責務を分け、色の欠落や暗黙fallbackでHTML/PDF/PNG/JPGの見た目が壊れる経路をなくす。

## Requirements
### Requirement: CLI入口は省略可能なlight/dark theme modeを受け取らなければならない

システムは、CLI向け入口でテーマ（theme）指定を必須にせず、light/darkの簡易指定を受け取らなければならない（MUST）。

#### Scenario: CLIでtheme modeを省略する

- **WHEN** CLI入口がtheme mode未指定でexport requestを作る
- **THEN** KDVは `katana-light` と同じ完全テーマを使う
- **THEN** 出力には `data-kdv-theme="katana-light"` が含まれる

#### Scenario: CLIでlightを指定する

- **WHEN** CLI入口が `--light` 相当のtheme modeを受け取る
- **THEN** KDVは `katana-light` の完全テーマを使う

#### Scenario: CLIでdarkを指定する

- **WHEN** CLI入口が `--dark` 相当のtheme modeを受け取る
- **THEN** KDVは `katana-dark` の完全テーマを使う
- **THEN** 出力には `data-kdv-theme="katana-dark"` が含まれる

### Requirement: app/API入口はcomplete theme objectを必須入力にしなければならない

システムは、app/API向け入口で完全なテーマ（complete theme object）を必須入力として受け取らなければならない（MUST）。

#### Scenario: app/APIがexport requestを作る

- **WHEN** app/APIが `ExportRequest` を構築する
- **THEN** `KdvThemeSnapshot` が必須fieldとして必要になる
- **THEN** `KdvThemeSnapshot` の配色fieldは省略できない
- **THEN** KDVは欠落値を暗黙fallbackで補完しない

#### Scenario: complete theme objectをHTML exportへ反映する

- **WHEN** app/APIが完全テーマを渡してHTML exportを実行する
- **THEN** KDVはHTMLのCSS変数を完全テーマの値から生成する
- **THEN** KDVは固定のlight色を直接使わない

### Requirement: diagram themeはKDV theme modeと一致しなければならない

システムは、diagram exportに使うtheme modeと配色をKDV themeから決定しなければならない（MUST）。

#### Scenario: dark themeでdiagramをexportする

- **WHEN** `katana-dark` の完全テーマでMermaid / Draw.io / PlantUMLをHTML exportする
- **THEN** diagram figureには `data-kdv-diagram-theme="dark"` が含まれる

#### Scenario: app/API themeをKRRへ渡す

- **WHEN** app/APIが独自の完全テーマを渡してdiagramをbuildする
- **THEN** KDVは完全テーマからKRR `RenderThemeSnapshot` を生成する
- **THEN** KDVは `RenderThemeSnapshot` をKRR `RenderContext.theme` として渡す
- **THEN** dark-modeかどうかは `RenderThemeSnapshot.mode` でKRRへ渡される
- **THEN** KDVは欠落したdiagram配色を暗黙fallbackで補完しない

#### Scenario: diagram背景をcode block背景から分離する

- **WHEN** `katana-light` の完全テーマでdiagramをHTML exportする
- **THEN** diagram背景はcode block背景を使わない
- **THEN** diagram背景は `transparent` として扱う
- **THEN** diagramは親要素の `katana-light` 背景へ溶け込む

### Requirement: `--theme` JSON入口は専用仕様で扱わなければならない

システムは、完全テーマJSONを受け取るCLI入口を `cli-theme-json-entry` 仕様で扱い、complete theme contract自体へ混ぜ込んではならない（MUST）。

#### Scenario: 将来のJSON入口名を確認する

- **WHEN** CLIの完全テーマJSON入口を設計する
- **THEN** option名は `--theme` とする
- **THEN** `--thema` は採用しない
