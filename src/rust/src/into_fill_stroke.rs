use crate::{builder::LyonPathBuilder, result::PathTibble};

use lyon::tessellation::*;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: lyon::math::Point,
    glyph_id: u32,
    path_id: u32,
    color: [u8; 4],
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
            color: attr[2].to_ne_bytes(),
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
            color: attr[2].to_ne_bytes(),
        }
    }
}

impl LyonPathBuilder {
    /// Convert the outline paths into fill as triangles.
    pub fn into_fill(mut self) -> PathTibble {
        let paths = self.build();
        let mut result = PathTibble {
            x: Vec::new(),
            y: Vec::new(),
            glyph_id: Vec::new(),
            path_id: Vec::new(),
            triangle_id: Some(Vec::new()),
            color: Some(Vec::new()),
        };

        // Will contain the result of the tessellation.
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance);

        for path in paths {
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
            extract_vertex_buffer(geometry, &mut result);
        }
        result
    }

    /// Convert the outline paths into stroke with a specified line width as triangles.
    pub fn into_stroke(mut self) -> PathTibble {
        let paths = self.build();
        let mut result = PathTibble {
            x: Vec::new(),
            y: Vec::new(),
            glyph_id: Vec::new(),
            path_id: Vec::new(),
            triangle_id: Some(Vec::new()),
            color: Some(Vec::new()),
        };

        // Will contain the result of the tessellation.
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);
        for path in paths {
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

            extract_vertex_buffer(geometry, &mut result);
        }
        result
    }
}

fn extract_vertex_buffer(geometry: VertexBuffers<Vertex, usize>, dst: &mut PathTibble) {
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
                let [r, g, b, a] = v.color;
                color.push(format!("#{r:02x}{g:02x}{b:02x}{a:02x}"));
            }
        }
    }
}
