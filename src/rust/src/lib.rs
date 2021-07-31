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

#[cfg(test)]
mod tests {
    use crate::builder::LyonPathBuilder;

    #[test]
    fn test_path() {
        let mut builder = LyonPathBuilder::new(0.01, 0.);
        builder.outline("A", "test/font/test.ttf").unwrap();
        let result = builder.into_path();

        assert_eq!(result.x, vec![0., 100., 0., 0.]);
        assert_eq!(result.y, vec![0., 100., 100., 0.]);
    }

    #[test]
    fn test_stroke() {
        let mut builder = LyonPathBuilder::new(0.01, 50.);
        builder.outline("A", "test/font/test.ttf").unwrap();
        let result = builder.into_stroke();

        assert!(result.x.iter().any(|&i| 0. - 25.0 <= i && i <= 100. + 25.0));
        assert!(result.y.iter().any(|&i| 0. - 25.0 <= i && i <= 100. + 25.0));
    }

    #[test]
    fn test_fill() {
        let mut builder = LyonPathBuilder::new(0.01, 0.);
        builder.outline("A", "test/font/test.ttf").unwrap();
        let result = builder.into_fill();

        assert_eq!(result.x, vec![0., 100., 0., 0.]);
        assert_eq!(result.y, vec![0., 100., 100., 0.]);
    }
}
