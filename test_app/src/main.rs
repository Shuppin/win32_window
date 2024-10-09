use palm::window::{run_window_loop, WindowAttributes};

fn main() {
    let wa = WindowAttributes {
        title: "Hello! :)".to_string(),
        ..Default::default()
    };

    run_window_loop(wa).unwrap();
}
