# Native fixture 結合の検証

標準 RUSTFLAGS=-D warnings の旧 helper で新Typography回帰を実行し、`manifest fixture does not match the scorer fixture`、1FAIL/exit101を確認。修正後はnative6件PASS/0FAIL/exit0で、Typography/Diagrams正常decode・両方向のfixture mismatch・hash更新済みheader-only/CRC/zlib破損拒否・入力非変更を検証した。既存 threshold95/reference/geometry は非変更。

Python self-test はTypography側の対称CLI回帰を追加して28PASS/exit0。Justfile新専用recipeのdry-runは唯一のsample.md native testを --exact/--ignored で指定する。

raw SHA: RED a9d8810ed8012e3ae31ab247883a1f5b2848cdef594bd8ba360f606a26dcbd3d、GREEN a4cd86d1f58587071ed7e1bce889ba49a46c82c2606f4f135227302c26457589。

最初の標準RUSTFLAGSを付け忘れた診断buildは重複再生成を避けてowned cargoだけ中断、exit130。この中断をRED/PASSへ転用しない。上記RED→GREENは両方標準設定で別途終端を確認した。

readonly監査Ramanの既存Python両PNGdecode/expected-fixture回帰/専用Justfileの指摘をmainが実sourceへ照合して反映した。source generator書き換え・private入力公開・新非visual算式なし。current実crop/declaration不足と4分類の元DoDは未完のまま。
