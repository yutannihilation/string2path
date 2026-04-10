use i_overlay::core::fill_rule::FillRule;
use i_overlay::float::simplify::SimplifyShape;

use crate::builder::LyonPathBuilderForPath;
use crate::result::PathTibble;

impl LyonPathBuilderForPath {
    pub fn into_path(self) -> PathTibble {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut glyph_id = Vec::new();
        let mut path_id = Vec::new();
        let mut out_path_id: u32 = 0;

        for (gid, glyph_path) in &self.glyph_paths {
            // Collect contours from the flattened path for this glyph.
            let mut contours: Vec<Vec<[f32; 2]>> = Vec::new();
            let mut cur_contour: Vec<[f32; 2]> = Vec::new();

            for event in glyph_path.iter() {
                match event {
                    lyon::path::Event::Begin { at } => {
                        cur_contour = vec![[at.x, at.y]];
                    }
                    lyon::path::Event::Line { to, .. } => {
                        cur_contour.push([to.x, to.y]);
                    }
                    lyon::path::Event::End { first, close, .. } => {
                        if close {
                            cur_contour.push([first.x, first.y]);
                        }
                        // i_overlay needs at least 3 distinct points to form a polygon.
                        if cur_contour.len() >= 3 {
                            contours.push(std::mem::take(&mut cur_contour));
                        } else {
                            cur_contour.clear();
                        }
                    }
                    // Quadratic / Cubic do not appear in a FlattenedPathBuilder output.
                    _ => {}
                }
            }

            if contours.is_empty() {
                continue;
            }

            // simplify_shape(NonZero) merges overlapping contours (e.g. components
            // of a composite glyph) while preserving counter-shapes (holes).
            let merged = contours.simplify_shape(FillRule::NonZero);

            for shape in merged {
                for contour in shape {
                    if contour.is_empty() {
                        continue;
                    }
                    out_path_id += 1;
                    let first = contour[0];
                    for pt in &contour {
                        x.push(pt[0] as f64);
                        y.push(pt[1] as f64);
                        glyph_id.push(*gid as i32);
                        path_id.push(out_path_id as i32);
                    }
                    // i_overlay returns implicitly-closed contours (no repeated first
                    // point). Append the first point again so geom_path() draws a
                    // closed polygon.
                    x.push(first[0] as f64);
                    y.push(first[1] as f64);
                    glyph_id.push(*gid as i32);
                    path_id.push(out_path_id as i32);
                }
            }
        }

        PathTibble {
            x,
            y,
            glyph_id,
            path_id: Some(path_id),
            triangle_id: None,
            color: None,
        }
    }
}
