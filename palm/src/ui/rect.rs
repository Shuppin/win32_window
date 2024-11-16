use crate::renderer::skia::to_skia_color;

use super::{color::Color, component::Component};

pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Component for Rect {
    fn render(&self, canvas: &skia_safe::Canvas) {
        let mut paint = skia_safe::Paint::default();
        paint.set_color(to_skia_color(&self.color));
        canvas.draw_rect(
            skia_safe::Rect::from_xywh(
                self.x as f32,
                self.y as f32,
                self.width as f32,
                self.height as f32,
            ),
            &paint,
        );
    }
}
