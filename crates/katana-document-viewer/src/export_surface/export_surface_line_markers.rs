use crate::export_surface_line::{
    SurfaceLine, SurfaceLineLevel, SurfaceLineMarker, SurfaceTaskMarker,
};

const HEADING_1_TOP_MARGIN: u32 = 16;
const HEADING_2_TOP_MARGIN: u32 = 14;
const HEADING_DEFAULT_TOP_MARGIN: u32 = 12;
const BODY_TOP_MARGIN: u32 = 5;
const CODE_TOP_MARGIN: u32 = 0;
const COMPACT_BODY_TOP_MARGIN_SCALE_MAX: f32 = 14.0 / 24.0;

impl SurfaceLine {
    pub(crate) fn aligns_with_list_marker(&self) -> bool {
        !self.spans.is_empty() && self.spans[0].text.starts_with('◩')
    }

    pub(crate) fn list_marker(&self) -> Option<SurfaceLineMarker> {
        let marker = self.spans.first()?.text.trim();
        match marker {
            "•" => Some(SurfaceLineMarker::Bullet),
            "☑" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Done)),
            "☐" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Empty)),
            "⊟" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Blocked)),
            "◩" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::InProgress)),
            _ if super::export_surface_line_debug::ordered_marker(marker) => {
                Some(SurfaceLineMarker::Ordered(marker.to_string()))
            }
            _ => None,
        }
    }

    pub(crate) fn content_spans(&self) -> &[crate::export_surface_span::SurfaceTextSpan] {
        if self.list_marker().is_some() {
            return &self.spans[1..];
        }
        &self.spans
    }

    pub(super) fn top_margin(&self) -> u32 {
        if matches!(self.level, SurfaceLineLevel::Body)
            && self.font_scale() <= COMPACT_BODY_TOP_MARGIN_SCALE_MAX
        {
            return 0;
        }
        self.scale_dimension(match self.level {
            SurfaceLineLevel::Heading(1) => HEADING_1_TOP_MARGIN,
            SurfaceLineLevel::Heading(2) => HEADING_2_TOP_MARGIN,
            SurfaceLineLevel::Heading(_) => HEADING_DEFAULT_TOP_MARGIN,
            SurfaceLineLevel::Body => BODY_TOP_MARGIN,
            SurfaceLineLevel::Code => CODE_TOP_MARGIN,
        })
    }
}

#[cfg(test)]
#[path = "export_surface_line_markers_tests.rs"]
mod tests;
