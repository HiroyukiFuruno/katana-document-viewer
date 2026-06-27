use crate::canvas::Canvas;
use crate::layout::StorybookPreviewArea;

#[derive(Clone, Copy, Debug)]
pub(crate) struct PreviewBounds {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl PreviewBounds {
    pub(crate) fn for_frame(width: usize, height: usize) -> Self {
        let area = StorybookPreviewArea::for_window(width, height, 0.0);
        Self {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
        }
    }

    pub(crate) fn for_canvas(canvas: &Canvas) -> Self {
        Self::for_frame(canvas.width(), canvas.height())
    }

    pub(crate) fn right(&self) -> usize {
        self.x + self.width
    }

    pub(crate) fn bottom(&self) -> usize {
        self.y + self.height
    }

    pub(crate) fn center_x(&self) -> f32 {
        self.x as f32 + self.width as f32 / 2.0
    }
}
