mod architecture;
mod attributes;
mod file_length;
mod function_length;
mod kdv_ui_adapter_ownership;
mod kuc_core_boundary;
mod lazy_code;
mod length_baseline;
mod manifest_boundary;
mod method_calls;
mod nesting_depth;
mod openspec_integrity;
mod pub_free_fn;
mod rendering_contract;
mod storybook_contract;
#[cfg(test)]
pub(crate) mod test_helpers;
mod vendor_boundary;
mod vendor_boundary_manifest;
mod vendor_boundary_source;
mod viewer_media_action_prefix;

use crate::diagnostics::{KdvLintError, Violation};
use crate::workspace::WorkspaceModel;
use architecture::ArchitectureRule;
use attributes::ProhibitedAttributeRule;
use file_length::FileLengthRule;
use function_length::FunctionLengthRule;
use kdv_ui_adapter_ownership::KdvUiAdapterOwnershipRule;
use kuc_core_boundary::KucCoreBoundaryRule;
use lazy_code::LazyCodeRule;
use length_baseline::LengthBaseline;
use method_calls::ProhibitedMethodRule;
use nesting_depth::NestingDepthRule;
use openspec_integrity::OpenSpecIntegrityRule;
use pub_free_fn::PublicFreeFunctionRule;
use rendering_contract::RenderingContractRule;
use storybook_contract::StorybookContractRule;
use vendor_boundary::VendorBoundaryRule;
use viewer_media_action_prefix::ViewerMediaActionPrefixRule;

type RuleCheck = fn(&WorkspaceModel) -> Result<Vec<Violation>, KdvLintError>;

const RULE_CHECKS: &[RuleCheck] = &[
    FileLengthRule::check,
    FunctionLengthRule::check,
    NestingDepthRule::check,
    PublicFreeFunctionRule::check,
    ProhibitedMethodRule::check,
    LazyCodeRule::check,
    ProhibitedAttributeRule::check,
    ArchitectureRule::check,
    KdvUiAdapterOwnershipRule::check,
    RenderingContractRule::check,
    KucCoreBoundaryRule::check,
    StorybookContractRule::check,
    ViewerMediaActionPrefixRule::check,
    VendorBoundaryRule::check,
    |workspace| OpenSpecIntegrityRule::check(workspace.root()),
];

pub struct RuleRunner;

impl RuleRunner {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdvLintError> {
        let mut violations = Vec::new();
        for rule in Self::rules() {
            violations.extend(rule(workspace)?);
        }
        violations.retain(|violation| !LengthBaseline::contains(workspace.root(), violation));
        Ok(violations)
    }

    fn rules() -> &'static [RuleCheck] {
        RULE_CHECKS
    }
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
