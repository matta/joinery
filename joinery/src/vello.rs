pub use kurbo;
pub use peniko;

#[derive(Clone)]
pub(crate) struct Scene {}
impl Scene {
    pub(crate) fn new() -> Scene {
        todo!()
    }
}
pub(crate) struct AaSupport {}
pub(crate) struct RenderParams {}
pub(crate) struct Renderer {}
pub(crate) struct RendererOptions {}
pub(crate) enum AaConfig {
    Area,
    Msaa8,
    Msaa16,
}

pub(crate) mod util {
    pub(crate) struct RenderContext {}
    pub(crate) struct RenderSurface {}
}
