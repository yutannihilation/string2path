use extendr_api::prelude::*;

mod builder;
mod font;
mod into_fill_stroke;
mod into_path;

#[extendr(use_try_from = true)]
fn string2path_impl(text: &str, font_file: &str, tolerance: f32) -> Robj {
    let mut builder = builder::LyonPathBuilder::new(tolerance, 0.);

    builder.outline(text, font_file).unwrap();

    let result = builder.into_path();

    data_frame!(
        x = result.0,
        y = result.1,
        glyph_id = result.2,
        path_id = result.3
    )
}

#[extendr(use_try_from = true)]
fn string2stroke_impl(text: &str, font_file: &str, tolerance: f32, line_width: f32) -> Robj {
    let mut builder = builder::LyonPathBuilder::new(tolerance, line_width);

    builder.outline(text, font_file).unwrap();

    let result = builder.into_stroke();

    data_frame!(
        x = result.0,
        y = result.1,
        glyph_id = result.2,
        path_id = result.3,
        triangle_id = result.4
    )
}

#[extendr(use_try_from = true)]
fn string2fill_impl(text: &str, font_file: &str, tolerance: f32) -> Robj {
    let mut builder = builder::LyonPathBuilder::new(tolerance, 0.);

    builder.outline(text, font_file).unwrap();

    let result = builder.into_fill();

    data_frame!(
        x = result.0,
        y = result.1,
        glyph_id = result.2,
        path_id = result.3,
        triangle_id = result.4
    )
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod string2path;
    fn string2path_impl;
    fn string2stroke_impl;
    fn string2fill_impl;
}
