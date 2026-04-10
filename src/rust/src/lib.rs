use font::FONT_COLLECTION;
use result::FontDBTibble;
use savvy::savvy;

pub mod builder;
pub mod font;
pub mod into_fill_stroke;
pub mod into_path;
pub mod result;

enum ConversionType {
    Path,
    Stroke,
    Fill,
}

fn string2any_family(
    text: &str,
    font_family: &str,
    font_weight: &str,
    font_style: &str,
    tolerance: f64,
    line_width: f64,
    ct: ConversionType,
) -> savvy::Result<savvy::Sexp> {
    let result = match ct {
        ConversionType::Path => {
            let mut builder = builder::LyonPathBuilderForPath::new(tolerance as _, line_width as _);
            builder.outline(text, font_family, font_weight, font_style)?;
            builder.into_path()
        }
        ConversionType::Stroke | ConversionType::Fill => {
            let mut builder =
                builder::LyonPathBuilderForStrokeAndFill::new(tolerance as _, line_width as _);
            builder.outline(text, font_family, font_weight, font_style)?;
            if matches!(ct, ConversionType::Stroke) {
                builder.into_stroke()
            } else {
                builder.into_fill()
            }
        }
    };

    result.try_into()
}

fn string2any_file(
    text: &str,
    font_file: &str,
    tolerance: f64,
    line_width: f64,
    ct: ConversionType,
) -> savvy::Result<savvy::Sexp> {
    let result = match ct {
        ConversionType::Path => {
            let mut builder = builder::LyonPathBuilderForPath::new(tolerance as _, line_width as _);
            builder.outline_from_file(text, font_file)?;
            builder.into_path()
        }
        ConversionType::Stroke | ConversionType::Fill => {
            let mut builder =
                builder::LyonPathBuilderForStrokeAndFill::new(tolerance as _, line_width as _);
            builder.outline_from_file(text, font_file)?;
            if matches!(ct, ConversionType::Stroke) {
                builder.into_stroke()
            } else {
                builder.into_fill()
            }
        }
    };

    result.try_into()
}

#[savvy]
fn string2path_family(
    text: &str,
    font_family: &str,
    font_weight: &str,
    font_style: &str,
    tolerance: f64,
) -> savvy::Result<savvy::Sexp> {
    string2any_family(
        text,
        font_family,
        font_weight,
        font_style,
        tolerance,
        0.,
        ConversionType::Path,
    )
}

#[savvy]
fn string2path_file(text: &str, font_file: &str, tolerance: f64) -> savvy::Result<savvy::Sexp> {
    string2any_file(text, font_file, tolerance, 0., ConversionType::Path)
}

#[savvy]
fn string2stroke_family(
    text: &str,
    font_family: &str,
    font_weight: &str,
    font_style: &str,
    tolerance: f64,
    line_width: f64,
) -> savvy::Result<savvy::Sexp> {
    string2any_family(
        text,
        font_family,
        font_weight,
        font_style,
        tolerance,
        line_width,
        ConversionType::Stroke,
    )
}

#[savvy]
fn string2stroke_file(
    text: &str,
    font_file: &str,
    tolerance: f64,
    line_width: f64,
) -> savvy::Result<savvy::Sexp> {
    string2any_file(
        text,
        font_file,
        tolerance,
        line_width,
        ConversionType::Stroke,
    )
}

#[savvy]
fn string2fill_family(
    text: &str,
    font_family: &str,
    font_weight: &str,
    font_style: &str,
    tolerance: f64,
) -> savvy::Result<savvy::Sexp> {
    string2any_family(
        text,
        font_family,
        font_weight,
        font_style,
        tolerance,
        0.,
        ConversionType::Fill,
    )
}

#[savvy]
fn string2fill_file(text: &str, font_file: &str, tolerance: f64) -> savvy::Result<savvy::Sexp> {
    string2any_file(text, font_file, tolerance, 0., ConversionType::Fill)
}

#[savvy]
fn dump_fontdb_impl() -> savvy::Result<savvy::Sexp> {
    let mut source: Vec<String> = Vec::new();
    let mut index: Vec<i32> = Vec::new();
    let mut family: Vec<String> = Vec::new();
    let mut weight: Vec<String> = Vec::new();
    let mut style: Vec<String> = Vec::new();

    let mut collection = FONT_COLLECTION.lock().unwrap();

    // Collect all family names first to avoid borrow conflicts during lookup.
    let family_names: Vec<String> = collection.family_names().map(|s| s.to_string()).collect();

    for name in &family_names {
        let Some(fam) = collection.family_by_name(name) else {
            continue;
        };

        for font_info in fam.fonts() {
            // TODO: fontique::FontInfo does not expose the source file path in the
            // same way fontdb did. Using a placeholder for now.
            source.push("(system font)".to_string());
            index.push(font_info.index() as i32);
            family.push(name.clone());

            #[rustfmt::skip]
            weight.push(match font_info.weight().value() as u32 {
                100 => "thin",
                200 => "extra_light",
                300 => "light",
                400 => "normal",
                500 => "medium",
                600 => "semibold",
                700 => "bold",
                800 => "extra_bold",
                900 => "black",
                _   => "unknown",
            }.to_string());

            style.push(
                match font_info.style() {
                    fontique::FontStyle::Normal => "normal",
                    fontique::FontStyle::Italic => "italic",
                    fontique::FontStyle::Oblique(_) => "oblique",
                }
                .to_string(),
            );
        }
    }

    let result = FontDBTibble {
        source,
        index,
        family,
        weight,
        style,
    };

    result.try_into()
}

#[cfg(feature = "savvy_test")]
mod tests {
    use crate::builder::LyonPathBuilder;

    #[test]
    fn test_path() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_path();

        assert!(
            result
                .x
                .iter()
                .zip(vec![0., 100. / 125., 0., 0.])
                .all(|(actual, expect)| (expect - actual).abs() < f64::EPSILON.sqrt())
        );
        assert!(
            result
                .y
                .iter()
                .zip(vec![0., 100. / 125., 100. / 125., 0.])
                .all(|(actual, expect)| (expect - actual).abs() < 0.0001)
        );
    }

    #[test]
    fn test_stroke() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.2);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_stroke();

        assert!(
            result
                .x
                .iter()
                .any(|&i| (0. - 0.1..=100. / 125. + 0.1).contains(&i))
        );
        assert!(
            result
                .y
                .iter()
                .any(|&i| (0. - 0.1..=100. / 125. + 0.1).contains(&i))
        );
    }

    #[test]
    fn test_fill() {
        let mut builder = LyonPathBuilder::new(0.00001, 0.);
        builder
            .outline_from_file("A", "test/font/test.ttf")
            .unwrap();
        let result = builder.into_fill();

        // TODO: Is this correct...?
        assert!(
            result
                .x
                .iter()
                .zip(vec![0., 0., 100. / 125.])
                .all(|(actual, expect)| (expect - actual).abs() < f64::EPSILON.sqrt())
        );
        assert!(
            result
                .y
                .iter()
                .zip(vec![0., 100. / 125., 100. / 125.])
                .all(|(actual, expect)| (expect - actual).abs() < f64::EPSILON.sqrt())
        );
    }
}
