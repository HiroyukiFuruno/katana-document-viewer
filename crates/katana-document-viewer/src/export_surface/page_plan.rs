use super::SurfaceBlock;
use crate::export_surface_helpers::{PAGE_PADDING, SURFACE_PAGE_HEIGHT};

pub(super) struct SurfacePagePlan {
    pub(super) pages: Vec<Vec<usize>>,
}

impl SurfacePagePlan {
    pub(super) fn from_blocks(blocks: &[SurfaceBlock]) -> Self {
        let mut pages = Vec::new();
        let mut current = Vec::new();
        let mut y = PAGE_PADDING;
        let mut index = 0;
        while let Some(block) = blocks.get(index) {
            if Self::should_move_to_next_page(blocks, index, y) {
                pages.push(current);
                current = Vec::new();
                y = PAGE_PADDING;
                continue;
            }
            if y > PAGE_PADDING && y + block.height() > Self::page_bottom() {
                pages.push(current);
                current = Vec::new();
                y = PAGE_PADDING;
                continue;
            }
            current.push(index);
            y += block.height();
            index += 1;
        }
        if !current.is_empty() {
            pages.push(current);
        }
        Self { pages }
    }

    fn should_move_to_next_page(blocks: &[SurfaceBlock], index: usize, y: u32) -> bool {
        if y == PAGE_PADDING {
            return false;
        }
        let Some(keep_height) = Self::heading_keep_with_next_height(blocks, index) else {
            return false;
        };
        keep_height <= Self::page_content_height() && y + keep_height > Self::page_bottom()
    }

    fn heading_keep_with_next_height(blocks: &[SurfaceBlock], index: usize) -> Option<u32> {
        let block = blocks.get(index)?;
        if !block.is_heading() {
            return None;
        }
        let mut height = block.height();
        let mut cursor = index + 1;
        let mut has_following_block = false;
        while let Some(next) = blocks.get(cursor) {
            height += next.height();
            has_following_block = true;
            cursor += 1;
            if !next.is_heading() {
                break;
            }
        }
        has_following_block.then_some(height)
    }

    fn page_bottom() -> u32 {
        SURFACE_PAGE_HEIGHT - PAGE_PADDING
    }

    fn page_content_height() -> u32 {
        SURFACE_PAGE_HEIGHT - PAGE_PADDING * 2
    }
}

#[cfg(test)]
#[path = "page_plan_tests.rs"]
mod tests;
