#![allow(dead_code, unused_variables)]

pub use kurbo;
pub use peniko;

use kurbo::{Affine, Rect, Shape, Stroke};
use peniko::{BlendMode, BrushRef, Fill, Font, Image, StyleRef};

use self::glyph::Glyph;

#[derive(Clone, Default)]
pub struct Encoding {}
impl Encoding {
    fn reset(&mut self) {
        todo!()
    }
}

/// The main datatype for rendering graphics.
///
/// A Scene stores a sequence of drawing commands, their context, and the
/// associated resources, which can later be rendered.
#[derive(Clone, Default)]
pub struct Scene {
    encoding: Encoding,
}

impl Scene {
    pub fn new() -> Scene {
        Self::default()
    }

    /// Removes all content from the scene.
    pub fn reset(&mut self) {
        self.encoding.reset();
    }

    /// Returns the underlying raw encoding.
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    /// Pushes a new layer clipped by the specified shape and composed with
    /// previous layers using the specified blend mode.
    ///
    /// Every drawing command after this call will be clipped by the shape
    /// until the layer is popped.
    ///
    /// **However, the transforms are *not* saved or modified by the layer stack.**
    pub fn push_layer(
        &mut self,
        _blend: impl Into<BlendMode>,
        _alpha: f32,
        _transform: Affine,
        _clip: &impl Shape,
    ) {
        todo!()
    }

    /// Pops the current layer.
    pub fn pop_layer(&mut self) {
        todo!()
    }

    /// Fills a shape using the specified style and brush.
    pub fn fill<'b>(
        &mut self,
        _style: Fill,
        _transform: Affine,
        _brush: impl Into<BrushRef<'b>>,
        _brush_transform: Option<Affine>,
        _shape: &impl Shape,
    ) {
        todo!()
    }

    /// Strokes a shape using the specified style and brush.
    pub fn stroke<'b>(
        &mut self,
        _style: &Stroke,
        _transform: Affine,
        _brush: impl Into<peniko::BrushRef<'b>>,
        _wbrush_transform: Option<Affine>,
        _shape: &impl Shape,
    ) {
        todo!()
    }

    /// Draws an image at its natural size with the given transform.
    pub fn draw_image(&mut self, image: &Image, transform: Affine) {
        self.fill(
            Fill::NonZero,
            transform,
            image,
            None,
            &Rect::new(0.0, 0.0, image.width as f64, image.height as f64),
        );
    }

    /// Returns a builder for encoding a glyph run.
    #[allow(unused_variables)]
    pub fn draw_glyphs(&mut self, font: &Font) -> DrawGlyphs {
        todo!()
    }

    /// Appends a child scene.
    ///
    /// The given transform is applied to every transform in the child.
    /// This is an O(N) operation.
    #[allow(unused_variables)]
    pub fn append(&mut self, other: &Scene, transform: Option<Affine>) {
        todo!()
    }
}

/// Builder for encoding a glyph run.
pub struct DrawGlyphs<'a> {
    encoding: &'a mut Encoding,
    // TODO: this was here -> run: GlyphRun,
    brush: BrushRef<'a>,
    brush_alpha: f32,
}

impl<'a> DrawGlyphs<'a> {
    /// Sets the global transform. This is applied to all glyphs after the offset
    /// translation.
    ///
    /// The default value is the identity matrix.
    #[allow(unused_mut, unused_variables)] // TODO: remove this
    pub fn transform(mut self, transform: Affine) -> Self {
        // TODO: this was here -> self.run.transform = Transform::from_kurbo(&transform);
        // TODO: this was here -> self
        todo!()
    }

    /// Sets the per-glyph transform. This is applied to all glyphs prior to
    /// offset translation. This is common used for applying a shear to simulate
    /// an oblique font.
    ///
    /// The default value is `None`.
    #[allow(unused_mut, unused_variables)] // TODO: remove this
    pub fn glyph_transform(mut self, transform: Option<Affine>) -> Self {
        todo!()
        // ignore this and return self?
    }

    /// Sets the font size in pixels per em units.
    ///
    /// The default value is 16.0.
    pub fn font_size(self, size: f32) -> Self {
        let _ = size;
        // TODO: this was here self.run.font_size = size;
        self
    }

    /// Sets the brush.
    ///
    /// The default value is solid black.
    pub fn brush(mut self, brush: impl Into<BrushRef<'a>>) -> Self {
        self.brush = brush.into();
        self
    }

    /// Encodes a fill or stroke for the given sequence of glyphs and consumes the builder.
    ///
    /// The `style` parameter accepts either `Fill` or `&Stroke` types.
    pub fn draw(self, style: impl Into<StyleRef<'a>>, glyphs: impl Iterator<Item = Glyph>) {
        todo!();

        #[cfg(any())]
        {
            let resources = &mut self.encoding.resources;
            self.run.style = style.into().to_owned();
            resources.glyphs.extend(glyphs);
            self.run.glyphs.end = resources.glyphs.len();
            if self.run.glyphs.is_empty() {
                resources
                    .normalized_coords
                    .truncate(self.run.normalized_coords.start);
                return;
            }
            let index = resources.glyph_runs.len();
            resources.glyph_runs.push(self.run);
            resources.patches.push(Patch::GlyphRun { index });
            self.encoding.encode_brush(self.brush, self.brush_alpha);
            // Glyph run resolve step affects transform and style state in a way
            // that is opaque to the current encoding.
            // See <https://github.com/linebender/vello/issues/424>
            self.encoding.force_next_transform_and_style();
        }
    }
}

pub mod glyph {
    use std::ops::Range;

    use peniko::{Font, Style};

    #[derive(Clone)]
    pub struct Transform {}
    #[derive(Clone)]
    pub struct StreamOffsets {}

    /// Positioned glyph.
    #[derive(Copy, Clone, Default, Debug)]
    pub struct Glyph {
        /// Glyph identifier.
        pub id: u32,
        /// X-offset in run, relative to transform.
        pub x: f32,
        /// Y-offset in run, relative to transform.
        pub y: f32,
    }

    /// Properties for a sequence of glyphs in an encoding.
    #[derive(Clone)]
    pub struct GlyphRun {
        /// Font for all glyphs in the run.
        pub font: Font,
        /// Global run transform.
        pub transform: Transform,
        /// Per-glyph transform.
        pub glyph_transform: Option<Transform>,
        /// Size of the font in pixels per em.
        pub font_size: f32,
        /// True if hinting is enabled.
        pub hint: bool,
        /// Range of normalized coordinates in the parent encoding.
        pub normalized_coords: Range<usize>,
        /// Fill or stroke style.
        pub style: Style,
        /// Range of glyphs in the parent encoding.
        pub glyphs: Range<usize>,
        /// Stream offsets where this glyph run should be inserted.
        pub stream_offsets: StreamOffsets,
    }
}

pub struct AaSupport {}
pub struct RenderParams {}
pub struct Renderer {}
pub struct RendererOptions {}
pub enum AaConfig {
    Area,
    Msaa8,
    Msaa16,
}

pub mod util {
    pub struct RenderContext {}
    pub struct RenderSurface {}
}
