# ビルド生成物の容量管理

通常の `just` のbuild/test/lint/coverage/package/publishは、開始前にこのリポジトリのtarget使用容量を確認する。直近の約20GiBのビルドを2回分保持できる目安として、初期閾値は40GiBとする。閾値以下では削除しない。

`KDV_BUILD_CACHE_MAX_GIB` で閾値を変更できる。Unixでは実際の割当容量を計測し、ハードリンクを重複計上しない。`CARGO_TARGET_DIR` とCargoの `--target-dir DIRECTORY` / `--target-dir=DIRECTORY` の別profileも、このリポジトリ内だけを集計する。directory名は任意で、リポジトリ外へのsymlink、Git管理領域、tracked sourceやcrate manifestを含むdirectoryは拒否する。

ネストしたprofileも個別のCargo所有タグを維持する。子だけがCargo targetで、親にタグがない場合は、親全体を整理対象に含めない。重なるprofileの同一ファイルは集計全体で一度だけ数える。

超過した場合は、Cargoの所有識別タグと使用中の生成物がないことを確認して、標準の `cargo clean` を実行する。開始前の整理からコマンド終了まで排他lockを保持する。使用中・所有確認不可・検査エラーの場合は生成物を消さず、理由を返す。別リポジトリやsourceは整理しない。

新規または空の出力directoryだけは、source保護を確認して所有タグを初期化する。CIもcache復元より前に空のtargetを初期化する。Cargoは先に存在するdirectoryへタグを生成しない場合があるため、この順序で所有情報を保持する。既存の非空directoryにタグがなければ、勝手に所有対象へ変更しない。

実装は `scripts/maintenance/cargo-guard` と `guard-build-cache.py`。既存の `CARGO` 設定の明示的な上書きは維持する。別checkoutへ移動する任意のKUC検査導線は通常のCargoを使い、KDVの容量guardを適用しない。設定を上書きしない通常の導線は自動でこのguardを使用する。

`just build-cache-script-test` は閾値境界・所有範囲・使用中の保護・Windows検査・実際のCargo整理・コマンド終了までの排他を検証し、既存の `just check` に含める。
