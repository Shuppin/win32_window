use windows::core::*; // Core utilities from the Windows API (like HRESULTs and Results).
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
// ^ Window messaging, window creation, and input handling functions.

use crate::error::PalmResult;
use crate::platform::win::gl::clear_screen;
use crate::renderer::Renderer;
use crate::window::WindowAttributes;

use super::gl::{cleanup_opengl, gl_swap_buffers, init_opengl, init_skia};
use super::IntoPalmError;

struct WindowState<R: Renderer> {
    sk_surface: skia_safe::Surface,
    sk_context: skia_safe::gpu::DirectContext,
    renderer: R,
}

pub fn run_window_loop<R: Renderer>(
    window_config: WindowAttributes,
    renderer: R,
) -> PalmResult<()> {
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
        lpfnWndProc: Some(wndproc::<R>),
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
            WS_OVERLAPPEDWINDOW,        // Standard window style with title bar and border.
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

    let (hglrc, hdc) = init_opengl(hwnd)?;

    let (sk_surface, sk_context) = init_skia(hwnd)?;

    let window_state = Box::new(WindowState {
        sk_surface,
        sk_context,
        renderer,
    });

    unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(window_state) as isize) };

    // HACK: For some reason, this prevents the window displaying a
    // blank frame (flashbang) before the renderer is invoked.
    // I think it has something to do with the rendering logic being in `WM_PAINT`?
    clear_screen(0.0, 0.0, 0.0, 1.0);

    _ = unsafe { ShowWindow(hwnd, SW_SHOW) };

    run_main_loop();

    cleanup_opengl(hwnd, hglrc, hdc)?;

    Ok(())
}

fn run_main_loop() {
    let mut msg = MSG::default();

    // Message loop: retrieves messages from the queue and dispatches them to the window procedure.
    while unsafe { GetMessageA(&mut msg, None, 0, 0).into() } {
        unsafe {
            _ = TranslateMessage(&msg);
            // Dispatch the message to the appropriate window procedure (wndproc in this case).
            DispatchMessageA(&msg);
        }
    }
}

// The window procedure function. It processes messages sent to the window.
extern "system" fn wndproc<R: Renderer>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        // Handle the WM_PAINT message, which is sent when the window needs to be repainted.
        WM_PAINT => {
            println!("WM_PAINT");
            // Access the window state
            let window_state_ptr =
                unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) } as *mut WindowState<R>;
            if !window_state_ptr.is_null() {
                let window_state = unsafe { &mut *window_state_ptr };
                // Invoke the render callback
                window_state
                    .renderer
                    .render(window_state.sk_surface.canvas());
                window_state.sk_context.flush_and_submit();
                gl_swap_buffers(window).unwrap();
            }
            // Validate the client area to indicate that it has been repainted (no need to redraw).
            _ = unsafe { ValidateRect(window, None) };
            // Return 0 to indicate the message has been processed.
            LRESULT(0)
        }

        // Handle the WM_DESTROY message, which is sent when the window is being destroyed.
        WM_DESTROY => {
            println!("WM_DESTROY");

            // When the window is being destroyed, clean up the state.
            let window_state_ptr =
                unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) } as *mut WindowState<R>;
            if !window_state_ptr.is_null() {
                _ = unsafe { Box::from_raw(window_state_ptr) }; // This will deallocate the struct.
            }

            // Post a quit message to end the message loop and terminate the application.
            unsafe { PostQuitMessage(0) };
            // Return 0 to indicate the message has been processed.
            LRESULT(0)
        }

        // Default case: pass all unhandled messages to the default window procedure.
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}
