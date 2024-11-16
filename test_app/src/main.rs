use palm::{
    ui::{color::Color, component::Component, rect::Rect},
    window::{run_window_loop, WindowAttributes},
};

fn main() {
    let wa = WindowAttributes {
        title: "Hello! :)".to_string(),
        ..Default::default()
    };

    let components: Vec<Box<dyn Component>> = vec![
        Box::new(Rect::new(0, 0, 10000, 10000, Color::from_gray(0x10))),
        Box::new(Rect::new(100, 100, 100, 100, Color::RED)),
        Box::new(Rect::new(0, 0, 80, 50, Color::BLUE)),
    ];

    run_window_loop(wa, components).unwrap();
}
