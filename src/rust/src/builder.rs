use std::collections::HashMap;

use lyon::{
    geom::euclid::UnknownUnit,
    math::point,
    path::{
        traits::{Build, PathBuilder},
        Path,
    },
};
use ttf_parser::{
    colr::{Paint, Painter},
    Face, NormalizedCoordinate, RgbaColor,
};

pub trait BuildPath: Build<PathType = Path> + PathBuilder {
    // TODO: lyon::path::builder::Transformed is a struct, not a trait. So, this
    // method is needed to forward the operation.
    fn set_transform(&mut self, transform: lyon::math::Transform);
    fn new_builder(tolerance: f32) -> Self;
}

pub struct LyonPathBuilder<T: BuildPath> {
    // It's not very elegant to store the glyph ID (not the `glyphId` ttf-parser
    // uses, but the glyph count) and path ID in attributes, but it seems the
    // attribute is the only thing we can pass to tessellators.
    pub builders: Vec<T>,
    // one layer has only one color
    pub layer_color: HashMap<usize, RgbaColor>,

    // index of builder
    pub cur_layer: usize,

    pub cur_glyph_id: u32,
    pub cur_path_id: u32,

    // path ID to glyph ID
    pub glyph_id_map: HashMap<u32, u32>,

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
            layer_color: HashMap::new(),
            cur_layer: 0,
            cur_glyph_id: 0,
            cur_path_id: 0,
            glyph_id_map: HashMap::new(),
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

    pub fn build(&mut self) -> Vec<(Path, Option<RgbaColor>)> {
        let builders = self.builders.drain(0..);
        builders
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                let path = x.build();
                let color = self.layer_color.get(&i).cloned();
                (path, color)
            })
            .collect()
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

    fn push_layer(&mut self) {
        self.cur_layer += 1;
        if self.builders.len() < self.cur_layer + 1 {
            self.builders.push(T::new_builder(self.tolerance));
        }
        self.update_transform();
    }

    // is this needed?
    //
    // fn pop_layer(&mut self) {
    //     self.cur_layer -= 1;
    // }
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
        lyon::path::builder::Transformed::set_transform(self, transform);
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
        self.set_transform(transform);
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

// ttf-parser

impl<T: BuildPath> ttf_parser::OutlineBuilder for LyonPathBuilder<T> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.cur_path_id += 1;

        // While it's not very cool, path ID can be re-calculated later. So, if
        // the corresponding glyph ID is recorded here, it can be acquired when
        // constructing a result data frame.
        self.glyph_id_map
            .insert(self.cur_path_id, self.cur_glyph_id);

        let at = point(x, y);
        self.cur_builder().begin(at, &[]);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let to = point(x, y);
        self.cur_builder().line_to(to, &[]);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = point(x1, y1);
        let to = point(x, y);
        self.cur_builder().quadratic_bezier_to(ctrl, to, &[]);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = point(x1, y1);
        let ctrl2 = point(x2, y2);
        let to = point(x, y);
        self.cur_builder().cubic_bezier_to(ctrl1, ctrl2, to, &[]);
    }

    fn close(&mut self) {
        self.cur_builder().end(true);
    }
}

pub struct LyonPathBuilderForPaint<'a, T: BuildPath> {
    builder: &'a mut LyonPathBuilder<T>,
    face: &'a Face<'a>,
}

impl<'a, T: BuildPath> LyonPathBuilderForPaint<'a, T> {
    pub fn new(builder: &'a mut LyonPathBuilder<T>, face: &'a Face<'a>) -> Self {
        Self { builder, face }
    }
}

impl<'a, T: BuildPath> Painter<'a> for LyonPathBuilderForPaint<'a, T> {
    fn outline_glyph(&mut self, glyph_id: ttf_parser::GlyphId) {
        self.face.outline_glyph(glyph_id, self.builder);
    }

    fn paint(&mut self, paint: Paint<'a>) {
        let color = match paint {
            Paint::Solid(rgba_color) => rgba_color,
            Paint::LinearGradient(linear_gradient) => {
                let stop = linear_gradient
                    .stops(0, &[NormalizedCoordinate::default()])
                    .next();
                stop.map_or(RgbaColor::new(0, 0, 0, 255), |cs| cs.color)
            }
            Paint::RadialGradient(radial_gradient) => {
                let stop = radial_gradient
                    .stops(0, &[NormalizedCoordinate::default()])
                    .next();
                stop.map_or(RgbaColor::new(0, 0, 0, 255), |cs| cs.color)
            }
            Paint::SweepGradient(sweep_gradient) => {
                let stop = sweep_gradient
                    .stops(0, &[NormalizedCoordinate::default()])
                    .next();
                stop.map_or(RgbaColor::new(0, 0, 0, 255), |cs| cs.color)
            }
        };
        self.builder
            .layer_color
            .insert(self.builder.cur_layer, color);
        self.builder.push_layer();
    }

    fn push_clip(&mut self) {
        // ignore
    }

    fn push_clip_box(&mut self, _clipbox: ttf_parser::colr::ClipBox) {
        // ignore
    }

    fn pop_clip(&mut self) {
        // ignore
    }

    fn push_layer(&mut self, _mode: ttf_parser::colr::CompositeMode) {
        // ignore. Only paint can push layer
    }

    fn pop_layer(&mut self) {
        // ignore
    }

    fn push_transform(&mut self, transform: ttf_parser::Transform) {
        let transform = lyon::geom::euclid::Transform2D::new(
            // cf. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-12-and-13-painttransform-paintvartransform
            transform.a, // xx
            transform.b, // yx
            transform.c, // xy
            transform.d, // yy
            transform.e, // dx
            transform.f, // dy
        );
        self.builder.set_transform(transform);
    }

    fn pop_transform(&mut self) {
        self.builder
            .set_transform(lyon::geom::euclid::Transform2D::identity());
    }
}
