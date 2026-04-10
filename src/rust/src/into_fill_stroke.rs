use crate::{
    builder::{LyonPathBuilderForStrokeAndFill, RgbaColor, color_to_hex},
    result::PathTibble,
};

use lyon::tessellation::*;

#[derive(Copy, Clone, Debug)]
struct Vertex(lyon::math::Point);

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
    pub fn into_fill(self) -> PathTibble {
        let has_color = self.glyph_paths.iter().any(|(_, _, c)| c.is_some());
        let mut result = PathTibble {
            x: Vec::new(),
            y: Vec::new(),
            glyph_id: Vec::new(),
            path_id: None,
            triangle_id: Some(Vec::new()),
            color: if has_color { Some(Vec::new()) } else { None },
        };

        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance).with_fill_rule(FillRule::NonZero);

        for (glyph_id, glyph_path, color) in &self.glyph_paths {
            let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
            tessellator
                .tessellate_path(
                    glyph_path,
                    &options,
                    &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                )
                .unwrap();
            extract_vertex_buffer(geometry, &mut result, *glyph_id as i32, *color);
        }
        result
    }

    /// Convert the outline paths into stroke with a specified line width as triangles.
    pub fn into_stroke(self) -> PathTibble {
        let has_color = self.glyph_paths.iter().any(|(_, _, c)| c.is_some());
        let mut result = PathTibble {
            x: Vec::new(),
            y: Vec::new(),
            glyph_id: Vec::new(),
            path_id: None,
            triangle_id: Some(Vec::new()),
            color: if has_color { Some(Vec::new()) } else { None },
        };

        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width);

        for (glyph_id, glyph_path, color) in &self.glyph_paths {
            let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
            tessellator
                .tessellate_path(
                    glyph_path,
                    &options,
                    &mut BuffersBuilder::new(&mut geometry, VertexCtor {}),
                )
                .unwrap();
            extract_vertex_buffer(geometry, &mut result, *glyph_id as i32, *color);
        }
        result
    }
}

fn extract_vertex_buffer(
    geometry: VertexBuffers<Vertex, usize>,
    dst: &mut PathTibble,
    glyph_id: i32,
    paint_color: Option<RgbaColor>,
) {
    let offset = dst.triangle_id.as_ref().map_or(0, |v| match v.last() {
        Some(last_triangle_id) => last_triangle_id + 1,
        None => 0,
    });
    let color_str = dst.color.as_ref().map(|_| color_to_hex(paint_color));
    for (n, &i) in geometry.indices.iter().enumerate() {
        if let Some(v) = geometry.vertices.get(i) {
            dst.x.push(v.0.x as _);
            dst.y.push(v.0.y as _);
            dst.glyph_id.push(glyph_id);
            if let Some(triangle_id) = &mut dst.triangle_id {
                triangle_id.push(n as i32 / 3 + offset);
            }
            if let Some(color) = &mut dst.color {
                color.push(color_str.as_ref().unwrap().clone());
            }
        }
    }
}
