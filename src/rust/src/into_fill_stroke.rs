use crate::{builder::LyonPathBuilderForStrokeAndFill, result::PathTibble};

use lyon::tessellation::*;
use ttf_parser::RgbaColor;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: lyon::math::Point,
    glyph_id: u32,
    path_id: u32,
}

// This can have some members so that it can be used in new_vertex(), but I
// don't find any useful usage yet.
struct VertexCtor {}

impl FillVertexConstructor<Vertex> for VertexCtor {
    fn new_vertex(&mut self, mut vertex: FillVertex) -> Vertex {
        let pos = vertex.position();
        let attr = vertex.interpolated_attributes();
        Vertex {
            position: pos,
            glyph_id: attr[0] as _,
            path_id: attr[1] as _,
        }
    }
}

impl StrokeVertexConstructor<Vertex> for VertexCtor {
    fn new_vertex(&mut self, mut vertex: StrokeVertex) -> Vertex {
        let pos = vertex.position();
        let attr = vertex.interpolated_attributes();
        Vertex {
            position: pos,
            glyph_id: attr[0] as _,
            path_id: attr[1] as _,
        }
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
            path_id: Vec::new(),
            triangle_id: Some(Vec::new()),
            color,
        };

        // Will contain the result of the tessellation.
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance);

        for (path, color) in paths {
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
            extract_vertex_buffer(geometry, &mut result, color);
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
            path_id: Vec::new(),
            triangle_id: Some(Vec::new()),
            color,
        };

        // Will contain the result of the tessellation.
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);
        for (path, color) in paths {
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

            extract_vertex_buffer(geometry, &mut result, color);
        }
        result
    }
}

fn extract_vertex_buffer(
    geometry: VertexBuffers<Vertex, usize>,
    dst: &mut PathTibble,
    paint_color: Option<RgbaColor>,
) {
    let offset = dst.triangle_id.as_ref().map_or(0, |v| match v.last() {
        Some(last_triangle_id) => last_triangle_id + 1,
        None => 0,
    });
    for (n, &i) in geometry.indices.iter().enumerate() {
        if let Some(v) = geometry.vertices.get(i) {
            dst.x.push(v.position.x as _);
            dst.y.push(v.position.y as _);
            dst.glyph_id.push(v.glyph_id as _);
            dst.path_id.push(v.path_id as _);
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
