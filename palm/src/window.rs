use crate::error::PalmResult;

/// Various platform-agnostic attributes to config the behaviour and appearance of the window
pub struct WindowAttributes {
    /// The title of the window
    pub title: String,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: "Palm app".to_string(),
        }
    }
}

pub fn run_window_loop(attrs: WindowAttributes) -> PalmResult<()> {
    #[cfg(target_os = "windows")]
    {
        crate::platform::win::window::run_window_loop(attrs)
    }
    #[cfg(not(target_os = "windows"))]
    {
        panic!("Creating a window is not supported on this platform")
    }
}
