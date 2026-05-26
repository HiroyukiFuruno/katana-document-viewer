use super::super::SurfaceBlock;
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_raw(
        blocks: &mut Vec<SurfaceBlock>,
        raw: &str,
        quote_depth: u32,
        list_depth: u32,
    ) {
        for line in raw.lines() {
            Self::append_wrapped(blocks, line.to_string(), quote_depth, list_depth);
        }
    }
}

#[cfg(test)]
#[path = "raw_tests.rs"]
mod tests;
