# ビルド生成物の容量管理

通常の `just` のbuild/test/lint/coverage/package/publishは、開始前にこのリポジトリのtarget使用容量を確認する。直近の約20GiBのビルドを2回分保持できる目安として、初期閾値は40GiBとする。閾値以下では削除しない。

POSIXの使用中検査には `lsof` が必須。Ubuntu/Debianの最小環境では、初回build/clean前に `sudo apt-get update` と `sudo apt-get install -y lsof` で準備する。Linux CIの共通依存インストールにも含めている。macOSは標準の `/usr/sbin/lsof` を使えるようPATHへ `/usr/sbin` を含め、`command -v lsof` で確認する。Windowsは標準の `tasklist` を使う。検査ツールがない場合は導入要求を表示して整理を拒否し、使用中検査を省略しない。

`KDV_BUILD_CACHE_MAX_GIB` で閾値を変更できる。Unixでは実際の割当容量を計測し、ハードリンクを重複計上しない。`CARGO_TARGET_DIR` とCargoの `--target-dir DIRECTORY` / `--target-dir=DIRECTORY` の別profileも、このリポジトリ内だけを集計する。directory名は任意で、リポジトリ外へのsymlink、Git管理領域、tracked sourceやcrate manifestを含むdirectoryは拒否する。

通常build/testで明示 `--target-dir` がない場合は、階層configや `--config` を引き継いだCargo metadataから実効出力先も取得する。相対パスと設定の優先順位はCargo自身に解決させる。

ネストしたprofileも個別のCargo所有タグを維持する。子だけがCargo targetで、親にタグがない場合は、親全体を整理対象に含めない。重なるprofileの同一ファイルは集計全体で一度だけ数える。

超過した場合は、Cargoの所有識別タグと使用中の生成物がないことを確認して、所有directoryの内容を整理する。Unixは保持FD相対、Windowsはvolume rootから保持HANDLE相対で祖先・対象・子を開き、HANDLEで列挙・削除し、検証後のpathname差替えで別directoryを消さない。target自身と所有タグは残す。開始前の整理からコマンド終了まで排他lockを保持する。所有確認不可・使用中は削除前に拒否し、整理中のエラーも成功として隠さない。別リポジトリやsourceは整理しない。

Windowsでは、属性専用の既存HANDLEからのreparse変更は共有制御だけでは拒否できない。子の列挙とopenも保持HANDLEを基準にし、一成分名・leaf no-followで外部へ遷移しない。祖先・target・所有タグのreparse化を検出した場合は整理を拒否する。検査・削除中の予期しないOSエラーも呼出元へ返す。

明示的な `just clean` も同じlockと所有・使用中検査を通し、閾値に関係なく同じ実体固定の全体整理を行う。guard経由のコマンドが実行中なら待たずに整理を拒否し、終了コード3を返す。`clean -p`・profile・doc等の選択的整理と `sweep` は、Cargo側のpathname削除を検証済み実体へ束縛できないため、終了コード2で拒否する。選択的整理を全体削除へ読み替えない。`--dry-run`・helpは削除を実行せず元コマンドへ委譲する。安全確認の失敗や整理の非0終了も呼出元へ返す。手動の未保護Cargoに対する使用中検査はプロセス・open fileの観測に基づくため、通常のbuild/testもguard経由で実行する。

明示的な整理は、Cargoの `--target-dir`、なければ `CARGO_TARGET_DIR` を対象にする。どちらもない場合は、呼び出し元のCargo作業場所・workspace・設定を引き継ぐCargo metadataで実効出力先を解決する。`--repo-root` は整理を許す範囲であり、Cargoの作業場所を変更しない。解決後もrepo内の所有範囲検査を維持し、無関係な既定targetを併合しない。設定を確認・解決できなければ削除を拒否する。manifest指定、`-C` / `--config` / toolchain等のglobal指定、重複target指定、guard専用のtarget指定は、整理対象を安全に証明できないため削除前に拒否する。help/dry-runではtargetや所有タグも初期化しない。

新規または空の出力directoryだけは、source保護を確認して所有タグを初期化する。CIもcache復元より前に空のtargetを初期化する。Cargoは先に存在するdirectoryへタグを生成しない場合があるため、この順序で所有情報を保持する。既存の非空directoryにタグがなければ、勝手に所有対象へ変更しない。

coverage開始前の `coverage-v8-refresh` は、復元済みV8 link artifactの再利用を防ぐため、`target/llvm-cov-target` 全体を明示的に刷新する。親の通常targetと両方の所有タグは保持する。coverage専用生成物の再生成範囲は広がるが、関数・行100%の検査条件は変えない。

実装は `scripts/maintenance/cargo-guard` と `guard-build-cache.py`。既存の `CARGO` 設定の明示的な上書きは維持する。別checkoutへ移動する任意のKUC検査導線は通常のCargoを使い、KDVの容量guardを適用しない。設定を上書きしない通常の導線は自動でこのguardを使用する。

`just build-cache-script-test` は閾値境界・所有範囲・使用中の保護・Windows検査・実際のCargo整理・コマンド終了までの排他を検証し、既存の `just check` に含める。

通常のbuild/testでCargoの `-C DIRECTORY` / `-CDIRECTORY` がある場合、相対 `--target-dir` と `CARGO_TARGET_DIR` は変更後のCargo作業場所を基準に計測する。解決後もこのrepo内の所有範囲検査を維持し、repo外のtargetは拒否する。
