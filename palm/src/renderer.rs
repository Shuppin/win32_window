pub trait Renderer {
    fn render(&mut self, canvas: &skia_safe::Canvas);
}

pub struct EmptyRenderer;

impl Renderer for EmptyRenderer {
    fn render(&mut self, _canvas: &skia_safe::Canvas) {}
}
