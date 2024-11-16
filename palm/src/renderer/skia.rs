use crate::error::PalmResult;

pub fn create_skia_gr_context() -> PalmResult<skia_safe::gpu::DirectContext> {
    // The interface between Skia and OpenGL
    let gr_gl_interface = skia_safe::gpu::gl::Interface::new_native().unwrap();
    // `new_native()` makes Skia extract pointers to OpenGL functions for
    // the current context in a platform-native way.

    // GrContext manages the GPU context, and related caches for textures and fonts.
    // GrContexts are matched one to one with OpenGL contexts.
    let gr_context = skia_safe::gpu::direct_contexts::make_gl(gr_gl_interface, None).unwrap();

    Ok(gr_context)
}

pub fn create_skia_surface(
    size: (i32, i32),
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> skia_safe::Surface {
    // Setup skia surface
    let fb_info = skia_safe::gpu::gl::FramebufferInfo {
        fboid: 0, // Bind to the window's framebuffer (0)
        format: skia_safe::gpu::gl::Format::RGBA8.into(),
        ..Default::default()
    };

    // TODO: Find out how to actually retrieve these values from windows
    let sample_count = 0; // Default to zero for now.
    let stencil_size = 0;

    let backend_render_target =
        skia_safe::gpu::backend_render_targets::make_gl(size, sample_count, stencil_size, fb_info);
    skia_safe::gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        skia_safe::ColorType::RGBA8888,
        None,
        None,
    )
    .expect("failed to create skia surface")
}

pub fn to_skia_color(color: &crate::ui::color::Color) -> skia_safe::Color {
    skia_safe::Color::new(color.inner())
}
