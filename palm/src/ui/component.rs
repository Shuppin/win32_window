
pub trait Component {
    fn render(&self, canvas: &skia_safe::Canvas);
}
