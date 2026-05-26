pub(crate) const KRR_RENDER_RUNTIME_ID: &str = "katana-render-runtime";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KrrMathMode {
    Inline,
    Display,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum KrrRenderKind {
    MathTex,
}

use katana_render_runtime::RenderThemeSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KrrRenderRequest {
    pub(crate) kind: KrrRenderKind,
    pub(crate) source: String,
    pub(crate) math_mode: KrrMathMode,
    pub(crate) theme: Option<RenderThemeSnapshot>,
}

impl KrrRenderRequest {
    pub(crate) fn math_tex(source: &str, math_mode: KrrMathMode) -> Self {
        Self {
            kind: KrrRenderKind::MathTex,
            source: source.to_string(),
            math_mode,
            theme: None,
        }
    }

    pub(crate) fn with_theme(mut self, theme: Option<RenderThemeSnapshot>) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KrrRenderDiagnostic {
    pub(crate) code: &'static str,
    pub(crate) message: String,
}

impl KrrRenderDiagnostic {
    pub(crate) fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum KrrRenderPayload {
    Svg(String),
    Raw(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KrrRenderOutput {
    pub(crate) payload: KrrRenderPayload,
    pub(crate) diagnostics: Vec<KrrRenderDiagnostic>,
}

impl KrrRenderOutput {
    pub(crate) fn svg(svg: String) -> Self {
        Self {
            payload: KrrRenderPayload::Svg(svg),
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn raw(raw: String, diagnostic: KrrRenderDiagnostic) -> Self {
        Self {
            payload: KrrRenderPayload::Raw(raw),
            diagnostics: vec![diagnostic],
        }
    }

    pub(crate) fn svg_payload(&self) -> Option<&str> {
        match &self.payload {
            KrrRenderPayload::Svg(svg) => Some(svg),
            KrrRenderPayload::Raw(_) => None,
        }
    }

    pub(crate) fn raw_payload(&self) -> &str {
        match &self.payload {
            KrrRenderPayload::Svg(_) => "",
            KrrRenderPayload::Raw(raw) => raw,
        }
    }

    pub(crate) fn diagnostic_message(&self) -> String {
        self.diagnostics
            .iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

pub(crate) trait KrrRenderRuntime {
    fn render(&self, request: KrrRenderRequest) -> KrrRenderOutput;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_supports_kind_and_theme() {
        let request = KrrRenderRequest::math_tex("a+b", KrrMathMode::Display).with_theme(None);

        assert_eq!(request.kind, KrrRenderKind::MathTex);
        assert_eq!(request.source, "a+b");
        assert_eq!(request.math_mode, KrrMathMode::Display);
        assert!(request.theme.is_none());
    }

    #[test]
    fn output_payload_helpers_and_diagnostics_message() {
        let svg = KrrRenderOutput::svg("<svg/>".to_string());
        let raw = KrrRenderOutput::raw(
            "source".to_string(),
            KrrRenderDiagnostic::new("code", "msg"),
        );
        let diagnostic_output = KrrRenderOutput {
            diagnostics: vec![
                KrrRenderDiagnostic::new("a", "first"),
                KrrRenderDiagnostic::new("b", "second"),
            ],
            ..KrrRenderOutput::raw("".to_string(), KrrRenderDiagnostic::new("x", "y"))
        };

        assert_eq!(svg.svg_payload(), Some("<svg/>"));
        assert_eq!(svg.raw_payload(), "");
        assert_eq!(raw.svg_payload(), None);
        assert_eq!(raw.raw_payload(), "source");
        assert_eq!(
            diagnostic_output.diagnostic_message(),
            "a: first; b: second"
        );
    }
}
