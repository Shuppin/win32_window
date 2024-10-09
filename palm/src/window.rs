//! # Window Module
//!
//! This module provides the functionality to create and manage a window for the application.
//! It is platform-agnostic in terms of configuration but platform-specific in execution,
//! currently supporting only Windows platforms.
use crate::error::PalmResult;

/// Various platform-agnostic attributes to config the behaviour and appearance of the window
pub struct WindowAttributes {
    /// The title of the window, displayed in the window's title bar.
    pub title: String,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: "Palm app".to_string(),
        }
    }
}

/// Initiates the window loop for the application using the provided window attributes.
///
/// On non-Windows platforms, it will panic, as window creation is not supported.
///
/// ## Example
///
/// ```rust
/// use palm::window::{WindowAttributes, run_window_loop};
///
/// fn main() -> PalmResult<()> {
///     let attrs = WindowAttributes::default();
///     run_window_loop(attrs)?;
///     Ok(())
/// }
/// ```
///
/// # Parameters
///
/// * `attrs`: A [`WindowAttributes`] structure containing configuration options for the window.
///
/// # Returns
///
/// Returns a [`PalmResult`], which is used to indicate success or failure
/// of the window creation and loop operation.
///
/// # Platform-specific behavior
///
/// - **Windows**: Calls the platform-specific function to create and run the window loop.
/// - **Non-Windows**: This function will panic as window creation is not supported.
///
/// # Panics
///
/// This function will panic on non-Windows platforms.
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
