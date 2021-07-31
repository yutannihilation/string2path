use crate::builder::LyonPathBuilder;

impl LyonPathBuilder {
    // returns `(x, y, glyphId, pathId)`
    pub fn into_path(self) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<u32>) {
        let path = self.builder.build();

        let mut x: Vec<f32> = Vec::new();
        let mut y: Vec<f32> = Vec::new();
        let mut glyph_ids: Vec<u32> = Vec::new();
        let mut path_ids: Vec<u32> = Vec::new();

        for p in path.iter_with_attributes() {
            match p {
                lyon::path::Event::Begin { at } => {
                    glyph_ids.push(at.1[0] as _);
                    path_ids.push(at.1[1] as _);
                    x.push(at.0.x);
                    y.push(at.0.y);
                }
                lyon::path::Event::Line { from, to } => {
                    glyph_ids.push(from.1[0] as _);
                    path_ids.push(from.1[1] as _);
                    x.push(to.0.x);
                    y.push(to.0.y);
                }
                lyon::path::Event::Quadratic { from, ctrl, to } => {
                    let seg = lyon::geom::QuadraticBezierSegment {
                        from: from.0,
                        ctrl,
                        to: to.0,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        glyph_ids.push(from.1[0] as _);
                        path_ids.push(from.1[1] as _);
                        x.push(p.x);
                        y.push(p.y);
                    }
                }
                lyon::path::Event::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    let seg = lyon::geom::CubicBezierSegment {
                        from: from.0,
                        ctrl1,
                        ctrl2,
                        to: to.0,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        glyph_ids.push(from.1[0] as _);
                        path_ids.push(from.1[1] as _);
                        x.push(p.x);
                        y.push(p.y);
                    }
                }
                lyon::path::Event::End { .. } => {}
            }
        }

        (x, y, glyph_ids, path_ids)
    }
}
