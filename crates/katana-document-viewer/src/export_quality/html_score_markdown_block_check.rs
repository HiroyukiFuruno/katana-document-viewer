#[derive(Clone, Copy)]
pub(super) struct HtmlBlockCheckSpec {
    pub(super) name: &'static str,
    pub(super) required: bool,
    pub(super) present: bool,
}

impl HtmlBlockCheckSpec {
    pub(super) fn new(name: &'static str, required: bool, present: bool) -> Self {
        Self {
            name,
            required,
            present,
        }
    }
}
