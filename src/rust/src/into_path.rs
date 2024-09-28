use ttf_parser::RgbaColor;

use crate::builder::LyonPathBuilder;
use crate::result::PathTibble;

impl LyonPathBuilder {
    pub fn into_path2(mut self) {
        let paths = self.build_flattened(self.tolerance);
        let color = if self.layer_color.is_empty() {
            None
        } else {
            Some(Vec::new())
        };
        let mut result = PathTibble {
            x: Vec::new(),
            y: Vec::new(),
            glyph_id: Vec::new(),
            path_id: Vec::new(),
            triangle_id: None,
            color,
        };
        for (path, paint_color) in paths {
            // TODO
        }
    }

    /// Extract the outline path to PathTibble.
    pub fn into_path(mut self) -> PathTibble {
        let paths = self.build();

        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        let mut glyph_id: Vec<i32> = Vec::new();
        let mut path_id: Vec<i32> = Vec::new();
        let mut color: Option<Vec<String>> = if self.layer_color.is_empty() {
            None
        } else {
            Some(Vec::new())
        };

        for (path, paint_color) in paths {
            let paint_color = match paint_color {
                Some(RgbaColor {
                    red,
                    green,
                    blue,
                    alpha,
                }) => format!("#{red:02x}{green:02x}{blue:02x}{alpha:02x}",),
                None => "#00000000".to_string(),
            };
            for p in path.iter_with_attributes() {
                match p {
                    lyon::path::Event::Begin { at } => {
                        glyph_id.push(at.1[0] as _);
                        path_id.push(at.1[1] as _);
                        x.push(at.0.x as _);
                        y.push(at.0.y as _);
                        if let Some(v) = color.as_mut() {
                            v.push(paint_color.clone())
                        }
                    }
                    lyon::path::Event::Line { from, to } => {
                        glyph_id.push(from.1[0] as _);
                        path_id.push(from.1[1] as _);
                        x.push(to.0.x as _);
                        y.push(to.0.y as _);
                        if let Some(v) = color.as_mut() {
                            v.push(paint_color.clone())
                        }
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
                            if let Some(v) = color.as_mut() {
                                v.push(paint_color.clone())
                            }
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
                            if let Some(v) = color.as_mut() {
                                v.push(paint_color.clone())
                            }
                        }
                    }
                    // glyph can be "open path," even when `close` is true. In that case, `first` should point to the begin point.
                    lyon::path::Event::End { last, first, close } => {
                        if close && last != first {
                            glyph_id.push(first.1[0] as _);
                            path_id.push(first.1[1] as _);
                            x.push(first.0.x as _);
                            y.push(first.0.y as _);
                            if let Some(v) = color.as_mut() {
                                v.push(paint_color.clone())
                            }
                        }
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
            color,
        }
    }
}
