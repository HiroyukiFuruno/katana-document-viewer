use super::{
    BADGE_HEIGHT, BADGE_HORIZONTAL_GAP, BADGE_VERTICAL_MARGIN, LIST_MARKER_COLUMN_WIDTH,
    SurfaceBadgeRowBlock, SurfaceBlock, SurfaceLine, SurfacePainter, SurfaceSpanMetadataRequest,
    SurfaceSpanMetrics, SurfaceTextSpan,
};

impl SurfacePainter {
    pub(super) fn append_link_metadata(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        anchors: &mut Vec<super::SurfaceLinkAnchor>,
        block: &SurfaceBlock,
        page_index: usize,
        y: u32,
    ) {
        match block {
            SurfaceBlock::Line(line) => {
                Self::append_line_link_metadata(annotations, anchors, line, page_index, y)
            }
            SurfaceBlock::BadgeRow(row) => {
                Self::append_badge_link_annotations(annotations, row, page_index, y)
            }
            _ => {}
        }
    }

    pub(super) fn append_line_link_metadata(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        anchors: &mut Vec<super::SurfaceLinkAnchor>,
        line: &SurfaceLine,
        page_index: usize,
        y: u32,
    ) {
        let text_y = line.text_y(y);
        let mut x = Self::line_text_x(line);
        let spans = Self::line_link_target_spans(line, &mut x);
        let font_size = line.font_size();
        for span in spans {
            Self::append_line_span_metadata(
                annotations,
                anchors,
                SurfaceSpanMetadataRequest {
                    span,
                    font_size,
                    line_height: line.line_height(),
                    page_index,
                    text_y,
                },
                &mut x,
            );
        }
    }

    pub(super) fn line_link_target_spans<'a>(
        line: &'a SurfaceLine,
        x: &mut u32,
    ) -> &'a [SurfaceTextSpan] {
        if line.list_marker().is_some() {
            *x += LIST_MARKER_COLUMN_WIDTH;
            return line.content_spans();
        }
        &line.spans
    }

    pub(super) fn append_line_span_metadata(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        anchors: &mut Vec<super::SurfaceLinkAnchor>,
        request: SurfaceSpanMetadataRequest<'_>,
        x: &mut u32,
    ) {
        let span_width = SurfaceSpanMetrics::estimated_width(request.span, request.font_size);
        let Some(target) = &request.span.link_target else {
            *x += span_width;
            return;
        };
        if target.is_empty() {
            *x += span_width;
            return;
        }
        annotations.push(super::SurfaceLinkAnnotation {
            page_index: request.page_index,
            x: *x,
            y: request.text_y,
            width: span_width,
            height: request.line_height,
            target: target.clone(),
        });
        if let Some(anchor) =
            inferred_link_anchor(request.span, request.page_index, *x, request.text_y)
        {
            anchors.push(anchor);
        }
        *x += span_width;
    }

    pub(super) fn append_badge_link_annotations(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        row: &SurfaceBadgeRowBlock,
        page_index: usize,
        y: u32,
    ) {
        let mut x = Self::badge_row_start_x(row);
        for badge in row.badges() {
            if let Some(target) = &badge.link_target {
                annotations.push(super::SurfaceLinkAnnotation {
                    page_index,
                    x,
                    y: y + BADGE_VERTICAL_MARGIN,
                    width: badge.width(),
                    height: BADGE_HEIGHT,
                    target: target.clone(),
                });
            }
            x += badge.width() + BADGE_HORIZONTAL_GAP;
        }
    }
}

fn inferred_link_anchor(
    span: &SurfaceTextSpan,
    page_index: usize,
    x: u32,
    y: u32,
) -> Option<super::SurfaceLinkAnchor> {
    let target = span.link_target.as_deref()?;
    if let Some(label) = target.strip_prefix("#fn-") {
        return Some(super::SurfaceLinkAnchor {
            id: format!("fnref-{label}"),
            page_index,
            x,
            y,
        });
    }
    if let Some(label) = target.strip_prefix("#fnref-") {
        return Some(super::SurfaceLinkAnchor {
            id: format!("fn-{label}"),
            page_index,
            x,
            y,
        });
    }
    None
}
