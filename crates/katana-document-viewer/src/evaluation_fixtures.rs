use crate::evaluation::{CoverageStatus, EvaluationFixture, FixtureCategory};

const COMMONMARK_FIXTURE: &str = "fixtures/rendering/commonmark.md";
const GFM_FIXTURE: &str = "fixtures/rendering/gfm.md";
const COMMONMARK_FIXTURE_COUNT: usize = 2;
const GFM_FIXTURE_COUNT: usize = 3;
const COMPATIBILITY_FIXTURE_COUNT: usize = 1;
const EXTERNAL_RENDERING_FIXTURE_COUNT: usize = 2;

pub(crate) struct V01EvaluationFixtureFactory;

impl V01EvaluationFixtureFactory {
    pub(crate) fn create() -> Vec<EvaluationFixture> {
        let mut fixtures = Vec::new();
        fixtures.extend(Self::commonmark());
        fixtures.extend(Self::gfm());
        fixtures.extend(Self::compatibility());
        fixtures.extend(Self::external_rendering());
        fixtures
    }

    fn commonmark() -> [EvaluationFixture; COMMONMARK_FIXTURE_COUNT] {
        [
            Self::fixture(
                "commonmark-blocks",
                COMMONMARK_FIXTURE,
                FixtureCategory::CommonMark,
                "Markdown標準の全記法をKMM入力として評価する",
                CoverageStatus::KmmDto,
            ),
            Self::fixture(
                "commonmark-inline-gap",
                COMMONMARK_FIXTURE,
                FixtureCategory::CommonMark,
                "KMM coverage gapを補完parseしない",
                CoverageStatus::MissingImplementation,
            ),
        ]
    }

    fn gfm() -> [EvaluationFixture; GFM_FIXTURE_COUNT] {
        [
            Self::fixture(
                "gfm-table-task",
                GFM_FIXTURE,
                FixtureCategory::Gfm,
                "CommonMark / GFM / KatanA互換fixtureを評価する",
                CoverageStatus::KmmDto,
            ),
            Self::fixture(
                "gfm-alerts",
                GFM_FIXTURE,
                FixtureCategory::GitHubAlert,
                "数式とGitHub alertを評価対象に含める",
                CoverageStatus::KmmDto,
            ),
            Self::fixture(
                "math-mixed",
                "fixtures/rendering/math.md",
                FixtureCategory::Math,
                "数式とGitHub alertを評価対象に含める",
                CoverageStatus::MissingImplementation,
            ),
        ]
    }

    fn compatibility() -> [EvaluationFixture; COMPATIBILITY_FIXTURE_COUNT] {
        [Self::fixture(
            "katana-compat",
            "fixtures/rendering/katana-compat.md",
            FixtureCategory::KatanaCompatibility,
            "KatanA独自解釈を評価対象に含める",
            CoverageStatus::KmmDto,
        )]
    }

    fn external_rendering() -> [EvaluationFixture; EXTERNAL_RENDERING_FIXTURE_COUNT] {
        [
            Self::fixture(
                "external-success",
                "fixtures/rendering/external-success.md",
                FixtureCategory::ExternalRendering,
                "外部描画失敗時にsourceを失わない",
                CoverageStatus::KmmDto,
            ),
            Self::fixture(
                "external-failure",
                "fixtures/rendering/external-failure.md",
                FixtureCategory::ExternalRendering,
                "KRRのPlantUML direct renderingを評価する",
                CoverageStatus::KrrDirect,
            ),
        ]
    }

    fn fixture(
        id: &'static str,
        path: &'static str,
        category: FixtureCategory,
        scenario: &'static str,
        coverage: CoverageStatus,
    ) -> EvaluationFixture {
        EvaluationFixture {
            id: id.to_string(),
            path: path.to_string(),
            category,
            scenario: scenario.to_string(),
            coverage,
        }
    }
}
