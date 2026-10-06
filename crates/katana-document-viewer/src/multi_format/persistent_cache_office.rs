use super::{CachedPage, PersistentOfficeViewerSession};
use crate::multi_format::office_worker_parent::{OfficeWorkerOutput, OfficeWorkerRunner};
use crate::multi_format::persistent_cache::{
    PersistentCacheError, PersistentDocumentCache, key::CacheKey,
};
use crate::multi_format::{
    OfficeDocumentSource, OfficeStaticDocumentArtifact, OfficeStaticViewerSession,
    OfficeWorkerConfig, PdfPageRenderRequest, PdfRenderedPage,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ConversionArtifact {
    pdf: Vec<u8>,
    warnings: Vec<String>,
    diagnostics: Vec<crate::multi_format::ViewerDiagnostic>,
}

impl PersistentOfficeViewerSession {
    pub fn open(
        source: OfficeDocumentSource,
        config: OfficeWorkerConfig,
        cache: PersistentDocumentCache,
    ) -> Result<Self, PersistentCacheError> {
        if source.format == crate::multi_format::OfficeDocumentFormat::Xlsx {
            return Err(
                crate::multi_format::OfficeWorkerError::UnsupportedFormat(source.format).into(),
            );
        }
        let trace =
            crate::multi_format::office_worker_parent::trace::start_trace_session(&source.identity);
        let _scope = trace.map(crate::multi_format::debug_trace::DebugTrace::session);
        let _open = crate::multi_format::debug_trace::DebugTrace::start("office.session_open");
        let key = CacheKey::office(&cache, &source, &config)?;
        let (output, conversion_cache_hit) =
            ConversionArtifact::output(&cache, &key, &source, &config)?;
        let conversion_key =
            crate::multi_format::office_conversion_key::OfficeConversionKey::new(&source, &config);
        let session =
            OfficeStaticViewerSession::from_output(source, config, output, conversion_key, trace)?;
        Ok(Self {
            session,
            cache,
            key,
            conversion_cache_hit,
        })
    }

    #[must_use]
    pub const fn conversion_cache_hit(&self) -> bool {
        self.conversion_cache_hit
    }

    #[must_use]
    pub const fn artifact(&self) -> &OfficeStaticDocumentArtifact {
        self.session.artifact()
    }

    pub fn render_item(
        &mut self,
        request: PdfPageRenderRequest,
    ) -> Result<(PdfRenderedPage, bool), PersistentCacheError> {
        let limits = crate::multi_format::PdfViewerLimits::strict();
        self.session.validate_cached_request(request)?;
        let key = CacheKey::page(
            &self.key,
            request.page_index,
            request.scale,
            &format!("{limits:?}"),
        );
        if let Some(page) = CachedPage::load(&self.cache, &key, request, limits)? {
            return Ok((page, true));
        }
        let page = self.session.render_item_with_worker(request)?;
        CachedPage::save(&self.cache, &key, &page)?;
        Ok((page, false))
    }
}

impl ConversionArtifact {
    fn output(
        cache: &PersistentDocumentCache,
        key: &str,
        source: &OfficeDocumentSource,
        config: &OfficeWorkerConfig,
    ) -> Result<(OfficeWorkerOutput, bool), PersistentCacheError> {
        if let Some(bytes) = cache.load(key)? {
            crate::multi_format::OfficePackagePreflight::inspect(source, config.preflight_limits)
                .map_err(crate::multi_format::OfficeWorkerError::from)?;
            let artifact: Self =
                serde_json::from_slice(&bytes).map_err(|_| PersistentCacheError::Corrupt)?;
            if artifact.pdf.len() as u64 > config.max_output_bytes {
                return Err(PersistentCacheError::Capacity);
            }
            return Ok((artifact.into_output(), true));
        }
        let output = OfficeWorkerRunner::convert(source, config)?;
        let artifact = Self {
            pdf: output.pdf,
            warnings: output.warnings,
            diagnostics: output.preflight_diagnostics,
        };
        let bytes = serde_json::to_vec(&artifact).map_err(|_| PersistentCacheError::Corrupt)?;
        cache.save(key, &bytes)?;
        Ok((artifact.into_output(), false))
    }

    fn into_output(self) -> OfficeWorkerOutput {
        OfficeWorkerOutput {
            pdf: self.pdf,
            warnings: self.warnings,
            preflight_diagnostics: self.diagnostics,
        }
    }
}

#[cfg(test)]
#[path = "persistent_cache_office_tests.rs"]
mod tests;
