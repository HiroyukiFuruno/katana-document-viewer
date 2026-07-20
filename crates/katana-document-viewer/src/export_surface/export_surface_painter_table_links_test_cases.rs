use super::super::push_table_link_annotation;
use super::support::{annotation_request, empty_table_block};

#[test]
fn push_table_link_annotation_skips_empty_target() {
    let table = empty_table_block();
    let mut annotations = Vec::new();
    let request = annotation_request(&table, 0, 0);

    push_table_link_annotation(&mut annotations, request, "", 10, 20, 120, 40);

    assert!(annotations.is_empty());
}
