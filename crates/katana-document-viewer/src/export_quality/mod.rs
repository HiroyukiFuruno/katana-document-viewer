mod html_score;
mod html_score_svg_media;
mod image_score;
mod markdown_image_source;
mod markdown_link_source;
mod score;
mod surface_equivalence;
mod types;
mod visual_content_stats;

#[cfg(test)]
mod markdown_image_source_tests;
#[cfg(test)]
mod markdown_link_source_tests;

pub use score::ExportQualityGate;
pub use surface_equivalence::{
    SurfaceEquivalenceArtifacts, SurfaceEquivalenceGate, SurfaceEquivalenceImage,
    SurfaceEquivalenceReport,
};
pub use types::{
    ExportFormatQualityScore, ExportQualityArtifacts, ExportQualityCheck, ExportQualityReport,
};
