use palm::{
    renderer::Renderer,
    window::{run_window_loop, WindowAttributes},
};

fn main() {
    let wa = WindowAttributes {
        title: "Hello! :)".to_string(),
        ..Default::default()
    };

    run_window_loop(wa, MyRenderer).unwrap();
}

struct MyRenderer;

impl Renderer for MyRenderer {
    fn render(&mut self, canvas: &skia_safe::Canvas) {
        canvas.clear(skia_safe::Color::DARK_GRAY);
        let mut paint = skia_safe::Paint::default();
        paint.set_color(skia_safe::Color::BLACK);
        canvas.draw_circle((0, 0), 100.0, &paint);
    }
}
