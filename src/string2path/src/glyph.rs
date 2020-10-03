use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;
use path::Event::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_uint};

const HEIGHT: f32 = 10.0;
const DEFAULT_TOLERANCE: f64 = 0.1;

// A struct to pass the results from Rust to C
#[repr(C)]
pub struct Result {
    x: *mut c_double,
    y: *mut c_double,
    id: *mut c_uint,
    glyph_id: *mut c_uint,
    length: c_uint,
}

struct Point {
    x: f32,
    y: f32,
    id: u32,
    glyph_id: u32,
}

struct Builder {
    cur_path_id: u32,
    cur_glyph_id: u32,
    offset_x: f32,
    offset_y: f32,
    tolerance: f32,
    line_width: f32,
    builder: lyon::path::BuilderWithAttributes,
}

impl Builder {
    fn new(tolerance: f32, line_width: f32) -> Self {
        let builder = Path::builder_with_attributes(2);
        Self {
            cur_path_id: 0,
            cur_glyph_id: 0,
            offset_x: 0.0,
            offset_y: 0.0,
            tolerance,
            line_width,
            builder,
        }
    }

    // rusttype returns the positions to the origin, so we need to
    // move to each offsets by ourselves
    fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x + self.offset_x, y + self.offset_y)
    }

    fn next_glyph(&mut self, glyph_id: u32, bbox: &rusttype::Rect<i32>) {
        self.cur_glyph_id = glyph_id;
        self.offset_x = bbox.min.x as _;
        self.offset_y = bbox.min.y as _;
    }

    fn to_path(self, height: f32) -> Result {
        let path = self.builder.build();

        let mut points: Vec<Point> = vec![];

        for p in path.iter_with_attributes() {
            match p {
                Begin { at } => points.push(Point {
                    x: at.0.x,
                    y: at.0.y,
                    id: at.1[0] as _,
                    glyph_id: at.1[1] as _,
                }),
                Line { from, to } => points.push(Point {
                    x: to.0.x,
                    y: to.0.y,
                    id: from.1[0] as _,
                    glyph_id: from.1[1] as _,
                }),
                Quadratic { from, ctrl, to } => {
                    let seg = lyon::geom::QuadraticBezierSegment {
                        from: from.0,
                        ctrl: ctrl,
                        to: to.0,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        points.push(Point {
                            x: p.x,
                            y: p.y,
                            id: from.1[0] as _,
                            glyph_id: from.1[1] as _,
                        })
                    }
                }
                Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    let seg = lyon::geom::CubicBezierSegment {
                        from: from.0,
                        ctrl1: ctrl1,
                        ctrl2: ctrl2,
                        to: to.0,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        points.push(Point {
                            x: p.x,
                            y: p.y,
                            id: from.1[0] as _,
                            glyph_id: from.1[1] as _,
                        })
                    }
                }
                End { last, first, close } => points.push(Point {
                    x: last.0.x,
                    y: last.0.y,
                    id: last.1[0] as _,
                    glyph_id: last.1[1] as _,
                }),
            };
        }

        let length = points.len();
        let mut x_vec: Vec<c_double> = vec![0.0; length];
        let mut y_vec: Vec<c_double> = vec![0.0; length];
        let mut id_vec: Vec<c_uint> = vec![0; length];
        let mut glyph_id_vec: Vec<c_uint> = vec![0; length];

        for (i, p) in points.iter().enumerate() {
            x_vec[i] = p.x as _;
            y_vec[i] = (height - p.y) as _;
            id_vec[i] = p.id as _;
            glyph_id_vec[i] = p.glyph_id as _;
        }

        let x = x_vec.as_mut_ptr();
        std::mem::forget(x_vec);
        let y = y_vec.as_mut_ptr();
        std::mem::forget(y_vec);
        let id = id_vec.as_mut_ptr();
        std::mem::forget(id_vec);
        let glyph_id = glyph_id_vec.as_mut_ptr();
        std::mem::forget(glyph_id_vec);

        Result {
            x,
            y,
            id,
            glyph_id,
            length: length as _,
        }
    }

    fn to_stroke(self, height: f32) -> Result {
        let path = self.builder.build();

        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();

        {
            // Compute the tessellation.
            let res = tessellator.tessellate_path(
                &path,
                &StrokeOptions::tolerance(self.tolerance).with_line_width(self.line_width),
                &mut BuffersBuilder::new(
                    &mut geometry,
                    |pos: lyon::math::Point, mut attr: StrokeAttributes| {
                        let ids = attr.interpolated_attributes();
                        Point {
                            x: pos.x,
                            y: pos.y,
                            id: ids[0] as _,
                            glyph_id: ids[1] as _,
                        }
                    },
                ),
            );
            match res {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("tolerance: {:?}, error: {:?}", self.tolerance, e);
                    return null_result();
                }
            }
        }

        let length = geometry.indices.len();
        let mut x_vec: Vec<c_double> = vec![0.0; length];
        let mut y_vec: Vec<c_double> = vec![0.0; length];
        let mut id_vec: Vec<c_uint> = vec![0; length];
        let mut glyph_id_vec: Vec<c_uint> = vec![0; length];

        for (i, &idx) in geometry.indices.iter().enumerate() {
            let p = &geometry.vertices[idx as usize];
            x_vec[i] = p.x as _;
            y_vec[i] = (height - p.y) as _;
            // id_vec[i] = p.id as _;
            id_vec[i] = (i / 3) as _; // indices form triangles for each 3 ones
            glyph_id_vec[i] = p.glyph_id as _;
        }

        let x = x_vec.as_mut_ptr();
        std::mem::forget(x_vec);
        let y = y_vec.as_mut_ptr();
        std::mem::forget(y_vec);
        let id = id_vec.as_mut_ptr();
        std::mem::forget(id_vec);
        let glyph_id = glyph_id_vec.as_mut_ptr();
        std::mem::forget(glyph_id_vec);

        Result {
            x,
            y,
            id,
            glyph_id,
            length: length as _,
        }
    }

    fn to_vertex(self, height: f32) -> Result {
        let path = self.builder.build();

        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();

        {
            // Compute the tessellation.
            let res = tessellator.tessellate_path(
                &path,
                &FillOptions::tolerance(self.tolerance),
                &mut BuffersBuilder::new(
                    &mut geometry,
                    |pos: lyon::math::Point, mut attr: FillAttributes| {
                        let ids = attr.interpolated_attributes();
                        Point {
                            x: pos.x,
                            y: pos.y,
                            id: ids[0] as _,
                            glyph_id: ids[1] as _,
                        }
                    },
                ),
            );
            match res {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("tolerance: {:?}, error: {:?}", self.tolerance, e);
                    return null_result();
                }
            }
        }

        let length = geometry.indices.len();
        let mut x_vec: Vec<c_double> = vec![0.0; length];
        let mut y_vec: Vec<c_double> = vec![0.0; length];
        let mut id_vec: Vec<c_uint> = vec![0; length];
        let mut glyph_id_vec: Vec<c_uint> = vec![0; length];

        for (i, &idx) in geometry.indices.iter().enumerate() {
            let p = &geometry.vertices[idx as usize];
            x_vec[i] = p.x as _;
            y_vec[i] = (height - p.y) as _;
            // id_vec[i] = p.id as _;
            id_vec[i] = (i / 3) as _; // indices form triangles for each 3 ones
            glyph_id_vec[i] = p.glyph_id as _;
        }

        let x = x_vec.as_mut_ptr();
        std::mem::forget(x_vec);
        let y = y_vec.as_mut_ptr();
        std::mem::forget(y_vec);
        let id = id_vec.as_mut_ptr();
        std::mem::forget(id_vec);
        let glyph_id = glyph_id_vec.as_mut_ptr();
        std::mem::forget(glyph_id_vec);

        Result {
            x,
            y,
            id,
            glyph_id,
            length: length as _,
        }
    }
}

