use serde::{Deserialize, Serialize};

use crate::export_contract_entries::{ENTRIES, EntrySeed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HtmlExportReadiness {
    Implemented,
    RequiresKmmDto,
    RequiresKdvImplementation,
    RequiresKdrRender,
    ExternalBackendRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlExportContractEntry {
    pub feature: String,
    pub notation: String,
    pub current_state: String,
    pub readiness: HtmlExportReadiness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlExportContractMatrix {
    pub entries: Vec<HtmlExportContractEntry>,
}

impl HtmlExportContractMatrix {
    pub fn v0_1() -> Self {
        Self {
            entries: ENTRIES.iter().map(Self::entry).collect(),
        }
    }

    pub fn incomplete_entries(&self) -> Vec<&HtmlExportContractEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.readiness != HtmlExportReadiness::Implemented)
            .collect()
    }

    pub fn kdv_owned_incomplete_entries(&self) -> Vec<&HtmlExportContractEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.readiness == HtmlExportReadiness::RequiresKdvImplementation)
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        self.incomplete_entries().is_empty()
    }

    pub fn is_kdv_owned_complete(&self) -> bool {
        self.kdv_owned_incomplete_entries().is_empty()
    }

    pub fn contains_feature(&self, feature: &str, readiness: HtmlExportReadiness) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.feature == feature && entry.readiness == readiness)
    }

    fn entry(seed: &EntrySeed) -> HtmlExportContractEntry {
        let (feature, notation, current_state, readiness) = *seed;
        HtmlExportContractEntry {
            feature: feature.to_string(),
            notation: notation.to_string(),
            current_state: current_state.to_string(),
            readiness,
        }
    }
}

#[cfg(test)]
#[path = "export_contract_tests.rs"]
mod tests;
