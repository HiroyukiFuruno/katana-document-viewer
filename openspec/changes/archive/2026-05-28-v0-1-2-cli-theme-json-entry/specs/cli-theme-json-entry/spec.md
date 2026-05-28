## ADDED Requirements

### Requirement: CLIは完全テーマJSONを `--theme` で受け取らなければならない

システムは、将来のCLI入口で完全テーマJSONを `--theme` optionから受け取らなければならない（MUST）。

#### Scenario: 完全テーマJSONを指定する

- **WHEN** ユーザーが `--theme <json>` を指定する
- **THEN** KDVはJSONを `KdvThemeSnapshot` として読む
- **THEN** 欠落fieldがある場合はexportを開始しない

#### Scenario: `--thema` は受け付けない

- **WHEN** ユーザーが `--thema <json>` を指定する
- **THEN** KDVは未知optionとして拒否する

### Requirement: JSON themeは部分指定を許可してはならない

システムは、CLI JSON themeで部分指定や暗黙fallbackを許可してはならない（MUST NOT）。

#### Scenario: 部分JSONを指定する

- **WHEN** JSONに必要な配色fieldが不足している
- **THEN** KDVはthemeを補完しない
- **THEN** エラーとして扱う