impl<'a> rusttype::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(
            self.point(x, y),
            &[self.cur_path_id as _, self.cur_glyph_id as _],
        );
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(
            self.point(x, y),
            &[self.cur_path_id as _, self.cur_glyph_id as _],
        );
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quadratic_bezier_to(
            self.point(x1, y1),
            self.point(x, y),
            &[self.cur_path_id as _, self.cur_glyph_id as _],
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_bezier_to(
            self.point(x1, y1),
            self.point(x2, y2),
            self.point(x, y),
            &[self.cur_path_id as _, self.cur_glyph_id as _],
        );
    }

    fn close(&mut self) {
        self.cur_path_id += 1;
        self.builder.close();
    }
}

// Utility function to convert c_char to string
fn c_char_to_string(c: *const c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() }
}

fn null_result() -> Result {
    let mut res: Vec<c_double> = vec![];
    let ptr = res.as_mut_ptr();
    std::mem::forget(res);
    Result {
        x: ptr,
        y: ptr,
        id: ptr as _,
        glyph_id: ptr as _,
        length: 0,
    }
}

#[no_mangle]
pub extern "C" fn string2path(
    c_str: *const c_char,
    c_ttf_file: *const c_char,
    tolerance: c_double,
    line_width: c_double,
    result_type: c_uint,
) -> Result {
    let str = c_char_to_string(c_str);

    let ttf_file = c_char_to_string(c_ttf_file);
    let font = {
        if let Ok(data) = std::fs::read(ttf_file.clone()) {
            rusttype::Font::try_from_vec(data).unwrap()
        } else {
            eprintln!("Failed to read {}", ttf_file);
            return null_result();
        }
    };

    // provided tolerance might be negative, so we need to handle it
    let tolerance_local = if tolerance <= 0.0 {
        eprintln!(
            "Warning: tolerance must be larger than 0: got {}",
            tolerance
        );
        DEFAULT_TOLERANCE
    } else {
        tolerance
    };

    let scale = rusttype::Scale::uniform(HEIGHT);
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(0.0, v_metrics.ascent);

    let q_glyph = font.layout(&str, scale, offset);

    let mut builder = Builder::new(tolerance_local as _, line_width as _);

    let mut bbox_y: Vec<i32> = vec![];

    for (glyph_id, g) in q_glyph.enumerate() {
        if let Some(bbox) = g.pixel_bounding_box() {
            bbox_y.push(bbox.max.y);
            builder.next_glyph(glyph_id as _, &bbox);
        } else {
            continue;
        }

        // println!("{:?}", g);
        if !g.build_outline(&mut builder) {
            println!("empty");
        }
    }

    let height = bbox_y.into_iter().max().unwrap_or(0);
    match result_type {
        0 => builder.to_vertex(height as _),
        1 => builder.to_stroke(height as _),
        _ => builder.to_path(height as _),
    }
}

#[no_mangle]
pub extern "C" fn free_result(res: Result) {
    free_vec(res.x, res.length as _);
    free_vec(res.y, res.length as _);
    free_vec(res.id, res.length as _);
}

fn free_vec<T>(data: *mut T, len: usize) {
    let s: &mut [T] = unsafe { std::slice::from_raw_parts_mut(data, len) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}
