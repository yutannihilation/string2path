use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_uint};

const HEIGHT: f32 = 10.0;

// A struct to pass the results from Rust to C
#[repr(C)]
pub struct Result {
    x: *mut c_double,
    y: *mut c_double,
    id: *mut c_uint,
    length: c_uint,
}

struct Point {
    x: f32,
    y: f32,
    id: u32,
}

struct Builder {
    cur_path_id: u32,
    base_position: rusttype::Point<f32>,
    tolerance: f32,
    points_cur_glyph: Vec<Point>,
    points: Vec<Point>,
}

impl Builder {
    fn new(tolerance: f32) -> Self {
        Self {
            cur_path_id: 0,
            base_position: rusttype::point(0.0, 0.0),
            points_cur_glyph: vec![],
            points: vec![],
            tolerance,
        }
    }

    fn finish_cur_glyph(&mut self) {
        if self.points_cur_glyph.len() > 0 {
            let init_y = self.points_cur_glyph.first().unwrap().y;
            let mut y_range = [init_y, init_y];
            y_range = self.points_cur_glyph.iter().fold(y_range, |mut sum, p| {
                if p.y < sum[0] {
                    sum[0] = p.y;
                }
                if p.y > sum[1] {
                    sum[1] = p.y;
                }
                sum
            });

            self.points.append(
                &mut self
                    .points_cur_glyph
                    .iter()
                    .map(|p| {
                        // reverse and move to zero
                        let y_reverse = (y_range[1] - y_range[0])
                            * (1.0 - (p.y - y_range[0]) / (y_range[1] - y_range[0]));
                        Point {
                            x: p.x + self.base_position.x,
                            y: y_reverse,
                            id: p.id,
                        }
                    })
                    .collect(),
            );
            self.points_cur_glyph.clear();
        }
    }

    fn next_glyph(&mut self, position: &rusttype::Point<f32>) {
        self.finish_cur_glyph();
        self.base_position = position.clone();
    }

    fn add_point(&mut self, x: f32, y: f32) {
        self.points_cur_glyph.push(Point {
            x,
            y,
            id: self.cur_path_id,
        });
    }

    // fn to_path(mut self) -> Vec<[f32; 3]> {
    fn to_path(mut self) -> Result {
        self.finish_cur_glyph();

        let length = self.points.len();
        let mut x_vec: Vec<c_double> = vec![0.0; length];
        let mut y_vec: Vec<c_double> = vec![0.0; length];
        let mut id_vec: Vec<c_uint> = vec![0; length];

        for (i, p) in self.points.iter().enumerate() {
            x_vec[i] = p.x as _;
            y_vec[i] = p.y as _;
            id_vec[i] = p.id as _;
        }

        let x = x_vec.as_mut_ptr();
        std::mem::forget(x_vec);
        let y = y_vec.as_mut_ptr();
        std::mem::forget(y_vec);
        let id = id_vec.as_mut_ptr();
        std::mem::forget(id_vec);

        Result {
            x,
            y,
            id,
            length: length as _,
        }
    }
}

impl<'a> rusttype::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.add_point(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.add_point(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let cur = self.points_cur_glyph.last().unwrap();
        let segment = lyon::geom::QuadraticBezierSegment {
            from: lyon::math::point(cur.x, cur.y),
            ctrl: lyon::math::point(x1, y1),
            to: lyon::math::point(x, y),
        };
        for p in segment.flattened(self.tolerance) {
            self.add_point(p.x, p.y);
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let cur = self.points_cur_glyph.last().unwrap();
        let segment = lyon::geom::CubicBezierSegment {
            from: lyon::math::point(cur.x, cur.y),
            ctrl1: lyon::math::point(x1, y1),
            ctrl2: lyon::math::point(x2, y2),
            to: lyon::math::point(x, y),
        };
        for p in segment.flattened(self.tolerance) {
            self.add_point(p.x, p.y);
        }
    }

    fn close(&mut self) {
        self.cur_path_id += 1;
    }
}

// Utility function to convert c_char to string
fn c_char_to_string(c: *const c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() }
}

#[no_mangle]
pub extern "C" fn string2path(c_str: *const c_char, c_ttf_file: *const c_char) -> Result {
    let str = c_char_to_string(c_str);

    let ttf_file = c_char_to_string(c_ttf_file);
    let font = {
        if let Ok(data) = std::fs::read(ttf_file.clone()) {
            rusttype::Font::try_from_vec(data).unwrap()
        } else {
            eprintln!("Failed to read {}", ttf_file);
            let mut res: Vec<c_double> = vec![];
            let ptr = res.as_mut_ptr();
            std::mem::forget(res);
            return Result {
                x: ptr,
                y: ptr,
                id: ptr as _,
                length: 0,
            };
        }
    };

    let scale = rusttype::Scale::uniform(HEIGHT);
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(0.0, v_metrics.ascent);

    let q_glyph = font.layout(&str, scale, offset);

    let mut builder = Builder::new(0.0001);
    for g in q_glyph {
        builder.next_glyph(&g.position());
        // println!("{:?}", g);
        if !g.build_outline(&mut builder) {
            println!("empty");
        }
    }
    builder.to_path()
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
