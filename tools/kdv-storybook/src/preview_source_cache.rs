use crate::catalog::StorybookFixture;
use crate::preview_build_support::PreviewBuildSupport;
use katana_document_viewer::MarkdownSource;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Default)]
pub(crate) struct PreviewSourceCache {
    entries: BTreeMap<PathBuf, CachedSourceEntry>,
    hits: usize,
    misses: usize,
}

impl PreviewSourceCache {
    pub(crate) fn source_for_fixture(
        &mut self,
        fixture: &StorybookFixture,
    ) -> Result<MarkdownSource, Box<dyn std::error::Error>> {
        let signature = SourceFileSignature::read(&fixture.path)?;
        if let Some(entry) = self.entries.get(&fixture.path)
            && entry.signature == signature
        {
            self.hits += 1;
            return Ok(entry.source.clone());
        }
        let source = PreviewBuildSupport::source_for_fixture(fixture)?;
        self.entries.insert(
            fixture.path.clone(),
            CachedSourceEntry {
                signature,
                source: source.clone(),
            },
        );
        self.misses += 1;
        Ok(source)
    }

    #[cfg(test)]
    pub(crate) const fn stats(&self) -> PreviewSourceCacheStats {
        PreviewSourceCacheStats {
            hits: self.hits,
            misses: self.misses,
        }
    }
}

struct CachedSourceEntry {
    signature: SourceFileSignature,
    source: MarkdownSource,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SourceFileSignature {
    len: u64,
    modified: SystemTime,
}

impl SourceFileSignature {
    fn read(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = std::fs::metadata(path)?;
        Ok(Self {
            len: metadata.len(),
            modified: metadata.modified()?,
        })
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PreviewSourceCacheStats {
    pub(crate) hits: usize,
    pub(crate) misses: usize,
}
