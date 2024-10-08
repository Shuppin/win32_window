use win::window::WindowBuilder;

mod error;
mod win;

fn main() {
    WindowBuilder::default()
        .with_title("Hello!")
        .build_and_run()
        .unwrap();
}
