use lyon::path::traits::Build;

use crate::builder::LyonPathBuilder;
use crate::result::PathTibble;

impl LyonPathBuilder {
    /// Extract the outline path to PathTibble.
    pub fn into_path(self) -> PathTibble {
        let path = self.builder.build();

        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        let mut glyph_id: Vec<i32> = Vec::new();
        let mut path_id: Vec<i32> = Vec::new();

        for p in path.iter_with_attributes() {
            match p {
                lyon::path::Event::Begin { at } => {
                    glyph_id.push(at.1[0] as _);
                    path_id.push(at.1[1] as _);
                    x.push(at.0.x as _);
                    y.push(at.0.y as _);
                }
                lyon::path::Event::Line { from, to } => {
                    glyph_id.push(from.1[0] as _);
                    path_id.push(from.1[1] as _);
                    x.push(to.0.x as _);
                    y.push(to.0.y as _);
                }
                lyon::path::Event::Quadratic { from, ctrl, to } => {
                    let seg = lyon::geom::QuadraticBezierSegment {
                        from: from.0,
                        ctrl,
                        to: to.0,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        glyph_id.push(from.1[0] as _);
                        path_id.push(from.1[1] as _);
                        x.push(p.x as _);
                        y.push(p.y as _);
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
                        glyph_id.push(from.1[0] as _);
                        path_id.push(from.1[1] as _);
                        x.push(p.x as _);
                        y.push(p.y as _);
                    }
                }
                // glyph can be "open path," even when `close` is true. In that case, `first` should point to the begin point.
                lyon::path::Event::End { last, first, close } => {
                    if close && last != first {
                        glyph_id.push(first.1[0] as _);
                        path_id.push(first.1[1] as _);
                        x.push(first.0.x as _);
                        y.push(first.0.y as _);
                    }
                }
            }
        }

        PathTibble {
            x,
            y,
            glyph_id,
            path_id,
            triangle_id: None,
        }
    }
}
