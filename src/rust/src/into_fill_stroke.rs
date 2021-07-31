use crate::{builder::LyonPathBuilder, result::PathTibble};

use lyon::tessellation::*;

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

impl LyonPathBuilder {
    /// Convert the outline paths into fill as triangles.
    pub fn into_fill(self) -> PathTibble {
        let path = self.builder.build();

        // Will contain the result of the tessellation.
        let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance);

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

        extract_vertex_buffer(geometry)
    }

    /// Convert the outline paths into stroke with a specified line width as triangles.
    pub fn into_stroke(self) -> PathTibble {
        let path = self.builder.build();

        // Will contain the result of the tessellation.
        let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);

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

        extract_vertex_buffer(geometry)
    }
}

fn extract_vertex_buffer(geometry: VertexBuffers<Vertex, usize>) -> PathTibble {
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut glyph_id: Vec<u32> = Vec::new();
    let mut path_id: Vec<u32> = Vec::new();
    let mut triangle_id: Vec<u32> = Vec::new();

    for (n, &i) in geometry.indices.iter().enumerate() {
        if let Some(v) = geometry.vertices.get(i) {
            x.push(v.position.x);
            y.push(v.position.y);
            glyph_id.push(v.glyph_id);
            path_id.push(v.path_id);
            triangle_id.push(n as u32 / 3);
        }
    }

    PathTibble {
        x,
        y,
        glyph_id,
        path_id,
        triangle_id: Some(triangle_id),
    }
}
