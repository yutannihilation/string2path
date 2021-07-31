use lyon::math::point;

pub struct LyonPathBuilder {
    // It's not very elegant to store the glyph ID (not the `glyphId` ttf-parser
    // uses, but the glyph count) and path ID in attributes, but it seems the
    // attribute is the only thing we can pass to tessellators.
    pub builder: lyon::path::path::BuilderWithAttributes,

    pub cur_glyph_id: u32,
    pub cur_path_id: u32,

    pub offset_x: f32,
    pub offset_y: f32,

    pub tolerance: f32,

    // line width of the stroke
    pub line_width: f32,
}

impl LyonPathBuilder {
    pub fn new(tolerance: f32, line_width: f32) -> Self {
        Self {
            builder: lyon::path::Path::builder_with_attributes(2),
            cur_glyph_id: 0,
            cur_path_id: 0,
            offset_x: 0.,
            offset_y: 0.,
            tolerance,
            line_width,
        }
    }

    // adds offsets to x and y
    pub(crate) fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x + self.offset_x, y + self.offset_y)
    }

    pub(crate) fn ids(&self) -> [f32; 2] {
        [self.cur_glyph_id as _, self.cur_path_id as _]
    }
}

impl ttf_parser::OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin(self.point(x, y), &self.ids());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(self.point(x, y), &self.ids());
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = self.point(x1, y1);
        let to = self.point(x, y);
        self.builder.quadratic_bezier_to(ctrl, to, &self.ids());
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = self.point(x1, y1);
        let ctrl2 = self.point(x2, y2);
        let to = self.point(x, y);
        self.builder.cubic_bezier_to(ctrl1, ctrl2, to, &self.ids());
    }

    fn close(&mut self) {
        self.builder.close();
        self.cur_path_id += 1;
    }
}
