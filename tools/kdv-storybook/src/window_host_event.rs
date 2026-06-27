#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StorybookHostEvent {
    DiagramFullscreen { node_id: String, open: bool },
}
