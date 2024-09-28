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

pub struct LyonPathBuilder {
    // It's not very elegant to store the glyph ID (not the `glyphId` ttf-parser
    // uses, but the glyph count) and path ID in attributes, but it seems the
    // attribute is the only thing we can pass to tessellators.
    pub builders: Vec<
        lyon::path::builder::Transformed<
            lyon::path::path::BuilderWithAttributes,
            lyon::math::Transform,
        >,
    >,
    // one layer has only one color
    pub layer_color: HashMap<usize, RgbaColor>,

    // index of builder
    pub cur_layer: usize,

    pub cur_glyph_id: u32,
    pub cur_path_id: u32,

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

impl LyonPathBuilder {
    pub fn new(tolerance: f32, line_width: f32) -> Self {
        Self {
            builders: vec![lyon::path::Path::builder_with_attributes(2)
                .transformed(lyon::geom::euclid::Transform2D::identity())],
            layer_color: HashMap::new(),
            cur_layer: 0,
            cur_glyph_id: 0,
            cur_path_id: 0,
            base_transform: lyon::geom::euclid::Transform2D::identity(),
            scale_factor: 1.,
            offset_x: 0.,
            offset_y: 0.,
            tolerance,
            line_width,
        }
    }

    #[inline]
    pub fn cur_builder(
        &mut self,
    ) -> &mut lyon::path::builder::Transformed<
        lyon::path::path::BuilderWithAttributes,
        lyon::math::Transform,
    > {
        &mut self.builders[self.cur_layer]
    }

    fn build_inner(&mut self, tolerance: Option<f32>) -> Vec<(Path, Option<RgbaColor>)> {
        let builders = self.builders.drain(0..);
        builders
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                let path = match tolerance {
                    Some(tol) => x.flattened(tol).build(),
                    None => x.build(),
                };
                let color = self.layer_color.get(&i).cloned();
                (path, color)
            })
            .collect()
    }

    // for fill and stroke
    pub fn build(&mut self) -> Vec<(Path, Option<RgbaColor>)> {
        self.build_inner(None)
    }

    // for path
    pub fn build_flattened(&mut self, tolerance: f32) -> Vec<(Path, Option<RgbaColor>)> {
        self.build_inner(Some(tolerance))
    }

    // adds offsets to x and y
    pub fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x, y)
    }

    pub fn ids(&self) -> [f32; 2] {
        [self.cur_glyph_id as _, self.cur_path_id as _]
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
            self.builders.push(
                lyon::path::Path::builder_with_attributes(2)
                    .transformed(lyon::geom::euclid::Transform2D::identity()),
            );
        }
        self.update_transform();
    }

    // is this needed?
    //
    // fn pop_layer(&mut self) {
    //     self.cur_layer -= 1;
    // }
}

impl ttf_parser::OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let at = self.point(x, y);
        let custom_attributes = &self.ids();
        self.cur_builder().begin(at, custom_attributes);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let to = self.point(x, y);
        let custom_attributes = &self.ids();
        self.cur_builder().line_to(to, custom_attributes);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = self.point(x1, y1);
        let to = self.point(x, y);
        let custom_attributes = &self.ids();
        self.cur_builder()
            .quadratic_bezier_to(ctrl, to, custom_attributes);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = self.point(x1, y1);
        let ctrl2 = self.point(x2, y2);
        let to = self.point(x, y);
        let custom_attributes = &self.ids();
        self.cur_builder()
            .cubic_bezier_to(ctrl1, ctrl2, to, custom_attributes);
    }

    fn close(&mut self) {
        self.cur_builder().end(true);
        self.cur_path_id += 1;
    }
}

pub struct LyonPathBuilderForPaint<'a> {
    builder: &'a mut LyonPathBuilder,
    face: &'a Face<'a>,
}

impl<'a> LyonPathBuilderForPaint<'a> {
    pub fn new(builder: &'a mut LyonPathBuilder, face: &'a Face<'a>) -> Self {
        Self { builder, face }
    }
}

impl<'a> Painter<'a> for LyonPathBuilderForPaint<'a> {
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
