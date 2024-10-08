use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;

use crate::error::PalmResult;

use super::IntoPalmError;

pub(crate) fn init_opengl(hwnd: HWND) -> PalmResult<(HGLRC, HDC)> {
    // Get device context
    let hdc = unsafe { GetDC(hwnd) };

    // Define pixel format descriptor
    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        iLayerType: PFD_MAIN_PLANE.0 as u8,
        ..Default::default()
    };

    // Set pixel format
    let pixel_format = unsafe { ChoosePixelFormat(hdc, &pfd) };
    unsafe {
        SetPixelFormat(hdc, pixel_format, &pfd).map_palm_err()?;
    }

    // Create OpenGL context
    let hglrc = unsafe { wglCreateContext(hdc) }.with_err_msg("Failed to create gl context")?;

    // Make the context current
    unsafe {
        wglMakeCurrent(hdc, hglrc).map_palm_err()?;
    }

    Ok((hglrc, hdc))
}

pub(crate) fn cleanup_opengl(hwnd: HWND, hglrc: HGLRC, hdc: HDC) -> PalmResult<()> {
    unsafe {
        wglMakeCurrent(hdc, None).map_palm_err()?;
        wglDeleteContext(hglrc).map_palm_err()?;
        ReleaseDC(hwnd, hdc);
    };
    Ok(())
}
