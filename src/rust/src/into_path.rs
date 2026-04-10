use std::collections::HashMap;

use i_overlay::core::fill_rule::FillRule;
use i_overlay::float::simplify::SimplifyShape;

use crate::builder::LyonPathBuilderForPath;
use crate::result::PathTibble;

impl LyonPathBuilderForPath {
    pub fn into_path(mut self) -> PathTibble {
        let paths = self.build();

        // color support is deferred until COLR is implemented
        let color = if self.layer_color.is_empty() {
            None
        } else {
            Some(Vec::new())
        };

        // --- Step 1: collect contours from the flattened path, grouped by glyph_id ---
        //
        // FlattenedPathBuilder converts all curves to line segments, so only
        // Begin / Line / End events appear here (no Quadratic or Cubic).

        let mut contours_by_glyph: HashMap<u32, Vec<Vec<[f32; 2]>>> = HashMap::new();
        // glyph_order preserves the left-to-right drawing order.
        let mut glyph_order: Vec<u32> = Vec::new();
        let mut cur_path_id: u32 = 0;
        let mut cur_contour: Vec<[f32; 2]> = Vec::new();

        for (path, _paint_color) in &paths {
            for event in path.iter() {
                match event {
                    lyon::path::Event::Begin { at } => {
                        cur_path_id += 1;
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
                            let gid = *self.glyph_id_map.get(&cur_path_id).unwrap_or(&0);
                            if !glyph_order.contains(&gid) {
                                glyph_order.push(gid);
                            }
                            contours_by_glyph
                                .entry(gid)
                                .or_default()
                                .push(std::mem::take(&mut cur_contour));
                        } else {
                            cur_contour.clear();
                        }
                    }
                    // Quadratic / Cubic do not appear in a FlattenedPathBuilder output.
                    _ => {}
                }
            }
        }

        // --- Step 2 & 3: union each glyph's contours and rebuild output ---
        //
        // simplify_shape(NonZero) merges overlapping contours (e.g. components of
        // a variable-font composite glyph) while preserving counter-shapes (holes).
        // The result is Vec<Shape> = Vec<Vec<Vec<[f32;2]>>>.
        //   outer Vec  – independent shapes (usually 1 per glyph)
        //   middle Vec – contours within a shape (outer boundary + holes)
        //   inner Vec  – points of one contour

        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut glyph_id = Vec::new();
        let mut path_id = Vec::new();
        let mut out_path_id: u32 = 0;

        for gid in &glyph_order {
            let Some(contours) = contours_by_glyph.remove(gid) else {
                continue;
            };

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
            color,
        }
    }
}
