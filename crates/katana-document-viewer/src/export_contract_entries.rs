use crate::HtmlExportReadiness;

pub(crate) type EntrySeed = (
    &'static str,
    &'static str,
    &'static str,
    HtmlExportReadiness,
);

const IMPLEMENTED: HtmlExportReadiness = HtmlExportReadiness::Implemented;
const KDR_RENDER: HtmlExportReadiness = HtmlExportReadiness::RequiresKdrRender;

pub(crate) const ENTRIES: [EntrySeed; 31] = [
    (
        "commonmark-heading",
        "# heading",
        "KMM DTOからh1-h6へ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-paragraph",
        "paragraph",
        "KMM inline childrenをpへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-strong",
        "**text**",
        "KMM DTOからstrongへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-emphasis",
        "*text*",
        "KMM DTOからemへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-inline-code",
        "`code`",
        "KMM DTOからinline codeへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-link",
        "[text](url)",
        "KMM DTOからanchorへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-autolink",
        "<https://example.com>",
        "KMM DTOからautolink anchorへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-image",
        "![alt](src)",
        "KMM DTOからimgへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-footnote",
        "[^1]",
        "KMM DTOからfootnoteへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-nested-list",
        "- item",
        "KMM DTOからnested listへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-code-block",
        "```rust",
        "fenceを除いたpre/codeへ出力できる",
        IMPLEMENTED,
    ),
    (
        "commonmark-blockquote-children",
        "> block",
        "KMM DTOからblockquote childrenへ出力できる",
        IMPLEMENTED,
    ),
    (
        "gfm-strikethrough",
        "~~text~~",
        "KMM DTOからstrikethroughへ出力できる",
        IMPLEMENTED,
    ),
    (
        "gfm-table",
        "| h |",
        "table DTOからthead/tbodyへ出力できる",
        IMPLEMENTED,
    ),
    (
        "gfm-task-list",
        "- [x] item",
        "KMM DTOからtask listへ出力できる",
        IMPLEMENTED,
    ),
    (
        "github-alert",
        "> [!NOTE]",
        "label DTOとchildrenからasideへ出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-legacy-note",
        "> **Note**",
        "label DTOとchildrenからasideへ出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-inline-html",
        "<u>/<mark>",
        "KMM DTOからinline HTMLへ出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-centered-html",
        "<p align=\"center\">",
        "HTML block passthroughで出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-badge-row",
        "<a><img>",
        "HTML block passthroughで出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-data-uri-svg",
        "data:image/svg+xml",
        "KatanA fixtureのdata URI SVGを正規化して出力できる",
        IMPLEMENTED,
    ),
    (
        "katana-html-entity",
        "&amp;",
        "KMM textを1回decodeしてHTMLへescapeできる",
        IMPLEMENTED,
    ),
    (
        "math-fenced",
        "```math",
        "MathJaxでSVGへ出力できる",
        IMPLEMENTED,
    ),
    (
        "math-inline",
        "$ E = mc^2 $",
        "MathJaxでSVGへ出力できる",
        IMPLEMENTED,
    ),
    (
        "math-dollar-block",
        "$$ ... $$",
        "MathJaxでSVGへ出力できる",
        IMPLEMENTED,
    ),
    (
        "kdr-mermaid",
        "```mermaid",
        "KDR SVG結果をfigureへ埋め込める",
        IMPLEMENTED,
    ),
    (
        "kdr-drawio",
        "```drawio",
        "KDR SVG結果をfigureへ埋め込める",
        IMPLEMENTED,
    ),
    (
        "kdr-zenuml",
        "zenuml",
        "Mermaid互換入力として接続が必要",
        KDR_RENDER,
    ),
    (
        "kdr-plantuml",
        "```plantuml",
        "KDR v0.2.0のPlantUML rendererへ接続できる",
        IMPLEMENTED,
    ),
    (
        "export-pdf",
        "pdf",
        "BuildGraphからRust描画surfaceを作りPDFへ埋め込める",
        IMPLEMENTED,
    ),
    (
        "export-png-jpeg",
        "png/jpeg",
        "BuildGraphからRust描画surfaceを作りPNG/JPEGへ出力できる",
        IMPLEMENTED,
    ),
];
