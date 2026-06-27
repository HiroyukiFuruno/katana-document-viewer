use crate::viewer::commands::{ViewerCommand, ViewerScrollCommand};
use crate::viewer::types::{
    ViewerHitTestResponse, ViewerPoint, ViewerRect, ViewerTarget, ViewerTocItem,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ViewerHitTestIndex {
    targets: Vec<ViewerTarget>,
    y_index: Vec<usize>,
}

impl ViewerHitTestIndex {
    pub fn new(targets: Vec<ViewerTarget>) -> Self {
        let mut y_index = (0..targets.len()).collect::<Vec<_>>();
        y_index.sort_by(|left, right| {
            targets[*left]
                .rect
                .y
                .total_cmp(&targets[*right].rect.y)
                .then_with(|| left.cmp(right))
        });
        Self { targets, y_index }
    }

    pub fn hit_test(&self, point: ViewerPoint) -> ViewerHitTestResponse {
        let mut hit_index = None;
        for index in &self.y_index {
            let target = &self.targets[*index];
            if target.rect.y > point.y {
                break;
            }
            if target.rect.y + target.rect.height < point.y {
                continue;
            }
            if target.rect.contains(point) {
                hit_index = match hit_index {
                    Some(existing) if existing < *index => Some(existing),
                    _ => Some(*index),
                };
            }
        }
        if let Some(index) = hit_index {
            return ViewerHitTestResponse::Hit(self.targets[index].clone());
        }
        ViewerHitTestResponse::Miss(point)
    }
}

pub struct ViewerTocCommandFactory;

impl ViewerTocCommandFactory {
    pub fn scroll_to(item: ViewerTocItem) -> ViewerCommand {
        let target = ViewerTarget {
            node_id: item.node_id,
            source: item.source,
            artifact_id: crate::artifact::ArtifactId(format!("toc:{}", item.anchor_index)),
            rect: item.anchor_rect,
        };
        ViewerCommand::ScrollToHeading(ViewerScrollCommand { target })
    }
}

pub struct ViewerRectFactory;

impl ViewerRectFactory {
    pub fn from_origin_size(x: f32, y: f32, width: f32, height: f32) -> ViewerRect {
        ViewerRect {
            x,
            y,
            width,
            height,
        }
    }
}
