use crate::{
    builder::{LyonPathBuilderForStrokeAndFill, RgbaColor},
    result::PathTibble,
};

use lyon::tessellation::*;

#[derive(Copy, Clone, Debug)]
struct Vertex(lyon::math::Point);

// This can have some members so that it can be used in new_vertex(), but I
// don't find any useful usage yet.
struct VertexCtor {}

impl FillVertexConstructor<Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex(vertex.position())
    }
}

impl StrokeVertexConstructor<Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex(vertex.position())
    }
}

/// Split a lyon `Path` (which may contain contours for multiple glyphs) into
/// per-glyph sub-paths using the `glyph_id_map` that maps path-id → glyph-id.
///
/// Returns `(glyph_id, sub_path)` pairs in drawing order, with all contours of
/// the same glyph merged into one `Path` so that the tessellator can correctly
/// handle holes (e.g. the counter of the letter "O").
fn split_path_by_glyph(
    path: &lyon::path::Path,
    glyph_id_map: &std::collections::HashMap<u32, u32>,
    cur_path_id: &mut u32,
) -> Vec<(i32, lyon::path::Path)> {
    let mut glyph_builders: Vec<(i32, lyon::path::path::Builder)> = Vec::new();

    for event in path.iter() {
        match event {
            path::Event::Begin { at } => {
                *cur_path_id += 1;
                let gid = *glyph_id_map.get(cur_path_id).unwrap_or(&0) as i32;

                // Start a new builder when the glyph changes.
                if glyph_builders.last().map_or(true, |(id, _)| *id != gid) {
                    glyph_builders.push((gid, lyon::path::Path::builder()));
                }
                glyph_builders.last_mut().unwrap().1.begin(at);
            }
            path::Event::Line { to, .. } => {
                if let Some((_, b)) = glyph_builders.last_mut() {
                    b.line_to(to);
                }
            }
            path::Event::Quadratic { ctrl, to, .. } => {
                if let Some((_, b)) = glyph_builders.last_mut() {
                    b.quadratic_bezier_to(ctrl, to);
                }
            }
            path::Event::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                if let Some((_, b)) = glyph_builders.last_mut() {
                    b.cubic_bezier_to(ctrl1, ctrl2, to);
                }
            }
            path::Event::End { close, .. } => {
                if let Some((_, b)) = glyph_builders.last_mut() {
                    b.end(close);
                }
            }
        }
    }

    glyph_builders
        .into_iter()
        .map(|(gid, b)| (gid, b.build()))
        .collect()
}

impl LyonPathBuilderForStrokeAndFill {
    /// Convert the outline paths into fill as triangles.
    pub fn into_fill(mut self) -> PathTibble {
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
            path_id: None,
            triangle_id: Some(Vec::new()),
            color,
        };

        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance).with_fill_rule(FillRule::NonZero);

        let mut cur_path_id: u32 = 0;
        for (path, color) in paths {
            let glyph_paths = split_path_by_glyph(&path, &self.glyph_id_map, &mut cur_path_id);

            for (cur_glyph_id, glyph_path) in glyph_paths {
                let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
                tessellator
                    .tessellate_path(
                        &glyph_path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                    )
                    .unwrap();
                extract_vertex_buffer(geometry, &mut result, color, cur_glyph_id);
            }
        }
        result
    }

    /// Convert the outline paths into stroke with a specified line width as triangles.
    pub fn into_stroke(mut self) -> PathTibble {
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
            path_id: None,
            triangle_id: Some(Vec::new()),
            color,
        };

        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);

        let mut cur_path_id: u32 = 0;
        for (path, color) in paths {
            let glyph_paths = split_path_by_glyph(&path, &self.glyph_id_map, &mut cur_path_id);

            for (cur_glyph_id, glyph_path) in glyph_paths {
                let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
                tessellator
                    .tessellate_path(
                        &glyph_path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                    )
                    .unwrap();
                extract_vertex_buffer(geometry, &mut result, color, cur_glyph_id);
            }
        }
        result
    }
}

fn extract_vertex_buffer(
    geometry: VertexBuffers<Vertex, usize>,
    dst: &mut PathTibble,
    paint_color: Option<RgbaColor>,
    glyph_id: i32,
) {
    let offset = dst.triangle_id.as_ref().map_or(0, |v| match v.last() {
        Some(last_triangle_id) => last_triangle_id + 1,
        None => 0,
    });
    for (n, &i) in geometry.indices.iter().enumerate() {
        if let Some(v) = geometry.vertices.get(i) {
            dst.x.push(v.0.x as _);
            dst.y.push(v.0.y as _);
            dst.glyph_id.push(glyph_id);
            if let Some(triangle_id) = &mut dst.triangle_id {
                triangle_id.push(n as i32 / 3 + offset);
            }

            if let Some(color) = &mut dst.color {
                let paint_color = match paint_color {
                    Some(RgbaColor {
                        red,
                        green,
                        blue,
                        alpha,
                    }) => format!("#{red:02x}{green:02x}{blue:02x}{alpha:02x}",),
                    None => "#00000000".to_string(),
                };
                color.push(paint_color);
            }
        }
    }
}
