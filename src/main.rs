use window::WindowBuilder;

mod error;
mod window;

fn main() {
    WindowBuilder::default()
        .with_title("Hello!")
        .build_and_run()
        .unwrap();
}
