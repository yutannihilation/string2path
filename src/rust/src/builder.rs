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
    Face, RgbaColor,
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

    // index of builder
    pub cur_layer: usize,

    pub cur_glyph_id: u32,
    pub cur_path_id: u32,
    pub cur_color: RgbaColor,

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
            builders: vec![lyon::path::Path::builder_with_attributes(3)
                .transformed(lyon::geom::euclid::Transform2D::identity())],
            cur_layer: 0,
            cur_glyph_id: 0,
            cur_path_id: 0,
            cur_color: RgbaColor::new(0, 0, 0, 255),
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

    pub fn build(&mut self) -> Path {
        // TODO
        builder.build()
    }

    // adds offsets to x and y
    pub fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x, y)
    }

    pub fn ids(&self) -> [f32; 3] {
        let color = [
            self.cur_color.red,
            self.cur_color.green,
            self.cur_color.blue,
            self.cur_color.alpha,
        ];
        let color_f32 = f32::from_ne_bytes(color);
        [self.cur_glyph_id as _, self.cur_path_id as _, color_f32]
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

impl ttf_parser::OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        savvy::r_eprintln!("move_to");
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
        savvy::r_eprintln!("close");
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
        savvy::r_eprintln!("outline_glyph");
        self.face.outline_glyph(glyph_id, self.builder);
    }

    fn paint(&mut self, paint: Paint<'a>) {
        savvy::r_eprintln!("paint");
        match paint {
            Paint::Solid(rgba_color) => {
                self.builder.cur_color = rgba_color;
            }
            _ => {} // Paint::LinearGradient(linear_gradient) => {
                    //     let stop = linear_gradient
                    //         .stops(0, &[NormalizedCoordinate::default()])
                    //         .next();
                    //     if let Some(color) = stop {
                    //         self.builder.cur_color = color.color;
                    //     }
                    // }
                    // Paint::RadialGradient(radial_gradient) => {
                    //     let stop = radial_gradient
                    //         .stops(0, &[NormalizedCoordinate::default()])
                    //         .next();
                    //     if let Some(color) = stop {
                    //         self.builder.cur_color = color.color;
                    //     }
                    // }
                    // Paint::SweepGradient(sweep_gradient) => {
                    //     let stop = sweep_gradient
                    //         .stops(0, &[NormalizedCoordinate::default()])
                    //         .next();
                    //     if let Some(color) = stop {
                    //         self.builder.cur_color = color.color;
                    //     }
                    // }
        }
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
        // ignore
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
