use extendr_api::prelude::*;

mod builder;
mod font;
mod into_fill_stroke;
mod into_path;
mod result;

enum ConversionType {
    Path,
    Stroke,
    Fill,
}

fn string2path_inner(
    text: &str,
    font_file: &str,
    tolerance: f32,
    line_width: f32,
    ct: ConversionType,
) -> Robj {
    let mut builder = builder::LyonPathBuilder::new(tolerance, line_width);

    builder.outline(text, font_file).unwrap();

    let result = match ct {
        ConversionType::Path => builder.into_path(),
        ConversionType::Stroke => builder.into_stroke(),
        ConversionType::Fill => builder.into_fill(),
    };

    result.try_into().unwrap()
}

#[extendr(use_try_from = true)]
fn string2path_impl(text: &str, font_file: &str, tolerance: f32) -> Robj {
    string2path_inner(text, font_file, tolerance, 0., ConversionType::Path)
}

#[extendr(use_try_from = true)]
fn string2stroke_impl(text: &str, font_file: &str, tolerance: f32, line_width: f32) -> Robj {
    string2path_inner(
        text,
        font_file,
        tolerance,
        line_width,
        ConversionType::Stroke,
    )
}

#[extendr(use_try_from = true)]
fn string2fill_impl(text: &str, font_file: &str, tolerance: f32) -> Robj {
    string2path_inner(text, font_file, tolerance, 0., ConversionType::Fill)
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
