use crate::export_surface_line::SurfaceLine;

#[path = "export_surface_blocks_badge_alert.rs"]
mod badge_alert;
#[path = "export_surface_blocks_data_image.rs"]
mod data_image;
#[path = "export_surface_blocks_media.rs"]
mod media;
#[path = "export_surface_blocks_table.rs"]
mod table;

pub(crate) use self::badge_alert::{SurfaceAlertBlock, SurfaceBadge, SurfaceBadgeRowBlock};
pub(crate) use self::media::{
    SurfaceCodeBlock, SurfaceDiagramBlock, SurfaceImageBlock, SurfaceMathBlock, SurfaceSpanMetrics,
};
pub(crate) use self::table::{SurfaceTableBlock, SurfaceTableCellPaint, SurfaceTableLayout};

const CODE_VERTICAL_PADDING: u32 = 6;
const CODE_BLOCK_MARGIN: u32 = 14;
const DIAGRAM_MAX_WIDTH: u32 = 860;
const DIAGRAM_VERTICAL_MARGIN: u32 = 18;
const TABLE_ROW_HEIGHT: u32 = 52;
const TABLE_LINE_HEIGHT: u32 = 34;
const TABLE_CELL_PADDING: u32 = 16;
const TABLE_ROW_VERTICAL_PADDING: u32 = 16;
const RULE_HEIGHT: u32 = 34;
const BADGE_HEIGHT: u32 = 26;
const BADGE_VERTICAL_MARGIN: u32 = 10;
const BADGE_HORIZONTAL_GAP: u32 = 10;
const BADGE_HORIZONTAL_PADDING: u32 = 12;
const CODE_EMPTY_BLOCK_MIN_HEIGHT: u32 = 56;
const BADGE_SEGMENT_MIN_WIDTH: u32 = 38;
const DIAGRAM_FALLBACK_HEIGHT: u32 = 38;
const MATH_VERTICAL_MARGIN: u32 = 18;
const MATH_FALLBACK_HEIGHT: u32 = 74;

pub(crate) enum SurfaceBlock {
    Line(SurfaceLine),
    Code(SurfaceCodeBlock),
    Math(SurfaceMathBlock),
    Table(SurfaceTableBlock),
    Diagram(SurfaceDiagramBlock),
    Image(SurfaceImageBlock),
    BadgeRow(SurfaceBadgeRowBlock),
    Alert(SurfaceAlertBlock),
    Rule,
}

impl SurfaceBlock {
    pub(crate) fn height(&self) -> u32 {
        match self {
            SurfaceBlock::Line(line) => line.line_height(),
            SurfaceBlock::Code(code) => code.height(),
            SurfaceBlock::Math(math) => math.height(),
            SurfaceBlock::Table(table) => table.height(),
            SurfaceBlock::Diagram(diagram) => diagram.height(),
            SurfaceBlock::Image(local_image) => local_image.height(),
            SurfaceBlock::BadgeRow(row) => row.height(),
            SurfaceBlock::Alert(alert) => alert.height(),
            SurfaceBlock::Rule => RULE_HEIGHT,
        }
    }

    pub(crate) fn is_heading(&self) -> bool {
        matches!(self, SurfaceBlock::Line(line) if line.is_heading())
    }

    #[cfg(test)]
    pub(crate) fn text_for_tests(&self) -> String {
        match self {
            SurfaceBlock::Line(line) => line.text.clone(),
            SurfaceBlock::Code(code) => code.text_for_tests(),
            SurfaceBlock::Math(math) => math.text(),
            SurfaceBlock::Table(table) => table.text(),
            SurfaceBlock::Diagram(diagram) => diagram.fallback_text().to_string(),
            SurfaceBlock::Image(local_image) => local_image.alt_for_tests(),
            SurfaceBlock::BadgeRow(row) => row.text(),
            SurfaceBlock::Alert(alert) => alert.text(),
            SurfaceBlock::Rule => String::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn debug_for_tests(&self) -> String {
        match self {
            SurfaceBlock::Line(line) => Self::debug_line_for_tests(line),
            SurfaceBlock::Code(code) => Self::debug_code_for_tests(code),
            SurfaceBlock::Math(math) => Self::debug_math_for_tests(math),
            SurfaceBlock::Table(table) => Self::debug_table_for_tests(table),
            SurfaceBlock::Diagram(diagram) => Self::debug_diagram_for_tests(diagram),
            SurfaceBlock::Image(local_image) => Self::debug_image_for_tests(local_image),
            SurfaceBlock::BadgeRow(row) => Self::debug_badge_row_for_tests(row),
            SurfaceBlock::Alert(alert) => Self::debug_alert_for_tests(alert),
            SurfaceBlock::Rule => "rule".to_string(),
        }
    }

    #[cfg(test)]
    fn debug_line_for_tests(line: &SurfaceLine) -> String {
        format!("line:{}:{}", line.text, line.debug_style_tags().join("|"))
    }

    #[cfg(test)]
    fn debug_code_for_tests(code: &SurfaceCodeBlock) -> String {
        format!("code:{}", code.debug_style_tags().join("|"))
    }

    #[cfg(test)]
    fn debug_math_for_tests(math: &SurfaceMathBlock) -> String {
        format!("math:{}", math.text())
    }

    #[cfg(test)]
    fn debug_table_for_tests(table: &SurfaceTableBlock) -> String {
        format!(
            "table:{}x{}:{}",
            table.rows().len(),
            table.column_count(),
            table.text()
        )
    }

    #[cfg(test)]
    fn debug_diagram_for_tests(diagram: &SurfaceDiagramBlock) -> String {
        let size = diagram
            .image
            .as_ref()
            .map(|image| format!("{}x{}", image.image.width(), image.image.height()))
            .unwrap_or_else(|| "missing".to_string());
        format!("diagram:{size}")
    }

    #[cfg(test)]
    fn debug_image_for_tests(local_image: &SurfaceImageBlock) -> String {
        format!(
            "image:{}x{}:{}",
            local_image.image.width(),
            local_image.image.height(),
            local_image.alt_for_tests()
        )
    }

    #[cfg(test)]
    fn debug_badge_row_for_tests(row: &SurfaceBadgeRowBlock) -> String {
        format!("badges:{}:[\"centered\"]", row.text())
    }

    #[cfg(test)]
    fn debug_alert_for_tests(alert: &SurfaceAlertBlock) -> String {
        format!(
            "alert:{}:icon={}:{}",
            alert.label,
            super::markup::alert_icon_name(&alert.label),
            alert.title.text
        )
    }
}

#[cfg(test)]
#[path = "export_surface_blocks_tests.rs"]
mod tests;
