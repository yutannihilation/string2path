use ttf_parser::RgbaColor;

use crate::builder::LyonPathBuilderForPath;
use crate::result::PathTibble;

impl LyonPathBuilderForPath {
    pub fn into_path(mut self) -> PathTibble {
        let paths = self.build();
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
            let paint_color = match paint_color {
                Some(RgbaColor {
                    red,
                    green,
                    blue,
                    alpha,
                }) => format!("#{red:02x}{green:02x}{blue:02x}{alpha:02x}",),
                None => "#00000000".to_string(),
            };
            let mut cur_path_id: u32 = 0;
            for p in path.iter() {
                let point = match p {
                    lyon::path::Event::Begin { at } => {
                        cur_path_id += 1;
                        Some(at)
                    }
                    lyon::path::Event::Line { to, .. } => Some(to),
                    lyon::path::Event::Quadratic { to, .. } => Some(to),
                    lyon::path::Event::Cubic { to, .. } => Some(to),
                    // glyph can be "open path," even when `close` is true. In that case, `first` should point to the begin point.
                    lyon::path::Event::End { last, first, close } => {
                        if close && last != first {
                            Some(first)
                        } else {
                            None
                        }
                    }
                };

                if let Some(pos) = point {
                    result.x.push(pos.x as _);
                    result.y.push(pos.y as _);
                    result
                        .glyph_id
                        .push(*self.glyph_id_map.get(&cur_path_id).unwrap_or(&0) as _);
                    result.path_id.push(cur_path_id as _);

                    if let Some(v) = result.color.as_mut() {
                        v.push(paint_color.clone())
                    }
                }
            }
        }
        result
    }
}
