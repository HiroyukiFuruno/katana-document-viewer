use crate::evaluation::{CoverageStatus, EvaluationFeatureCoverage, FixtureCategory};

const FEATURE_COUNT: usize = 33;
const COMMONMARK: FixtureCategory = FixtureCategory::CommonMark;
const GFM: FixtureCategory = FixtureCategory::Gfm;
const MATH: FixtureCategory = FixtureCategory::Math;
const ALERT: FixtureCategory = FixtureCategory::GitHubAlert;
const KATANA: FixtureCategory = FixtureCategory::KatanaCompatibility;
const EXTERNAL: FixtureCategory = FixtureCategory::ExternalRendering;
const DTO: CoverageStatus = CoverageStatus::KmmDto;
const KDR: CoverageStatus = CoverageStatus::KdrDirect;
const ZENUML: CoverageStatus = CoverageStatus::KdrMermaidCompatibility;

type FeatureSeed = (&'static str, &'static str, FixtureCategory, CoverageStatus);

const FEATURES: [FeatureSeed; FEATURE_COUNT] = [
    ("commonmark-heading", "commonmark-blocks", COMMONMARK, DTO),
    ("commonmark-paragraph", "commonmark-blocks", COMMONMARK, DTO),
    (
        "commonmark-blockquote",
        "commonmark-blocks",
        COMMONMARK,
        DTO,
    ),
    ("commonmark-list", "commonmark-blocks", COMMONMARK, DTO),
    (
        "commonmark-code-block",
        "commonmark-blocks",
        COMMONMARK,
        DTO,
    ),
    (
        "commonmark-thematic-break",
        "commonmark-blocks",
        COMMONMARK,
        DTO,
    ),
    (
        "commonmark-emphasis",
        "commonmark-inline-gap",
        COMMONMARK,
        DTO,
    ),
    (
        "commonmark-strong",
        "commonmark-inline-gap",
        COMMONMARK,
        DTO,
    ),
    (
        "commonmark-inline-code",
        "commonmark-inline-gap",
        COMMONMARK,
        DTO,
    ),
    ("commonmark-link", "commonmark-inline-gap", COMMONMARK, DTO),
    ("commonmark-image", "commonmark-inline-gap", COMMONMARK, DTO),
    (
        "commonmark-footnote",
        "commonmark-inline-gap",
        COMMONMARK,
        DTO,
    ),
    ("gfm-table", "gfm-table-task", GFM, DTO),
    ("gfm-task-list", "gfm-table-task", GFM, DTO),
    ("github-alert-note", "gfm-alerts", ALERT, DTO),
    ("github-alert-warning", "gfm-alerts", ALERT, DTO),
    ("math-fenced", "math-mixed", MATH, DTO),
    ("math-inline", "math-mixed", MATH, DTO),
    ("math-dollar-block", "math-mixed", MATH, DTO),
    ("katana-centered-html", "katana-compat", KATANA, DTO),
    ("katana-badge-row", "katana-compat", KATANA, DTO),
    ("katana-legacy-note", "katana-compat", KATANA, DTO),
    ("katana-description-list", "katana-compat", KATANA, DTO),
    ("katana-task-marker", "katana-compat", KATANA, DTO),
    ("katana-drawio-block", "katana-compat", KATANA, DTO),
    ("katana-japanese-text", "katana-compat", KATANA, DTO),
    ("katana-html-entity", "katana-compat", KATANA, DTO),
    ("katana-drawio-file-ref", "katana-compat", KATANA, DTO),
    ("katana-xml-file-ref", "katana-compat", KATANA, DTO),
    ("mermaid-render", "external-success", EXTERNAL, KDR),
    ("drawio-render", "external-success", EXTERNAL, KDR),
    (
        "zenuml-mermaid-compat",
        "external-success",
        EXTERNAL,
        ZENUML,
    ),
    ("plantuml-render", "external-failure", EXTERNAL, KDR),
];

pub(crate) struct V01EvaluationCoverageFactory;

impl V01EvaluationCoverageFactory {
    pub(crate) fn create() -> Vec<EvaluationFeatureCoverage> {
        FEATURES.iter().map(Self::feature).collect()
    }

    fn feature(seed: &FeatureSeed) -> EvaluationFeatureCoverage {
        let (id, fixture_id, category, status) = *seed;
        EvaluationFeatureCoverage {
            id: id.to_string(),
            fixture_id: fixture_id.to_string(),
            category,
            scenario: Self::scenario(category).to_string(),
            status,
        }
    }

    fn scenario(category: FixtureCategory) -> &'static str {
        match category {
            FixtureCategory::CommonMark => "Markdown標準の全記法をKMM入力として評価する",
            FixtureCategory::Gfm => "CommonMark / GFM / KatanA互換fixtureを評価する",
            FixtureCategory::Math => "数式とGitHub alertを評価対象に含める",
            FixtureCategory::GitHubAlert => "数式とGitHub alertを評価対象に含める",
            FixtureCategory::KatanaCompatibility => "KatanA独自解釈を評価対象に含める",
            FixtureCategory::ExternalRendering => "KDR direct renderingの対象を限定する",
        }
    }
}
