## MODIFIED Requirements

### Requirement: export requestから成果物を生成しなければならない

システムは、ホストから渡された `ExportRequest` と `ExportFormat` から `ExportOutput` を生成しなければならない（MUST）。

#### Scenario: HTML / PDF / PNG / JPEGを書き出す

- **GIVEN** `BuildGraph` が作成済みである
- **WHEN** ホストがcomplete theme objectを含む `ExportRequest` と `ExportFormat` を渡す
- **THEN** KDVは `BuildGraph` から `ExportOutput` を生成する
- **THEN** `ArtifactManifest` はformat、backend、source revisionを含む
- **THEN** `ArtifactBytes` は空ではない
- **THEN** HTML exportはcomplete theme objectからCSS変数を生成する
