use serde::{Deserialize, Serialize};

#[path = "evaluation_coverage.rs"]
mod evaluation_coverage;
#[path = "evaluation_fixtures.rs"]
mod evaluation_fixtures;

use evaluation_coverage::V01EvaluationCoverageFactory;
use evaluation_fixtures::V01EvaluationFixtureFactory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FixtureCategory {
    CommonMark,
    Gfm,
    Math,
    GitHubAlert,
    KatanaCompatibility,
    ExternalRendering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoverageStatus {
    KmmDto,
    KrrDirect,
    KrrMermaidCompatibility,
    KdvExportContract,
    MissingImplementation,
    ExternalBackendRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationFixture {
    pub id: String,
    pub path: String,
    pub category: FixtureCategory,
    pub scenario: String,
    pub coverage: CoverageStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationFixtureMatrix {
    pub fixtures: Vec<EvaluationFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationFeatureCoverage {
    pub id: String,
    pub fixture_id: String,
    pub category: FixtureCategory,
    pub scenario: String,
    pub status: CoverageStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationCoverageMatrix {
    pub features: Vec<EvaluationFeatureCoverage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendCapability {
    KrrDirect,
    KrrMermaidCompatibility,
    KdvManifestExport,
    ExternalBackendRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCapabilityMatrix {
    pub capabilities: Vec<BackendCapabilityEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCapabilityEntry {
    pub feature: String,
    pub capability: BackendCapability,
    pub backend: String,
}

impl EvaluationFixtureMatrix {
    pub fn v0_1() -> Self {
        Self {
            fixtures: V01EvaluationFixtureFactory::create(),
        }
    }

    pub fn status_count(&self, status: CoverageStatus) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.coverage == status)
            .count()
    }

    pub fn contains_category(&self, category: FixtureCategory) -> bool {
        self.fixtures
            .iter()
            .any(|fixture| fixture.category == category)
    }
}

impl EvaluationCoverageMatrix {
    pub fn v0_1() -> Self {
        Self {
            features: V01EvaluationCoverageFactory::create(),
        }
    }

    pub fn status_count(&self, status: CoverageStatus) -> usize {
        self.features
            .iter()
            .filter(|feature| feature.status == status)
            .count()
    }

    pub fn is_complete(&self) -> bool {
        self.status_count(CoverageStatus::MissingImplementation) == 0
            && self.status_count(CoverageStatus::ExternalBackendRequired) == 0
    }

    pub fn is_kdv_owned_complete(&self) -> bool {
        self.status_count(CoverageStatus::MissingImplementation) == 0
    }

    pub fn contains_feature(&self, id: &str, status: CoverageStatus) -> bool {
        self.features
            .iter()
            .any(|feature| feature.id == id && feature.status == status)
    }
}

impl BackendCapabilityMatrix {
    pub fn v0_1() -> Self {
        Self {
            capabilities: vec![
                capability("mermaid", BackendCapability::KrrDirect, "krr"),
                capability("drawio", BackendCapability::KrrDirect, "krr"),
                capability("zenuml", BackendCapability::KrrMermaidCompatibility, "krr"),
                capability("plantuml", BackendCapability::KrrDirect, "krr"),
                capability("html-export", BackendCapability::KdvManifestExport, "kdv"),
                capability("pdf-export", BackendCapability::KdvManifestExport, "kdv"),
                capability("png-export", BackendCapability::KdvManifestExport, "kdv"),
                capability("jpeg-export", BackendCapability::KdvManifestExport, "kdv"),
                capability("math", BackendCapability::KdvManifestExport, "kdv"),
            ],
        }
    }
}

fn capability(
    feature: &'static str,
    capability: BackendCapability,
    backend: &'static str,
) -> BackendCapabilityEntry {
    BackendCapabilityEntry {
        feature: feature.to_string(),
        capability,
        backend: backend.to_string(),
    }
}

#[cfg(test)]
#[path = "evaluation_kmm_tests.rs"]
mod kmm_tests;

#[cfg(test)]
#[path = "evaluation_tests.rs"]
mod tests;
