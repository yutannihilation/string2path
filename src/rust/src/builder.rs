use lyon::{
    geom::euclid::UnknownUnit,
    math::point,
    path::{
        Path,
        traits::{Build, PathBuilder},
    },
};
use skrifa::outline::OutlinePen;

// Minimal color type used for COLR glyph layers.
// Replaces ttf_parser::RgbaColor.
#[derive(Copy, Clone, Debug)]
pub struct RgbaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl std::fmt::Display for RgbaColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            red,
            green,
            blue,
            alpha,
        } = self;
        write!(f, "#{red:02x}{green:02x}{blue:02x}{alpha:02x}")
    }
}

/// Formats an optional COLR color as a hex string.
/// Non-COLR glyphs in mixed text default to opaque black.
pub fn color_to_hex(color: Option<RgbaColor>) -> String {
    match color {
        Some(c) => c.to_string(),
        None => "#000000ff".to_string(),
    }
}

pub trait BuildPath: Build<PathType = Path> + PathBuilder {
    // TODO: lyon::path::builder::Transformed is a struct, not a trait. So, this
    // method is needed to forward the operation.
    fn set_transform(&mut self, transform: lyon::math::Transform);
    fn new_builder(tolerance: f32) -> Self;
}

pub struct LyonPathBuilder<T: BuildPath> {
    pub builders: Vec<T>,
    pub cur_layer: usize,

    pub cur_glyph_id: u32,

    // Completed per-glyph paths produced by `finish_glyph()`.
    // Each entry holds (glyph_id, path, optional COLR color).
    pub glyph_paths: Vec<(u32, Path, Option<RgbaColor>)>,

    // This transformation is of COLR format.
    base_transform: lyon::geom::euclid::Transform2D<f32, UnknownUnit, UnknownUnit>,

    // multiply by this to scale the position into the range of [0, 1].
    scale_factor: f32,

    offset_x: f32,
    offset_y: f32,

    pub tolerance: f32,

    // line width of the stroke
    pub line_width: f32,
}

impl<T: BuildPath> LyonPathBuilder<T> {
    fn new_inner(builder: T, tolerance: f32, line_width: f32) -> Self {
        Self {
            builders: vec![builder],
            cur_layer: 0,
            cur_glyph_id: 0,
            glyph_paths: Vec::new(),
            base_transform: lyon::geom::euclid::Transform2D::identity(),
            scale_factor: 1.,
            offset_x: 0.,
            offset_y: 0.,
            tolerance,
            line_width,
        }
    }

    #[inline]
    pub fn cur_builder(&mut self) -> &mut T {
        &mut self.builders[self.cur_layer]
    }

    /// Finalize the current builder's path and store it with the current
    /// glyph ID. Called after each `glyph.draw()` in `draw_glyphs`.
    pub fn finish_glyph(&mut self) {
        self.finish_glyph_with_color(None);
    }

    /// Finalize the current builder's path and store it with a COLR color.
    pub fn finish_glyph_with_color(&mut self, color: Option<RgbaColor>) {
        let old = std::mem::replace(
            &mut self.builders[self.cur_layer],
            T::new_builder(self.tolerance),
        );
        let path = old.build();
        if path.iter().next().is_some() {
            self.glyph_paths.push((self.cur_glyph_id, path, color));
        }
        self.update_transform();
    }

    pub fn update_transform(&mut self) {
        let transform = self
            .base_transform
            .then_translate(lyon::geom::euclid::Vector2D::new(
                self.offset_x,
                self.offset_y,
            ))
            .then_scale(self.scale_factor, self.scale_factor);
        self.cur_builder().set_transform(transform);
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
        self.update_transform();
    }

    pub fn add_offset_x(&mut self, x: f32) {
        self.offset_x += x;
        self.update_transform();
    }

    pub fn add_offset_y(&mut self, y: f32) {
        self.offset_y += y;
        self.update_transform();
    }

    pub fn sub_offset_x(&mut self, x: f32) {
        self.offset_x -= x;
        self.update_transform();
    }

    pub fn sub_offset_y(&mut self, y: f32) {
        self.offset_y -= y;
        self.update_transform();
    }

    pub fn reset_offset_x(&mut self) {
        self.offset_x = 0.0;
        self.update_transform();
    }

    pub fn reset_offset_y(&mut self) {
        self.offset_y = 0.0;
        self.update_transform();
    }

    pub fn set_transform(
        &mut self,
        transform: lyon::geom::euclid::Transform2D<f32, UnknownUnit, UnknownUnit>,
    ) {
        self.base_transform = transform;
        self.update_transform();
    }
}

// For path

pub type FlattenedPathBuilder = lyon::path::builder::NoAttributes<
    lyon::path::builder::Transformed<
        lyon::path::builder::Flattened<lyon::path::path::BuilderImpl>,
        lyon::math::Transform,
    >,
>;

impl BuildPath for FlattenedPathBuilder {
    fn set_transform(&mut self, transform: lyon::math::Transform) {
        self.inner_mut().set_transform(transform);
    }

    fn new_builder(tolerance: f32) -> Self {
        lyon::path::Path::builder()
            .flattened(tolerance)
            .transformed(lyon::geom::euclid::Transform2D::identity())
    }
}

pub type LyonPathBuilderForPath = LyonPathBuilder<FlattenedPathBuilder>;

impl LyonPathBuilderForPath {
    pub fn new(tolerance: f32, line_width: f32) -> Self {
        let builder = FlattenedPathBuilder::new_builder(tolerance);
        Self::new_inner(builder, tolerance, line_width)
    }
}

// For stroke and fill

pub type NonFlattenedPathBuilder = lyon::path::builder::NoAttributes<
    lyon::path::builder::Transformed<lyon::path::path::BuilderImpl, lyon::math::Transform>,
>;

impl BuildPath for NonFlattenedPathBuilder {
    fn set_transform(&mut self, transform: lyon::math::Transform) {
        self.inner_mut().set_transform(transform);
    }

    fn new_builder(_tolerance: f32) -> Self {
        lyon::path::Path::builder().transformed(lyon::geom::euclid::Transform2D::identity())
    }
}

pub type LyonPathBuilderForStrokeAndFill = LyonPathBuilder<NonFlattenedPathBuilder>;

impl LyonPathBuilderForStrokeAndFill {
    pub fn new(tolerance: f32, line_width: f32) -> Self {
        let builder = NonFlattenedPathBuilder::new_builder(tolerance);
        Self::new_inner(builder, tolerance, line_width)
    }
}

// skrifa OutlinePen

impl<T: BuildPath> OutlinePen for LyonPathBuilder<T> {
    fn move_to(&mut self, x: f32, y: f32) {
        let at = point(x, y);
        self.cur_builder().begin(at, &[]);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let to = point(x, y);
        self.cur_builder().line_to(to, &[]);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let ctrl = point(cx0, cy0);
        let to = point(x, y);
        self.cur_builder().quadratic_bezier_to(ctrl, to, &[]);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let ctrl1 = point(cx0, cy0);
        let ctrl2 = point(cx1, cy1);
        let to = point(x, y);
        self.cur_builder().cubic_bezier_to(ctrl1, ctrl2, to, &[]);
    }

    fn close(&mut self) {
        self.cur_builder().end(true);
    }
}
