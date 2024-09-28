use crate::{builder::LyonPathBuilderForStrokeAndFill, result::PathTibble};

use lyon::tessellation::*;
use ttf_parser::RgbaColor;

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

        // Will contain the result of the tessellation.
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance);

        let mut cur_path_id: u32 = 0;
        for (path, color) in paths {
            let path_id_inc = path
                .iter()
                .filter(|x| matches!(x, path::Event::Begin { .. }))
                .count();
            cur_path_id += path_id_inc as u32;

            let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
            {
                // Compute the tessellation.
                tessellator
                    .tessellate_path(
                        &path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                    )
                    .unwrap();
            }

            let cur_glyph_id = *self.glyph_id_map.get(&cur_path_id).unwrap_or(&0) as i32;
            extract_vertex_buffer(geometry, &mut result, color, cur_glyph_id);
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

        let mut cur_path_id: u32 = 0;

        // Will contain the result of the tessellation.
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);
        for (path, color) in paths {
            let path_id_inc = path
                .iter()
                .filter(|x| matches!(x, path::Event::Begin { .. }))
                .count();
            cur_path_id += path_id_inc as u32;

            let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
            {
                // Compute the tessellation.
                tessellator
                    .tessellate_path(
                        &path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                    )
                    .unwrap();
            }

            let cur_glyph_id = *self.glyph_id_map.get(&cur_path_id).unwrap_or(&0) as i32;
            extract_vertex_buffer(geometry, &mut result, color, cur_glyph_id);
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
