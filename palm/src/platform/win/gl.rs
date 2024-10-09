use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::error::PalmResult;

use super::IntoPalmError;

pub fn init_opengl(hwnd: HWND) -> PalmResult<(HGLRC, HDC)> {
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

pub fn cleanup_opengl(hwnd: HWND, hglrc: HGLRC, hdc: HDC) -> PalmResult<()> {
    unsafe {
        wglMakeCurrent(hdc, None).map_palm_err()?;
        wglDeleteContext(hglrc).map_palm_err()?;
        ReleaseDC(hwnd, hdc);
    };
    Ok(())
}

pub fn init_skia(hwnd: HWND) -> PalmResult<(skia_safe::Surface, skia_safe::gpu::DirectContext)> {
    // Init Skia context for OpenGL rendering
    let interface = skia_safe::gpu::gl::Interface::new_native().unwrap();
    let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None).unwrap();

    // Setup skia surface
    let fb_info = skia_safe::gpu::gl::FramebufferInfo {
        fboid: 0, // Bind to the window's framebuffer (0)
        format: skia_safe::gpu::gl::Format::RGBA8.into(),
        ..Default::default()
    };

    // TODO: Find out how to actually retrieve these values from windows
    let sample_count = 0; // Default to zero for now.
    let stencil_size = 0;

    let surface = {
        let size = get_client_size(hwnd)?;
        let backend_render_target = skia_safe::gpu::backend_render_targets::make_gl(
            size,
            sample_count,
            stencil_size,
            fb_info,
        );
        skia_safe::gpu::surfaces::wrap_backend_render_target(
            &mut gr_context,
            &backend_render_target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .expect("failed to create skia surface")
    };

    Ok((surface, gr_context))
}

pub fn gl_swap_buffers(hwnd: HWND) -> PalmResult<()> {
    let hdc = unsafe { GetDC(hwnd) };

    unsafe {
        SwapBuffers(hdc).map_palm_err()?;
    }

    Ok(())
}

pub fn clear_screen(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        glClearColor(red, green, blue, alpha);
        glClear(GL_COLOR_BUFFER_BIT);
    }
}

fn get_client_size(hwnd: HWND) -> PalmResult<(i32, i32)> {
    let mut rect = RECT::default();
    unsafe { GetClientRect(hwnd, &mut rect) }.map_palm_err()?;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    Ok((width, height))
}
