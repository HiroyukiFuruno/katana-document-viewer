use crate::KdvThemeSnapshot;
use crate::PreviewOutput;
use crate::forge_diagram_render_types::DiagramRenderCacheOptions;
use katana_markdown_model::DiagramKind;
use katana_markdown_model::KmmNode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PreviewDiagramAssetCacheKey {
    engine: String,
    document: String,
    kind: String,
    source: String,
    position: String,
    theme: String,
    dpi: String,
    renderer_options: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewDiagramAssetCacheValue {
    pub(crate) svg: Vec<u8>,
}

pub(crate) struct PreviewDiagramAssetCache;

impl PreviewDiagramAssetCache {
    pub(crate) fn key(
        engine: &str,
        output: &PreviewOutput,
        node: &KmmNode,
        kind: &DiagramKind,
        source: &str,
        theme: &KdvThemeSnapshot,
        cache_options: &DiagramRenderCacheOptions,
    ) -> PreviewDiagramAssetCacheKey {
        PreviewDiagramAssetCacheKey {
            engine: Self::hash(engine),
            document: Self::document_key(output),
            kind: Self::kind_label(kind).to_string(),
            source: Self::source_key(kind, source),
            position: Self::position_key(node),
            theme: Self::hash(&Self::theme_fingerprint(theme)),
            dpi: cache_options.dpi.to_string(),
            renderer_options: Self::hash(&cache_options.renderer_options),
        }
    }

    pub(crate) fn get(key: &PreviewDiagramAssetCacheKey) -> Option<PreviewDiagramAssetCacheValue> {
        cache().lock().ok()?.get(key).cloned()
    }

    pub(crate) fn put(key: PreviewDiagramAssetCacheKey, svg: Vec<u8>) {
        if let Ok(mut cache) = cache().lock() {
            cache.insert(key, PreviewDiagramAssetCacheValue { svg });
        }
    }

    pub(crate) fn get_disk(
        root: &Path,
        key: &PreviewDiagramAssetCacheKey,
    ) -> Option<PreviewDiagramAssetCacheValue> {
        let path = key.cache_path(root);
        let svg = std::fs::read(&path).ok()?;
        if Self::is_svg(&svg) {
            return Some(PreviewDiagramAssetCacheValue { svg });
        }
        let _ = std::fs::remove_file(path);
        None
    }

    pub(crate) fn put_disk(root: &Path, key: &PreviewDiagramAssetCacheKey, svg: &[u8]) {
        if !Self::is_svg(svg) {
            return;
        }
        let path = key.cache_path(root);
        let Some(parent) = path.parent() else {
            return;
        };
        if std::fs::create_dir_all(parent).is_err() {
            return;
        }
        let temp = Self::temp_path(&path);
        if std::fs::write(&temp, svg).is_err() {
            return;
        }
        let _ = std::fs::rename(temp, path);
    }

    #[cfg(test)]
    pub(crate) fn clear_memory_for_tests() {
        if let Ok(mut cache) = cache().lock() {
            cache.clear();
        }
    }

    fn kind_label(kind: &DiagramKind) -> &'static str {
        match kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => "plantuml",
        }
    }

    fn theme_fingerprint(theme: &KdvThemeSnapshot) -> String {
        format!(
            "{}:{:?}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            theme.name,
            theme.mode,
            theme.background,
            theme.text,
            theme.diagram_background,
            theme.diagram_text,
            theme.diagram_fill,
            theme.diagram_stroke,
            theme.diagram_arrow,
            theme.mermaid_theme,
            theme.syntax_theme_dark,
            theme.syntax_theme_light
        )
    }

    fn document_key(output: &PreviewOutput) -> String {
        let path = output.input.snapshot.source_path.to_string_lossy();
        format!("doc_{}", Self::hash(path.as_ref()))
    }

    fn source_key(kind: &DiagramKind, source: &str) -> String {
        let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
        Self::hash(&format!(
            "kind={}\nsource={normalized}",
            Self::kind_label(kind)
        ))
    }

    fn position_key(node: &KmmNode) -> String {
        let source = &node.source;
        Self::hash(&format!(
            "node={}\nbytes={}:{}\nlines={}:{}-{}:{}",
            node.id.0,
            source.byte_range.start,
            source.byte_range.end,
            source.line_column_range.start.line,
            source.line_column_range.start.column,
            source.line_column_range.end.line,
            source.line_column_range.end.column
        ))
    }

    fn hash(data: &str) -> String {
        let mut hash = 0xcbf29ce484222325_u64;
        for byte in data.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:x}")
    }

    fn is_svg(svg: &[u8]) -> bool {
        let Ok(text) = std::str::from_utf8(svg) else {
            return false;
        };
        text.contains("<svg") && text.contains("</svg>")
    }

    fn temp_path(path: &Path) -> PathBuf {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("diagram.svg");
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        path.with_file_name(format!("{name}.tmp.{}-{nanos}", std::process::id()))
    }
}

impl PreviewDiagramAssetCacheKey {
    fn cache_path(&self, root: &Path) -> PathBuf {
        root.join(&self.document).join(&self.kind).join(format!(
            "{}_{}_{}_{}_{}_{}.svg",
            self.source, self.position, self.engine, self.theme, self.dpi, self.renderer_options
        ))
    }
}

fn cache() -> &'static Mutex<HashMap<PreviewDiagramAssetCacheKey, PreviewDiagramAssetCacheValue>> {
    static CACHE: OnceLock<
        Mutex<HashMap<PreviewDiagramAssetCacheKey, PreviewDiagramAssetCacheValue>>,
    > = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}
