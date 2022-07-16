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
    font_family: &str,
    font_weight: &str,
    tolerance: f32,
    line_width: f32,
    ct: ConversionType,
) -> Robj {
    let mut builder = builder::LyonPathBuilder::new(tolerance, line_width);

    builder.outline(text, font_family, font_weight).unwrap();

    let result = match ct {
        ConversionType::Path => builder.into_path(),
        ConversionType::Stroke => builder.into_stroke(),
        ConversionType::Fill => builder.into_fill(),
    };

    result.try_into().unwrap()
}

#[extendr(use_try_from = true)]
fn string2path_impl(text: &str, font_family: &str, font_weight: &str, tolerance: f32) -> Robj {
    string2path_inner(
        text,
        font_family,
        font_weight,
        tolerance,
        0.,
        ConversionType::Path,
    )
}

#[extendr(use_try_from = true)]
fn string2stroke_impl(
    text: &str,
    font_family: &str,
    font_weight: &str,
    tolerance: f32,
    line_width: f32,
) -> Robj {
    string2path_inner(
        text,
        font_family,
        font_weight,
        tolerance,
        line_width,
        ConversionType::Stroke,
    )
}

#[extendr(use_try_from = true)]
fn string2fill_impl(text: &str, font_family: &str, font_weight: &str, tolerance: f32) -> Robj {
    string2path_inner(
        text,
        font_family,
        font_weight,
        tolerance,
        0.,
        ConversionType::Fill,
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

#[cfg(test)]
mod tests {
    use crate::builder::LyonPathBuilder;

    #[test]
    fn test_path() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_path();

        // the height of the test.ttf is 125 (ascent 100 + descent 25)
        assert_eq!(result.x, vec![0., 100. / 125., 0., 0.]);
        assert_eq!(result.y, vec![0., 100. / 125., 100. / 125., 0.]);
    }

    #[test]
    fn test_stroke() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.2);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_stroke();

        assert!(result
            .x
            .iter()
            .any(|&i| 0. - 0.1 <= i && i <= 100. / 125. + 0.1));
        assert!(result
            .y
            .iter()
            .any(|&i| 0. - 0.1 <= i && i <= 100. / 125. + 0.1));
    }

    #[test]
    fn test_fill() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_fill();

        // TODO: Is this correct...?
        assert_eq!(result.x, vec![0., 0., 100. / 125.]);
        assert_eq!(result.y, vec![0., 100. / 125., 100. / 125.]);
    }
}
