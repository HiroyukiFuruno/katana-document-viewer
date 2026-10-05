# 依存監査

2026-10-05T09:38Z更新: KRR v0.4.23のGitHub Release、PR105通常merge `8654789f69d24401b8ae67036db1cdb3e7f4294e`、annotated tagのcommit、Release run37285317328 attempt2 SUCCESS、実registry取得をmainが独立照合。実runtime crate SHA `8e5e0803f343d781dc015d1fe21b052492d67579ba303cf86203f60455852e00`、assets crate SHA `729af5d660bb6f439ced0a7c56d86ad1cd15b6f1092becc70afd97d41c5a6073`、runtime VCSはmerge一致。未公開overrideなし。

Cargo.tomlのcaret最低版を0.4.23へ更新し、`cargo update -p katana-render-runtime --precise 0.4.23`でlockを採用。旧互換更新を維持し、KRR assets0.4.23/cfg_aliases0.2.2/nix0.31.3を追加。lock SHA `e006339e2a11f4d2214208f6c402b664948bfca845b17858e509d9e502c33224`。`cargo update --dry-run`は追加更新0。実inverse treeのV8は152.2.0一版、KDVとKRR0.4.23が共有。実linkは後続gateで確認する。

旧0.4.22固定consumer checkerを無検証で流用せず、新0.4.23 metadataの許可/旧0.4.22拒否の回帰を先に追加してRED(exit1)を確認。期待版とself-test graphを更新してGREEN(exit0)。release-contractのcaret最低版も0.4.23へ更新し、0.4.22拒否/0.4.24と0.4.99許可/0.5.0・path・重複・不正checksum拒否を保持してself-testと実contract exit0。これらは全件品質/元HTML正常close/元GUI性能の代替ではない。新graphの全gateは3.1で未完、旧lockの成功は新graphへ転用しない。

`cargo upgrade --dry-run --incompatible allow --pinned allow --recursive false` は直接依存36件latest、hayro exact0.7.1→0.8.0だけをmajor候補として提示。`cargo update --dry-run` はglam0.33.12→0.34.0/hayro-jbig2 0.3.0→0.3.1、新fearless_simd1.0.0/hayro-ccitt0.4.0を互換lock候補として提示。更新後の全gateは別途未完。

mainは公開registry hayro0.8.0を cargo info で実取得（MSRV1.92、workspace1.95.0以内）し、実sourceでrenderが4引数→5引数、scaleがRenderSettings→PixmapSettingsへ分離、vello_cpu0.3へのrenderer変更を確認した。readonly監査Aquinasは外部API403/404により公開差分を確認できなかったが、取得不能を採否根拠にせずmainが補完した。

実 cargo tree -i hayro@0.7.1 はKDV直接と公開office2pdf0.8.1→typst0.15.1→typst-html/typst-svgの共有hayro0.7.1を示す。KDVだけ0.8へ変更しても公開Office converter側0.7.1が残り、二つのPDF rasterizer/backendを同じworker graphへ増やす。RSS保持調査中の#59に対して測定済み改善・元PDF/Office fidelity同値の証明がなく、今回のtest-only導線修正でunmeasured renderer migrationは採用しない。major候補は評価済み、互換版・lock更新を検証してから採用する。未公開上流overrideやreference/品質緩和は行わない。

公開前の履歴: KDV0.5.11、KRR0.4.22。KRR PR105はDraft OPEN/newHEAD b03c0cd42fe2aaf9d1a0b128ff530d1d6a9c1d07、当時は公開前採用なし。現在の公開境界・採用graphは冒頭09:38Z更新を正とし、この旧snapshotを待機条件にしない。
