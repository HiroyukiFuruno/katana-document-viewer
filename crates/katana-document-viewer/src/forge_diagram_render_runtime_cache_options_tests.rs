use super::super::{DiagramRenderEngine, KrrDiagramRenderEngine};

#[test]
fn krr_diagram_render_cache_options_include_runtime_asset_versions() {
    let options = KrrDiagramRenderEngine.cache_options();

    assert_eq!(96, options.dpi);
    assert_ne!("default", options.renderer_options);
    assert_runtime_renderer_metadata(&options.renderer_options);
}

fn assert_runtime_renderer_metadata(renderer_options: &str) {
    assert!(renderer_options.contains(env!("CARGO_PKG_VERSION")));
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::mermaid_renderer::MERMAID_JS_VERSION)
    );
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::mermaid_renderer::MERMAID_JS_CHECKSUM)
    );
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::drawio_renderer::DRAWIO_JS_VERSION)
    );
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::drawio_renderer::DRAWIO_JS_CHECKSUM)
    );
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::plantuml_renderer::PLANTUML_JAR_VERSION)
    );
    assert!(
        renderer_options
            .contains(katana_render_runtime::markdown::plantuml_renderer::PLANTUML_JAR_CHECKSUM)
    );
}
