const SCORE_THRESHOLD: u8 = 95;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StorybookScoreReport {
    pub(crate) visual_score: u8,
    pub(crate) semantic_score: u8,
    pub(crate) interaction_score: u8,
    pub(crate) performance_score: u8,
    pub(crate) evidence: StorybookScoreEvidence,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct StorybookScoreEvidence {
    pub(crate) visual_katana_reference: bool,
    pub(crate) semantic_export_reference: bool,
    pub(crate) interaction_runtime_actions: bool,
    pub(crate) interaction_os_clipboard: bool,
    pub(crate) performance_budget_gate: bool,
}

impl StorybookScoreReport {
    pub(crate) fn final_score(&self) -> u8 {
        [
            self.visual_score,
            self.semantic_score,
            self.interaction_score,
            self.performance_score,
        ]
        .into_iter()
        .min()
        .unwrap_or(0)
    }

    pub(crate) fn is_pass(&self) -> bool {
        self.final_score() >= SCORE_THRESHOLD && self.missing_evidence().is_empty()
    }

    pub(crate) fn missing_evidence(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.evidence.visual_katana_reference {
            missing.push("visual:katana-reference");
        }
        if !self.evidence.semantic_export_reference {
            missing.push("semantic:export-reference");
        }
        if !self.evidence.interaction_runtime_actions {
            missing.push("interaction:runtime-actions");
        }
        if !self.evidence.interaction_os_clipboard {
            missing.push("interaction:os-clipboard");
        }
        if !self.evidence.performance_budget_gate {
            missing.push("performance:budget-gate");
        }
        missing
    }
}

#[cfg(test)]
#[path = "storybook_score_gate_tests.rs"]
mod tests;
