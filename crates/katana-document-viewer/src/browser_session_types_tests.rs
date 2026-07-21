use super::BrowserSessionOperation;

#[test]
fn operation_labels_cover_every_traceable_command() {
    assert_eq!(BrowserSessionOperation::Start.to_string(), "start");
    assert_eq!(BrowserSessionOperation::Input.to_string(), "input");
    assert_eq!(BrowserSessionOperation::Resize.to_string(), "resize");
    assert_eq!(BrowserSessionOperation::Navigate.to_string(), "navigate");
    assert_eq!(BrowserSessionOperation::Refresh.to_string(), "refresh");
}
