use windows::core::*; // Core utilities from the Windows API (like HRESULTs and Results).
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
// ^ Window messaging, window creation, and input handling functions.

use crate::error::PalmResult;
use crate::window::WindowAttributes;

use super::gl::{cleanup_opengl, init_opengl, temp_draw};
use super::IntoPalmError;

pub fn run_window_loop(window_config: WindowAttributes) -> PalmResult<()> {
    // Get the handle of the current instance (HINSTANCE).
    // This is required when registering the window class and creating windows.
    // Safety: This function almost never fails when `None` is passed,
    // if it does fail there is a serious issue with the underlying OS.
    let instance =
        unsafe { GetModuleHandleA(None) }.expect("failed to retrieve handle to executable");

    // Define the window class name, as a string.
    let window_class = s!("window");

    let cursor_handle = unsafe { LoadCursorW(None, IDC_ARROW) }
        .expect(&format!("failed to load cursor. error code {:?}", unsafe {
            GetLastError()
        }));

    // Set up the WNDCLASSA struct to define window behavior and appearance.
    let wc = WNDCLASSA {
        // Load a standard arrow cursor to be used for the window.
        hCursor: cursor_handle,
        // Specify the instance handle obtained earlier.
        hInstance: instance.into(),
        // Specify the name of the window class.
        lpszClassName: window_class,

        // Set the window class style flags, allowing redraw on horizontal and vertical resize.
        style: CS_HREDRAW | CS_VREDRAW,
        // Specify the window procedure function to handle window messages.
        lpfnWndProc: Some(wndproc),
        // Set default values for the remaining fields.
        ..Default::default()
    };

    // Register the window class with the system.
    let atom = unsafe { RegisterClassA(&wc) };
    // Ensure the class registration was successful by checking that the returned atom is non-zero.
    debug_assert!(atom != 0);

    // Create a window based on the registered window class.
    let hwnd = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(), // Extended window style (default).
            window_class,               // The class name.
            PCSTR::from_raw(format!("{}\0", window_config.title).as_ptr()), // The window title.
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, // Standard window style with title bar, border, and visible on creation.
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT, // Default position and size.
            None,          // No parent window.
            None,          // No menu.
            instance,      // Instance handle.
            None,          // No additional parameters.
        )
    }
    .with_err_msg("Failed to create window")?;

    run_main_loop(hwnd)?;

    Ok(())
}

fn run_main_loop(hwnd: HWND) -> PalmResult<()> {
    // Initialise OpenGL
    let (hglrc, hdc) = init_opengl(hwnd)?;

    let mut msg = MSG::default();

    // Message loop: retrieves messages from the queue and dispatches them to the window procedure.
    while unsafe { GetMessageA(&mut msg, None, 0, 0).into() } {
        unsafe {
            _ = TranslateMessage(&msg);
            // Dispatch the message to the appropriate window procedure (wndproc in this case).
            DispatchMessageA(&msg);
        }
    }

    cleanup_opengl(hwnd, hglrc, hdc)?;

    Ok(())
}

// The window procedure function. It processes messages sent to the window.
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        // Handle the WM_PAINT message, which is sent when the window needs to be repainted.
        WM_PAINT => {
            println!("WM_PAINT");
            temp_draw(window).unwrap();
            // Validate the client area to indicate that it has been repainted (no need to redraw).
            _ = unsafe { ValidateRect(window, None) };
            // Return 0 to indicate the message has been processed.
            LRESULT(0)
        }
        // Handle the WM_DESTROY message, which is sent when the window is being destroyed.
        WM_DESTROY => {
            println!("WM_DESTROY");
            // Post a quit message to end the message loop and terminate the application.
            unsafe { PostQuitMessage(0) };
            // Return 0 to indicate the message has been processed.
            LRESULT(0)
        }
        // Default case: pass all unhandled messages to the default window procedure.
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}
