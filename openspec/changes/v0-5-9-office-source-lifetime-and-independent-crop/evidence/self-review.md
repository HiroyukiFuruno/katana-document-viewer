# Self Review: v0.5.9候補

## 結論

PASS（候補のコミット/Draft PR準備）。未対応P0/P1は見つからず、分割後のsemver・通常check・strict coverage・package/publish dry-runを含む完全release-check session97206 exit0を確認した。公開・current-HEAD cloud review/3OS CI・元下流受入の完了とは区別する。

## 確認した差分

- `OfficeStaticViewerSession`は変換後の原本をPDF解析前に解放し、conversion identity・metadata・diagnostics・itemsを保持する。
- Officeの高水準`DocumentSession`だけprivate raster workerへ移し、直接PDFと低水準Officeの公開render APIは維持。親のstrict validation/cache/fingerprintを再利用し、worker失敗は既存型付きerrorへ伝播し、隠れたin-process fallbackを追加していない。
- private request128bytes/response64KiB、regular bounded read、request identity・dimension・pixels・RGBA lengthの検証を保持。PDF容量とRGBA容量の誤混同を実consumer RED→GREENで検出し、pixel由来の別上限へ分離した。
- readerはsymlink_metadataで通常fileかを検査してからbounded readする。atomic no-follow/open保証を実装したとは主張しない。workspaceは既存の専用TempDir、child完了後に親が読む既存責務を維持する。
- Linux/macOS constraints、timeout/error、Windows AppContainer/Job memory/kill-on-close/quoteの既存process契約を共通mode経路で再利用。Windows native実行は3OS CIで別途確認する。既存にない独立cancel tokenの保証を追加したとは扱わない。
- self-testのGit環境だけをnative repository-local変数から隔離し、通常のGit/hook委譲は変更しない。実Git metadata保持と例外復元回帰を確認。過去のbare=true発生経路自体の確定とは区別する。
- remote削除はRelease workflowから除き、公開後auditのみへ変更。remote削除の人間による明示なしに削除しない。
- registry KUC0.4.1/KRR0.4.22/office2pdf0.8.0/V8 singletonを使用し、path/git overrideはない。互換・major監査を実施した。
- 外部crop入力はSHA/寸法/frameをfail-closedで照合し、tracked referenceを書き換えない。visualからsemantic/interaction/performanceを推定しない。
- モデル証跡のLuna規則は最新ユーザー方針に整合し、旧Sparkの実行記録を別モデルへ書き換えない。

## 検証結果

通常check、strict coverage functions/lines100%・未カバー0、実Office互換IT、実worker失敗回帰、Git governance/Release契約/ハーネス、外部Office fidelity、同実入力のcold/warm RSSとframe/metadata一致、semver196pass、完全release-check session97206 exit0を確認。詳細・実終了コード・SHAは`final-rgba-limit-validation.md`。macOS debug linkの既存`__eh_frame`警告は残るため、警告なしとは報告しない。検査条件・許可/除外・閾値を変更していない。

## 残す完了条件

全Storybook664pass/0fail、明示crop visual100/95、追加回帰のtest-file分割後の完全release-checkも成功した。current-HEAD review・3OS/preflight・公開・fresh registry consumer・元下流受入・Issue closure・既存stash意味的整理は未完了。OpenSpecの未完了taskを完了へ書き換えず、archiveしない。Issue58の独立三項目と現行artifact採用は公開後の下流証跡と区別して追跡する。

私有scratch、原本、binary、生ログ、ignored AGENTS.mdはstage/packageへ含めない。
