use font::FONTDB;
use result::FontDBTibble;
use savvy::savvy;

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

fn string2any_family(
    text: &str,
    font_family: &str,
    font_weight: &str,
    font_style: &str,
    tolerance: f64,
    line_width: f64,
    ct: ConversionType,
) -> savvy::Result<savvy::Sexp> {
    let mut builder = builder::LyonPathBuilder::new(tolerance as _, line_width as _);

    builder
        .outline(text, font_family, font_weight, font_style)
        .unwrap();

    let result = match ct {
        ConversionType::Path => builder.into_path(),
        ConversionType::Stroke => builder.into_stroke(),
        ConversionType::Fill => builder.into_fill(),
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
    let mut builder = builder::LyonPathBuilder::new(tolerance as _, line_width as _);

    builder.outline_from_file(text, font_file).unwrap();

    let result = match ct {
        ConversionType::Path => builder.into_path(),
        ConversionType::Stroke => builder.into_stroke(),
        ConversionType::Fill => builder.into_fill(),
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
    string2any_file(text, font_file, tolerance, line_width, ConversionType::Path)
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
    string2any_file(text, font_file, tolerance, 0., ConversionType::Path)
}

#[savvy]
fn dump_fontdb_impl() -> savvy::Result<savvy::Sexp> {
    let mut source: Vec<String> = Vec::new();
    let mut index: Vec<i32> = Vec::new();
    let mut family: Vec<String> = Vec::new();
    let mut weight: Vec<String> = Vec::new();
    let mut style: Vec<String> = Vec::new();

    for f in FONTDB.faces() {
        source.push(match f.source {
            fontdb::Source::Binary(_) => "(binary)".to_string(),
            fontdb::Source::File(ref path) => path.to_string_lossy().to_string(),
            fontdb::Source::SharedFile(ref path, _) => path.to_string_lossy().to_string(),
        });

        index.push(f.index as _);

        // TODO: Now fontdb returns multiple family names (localized one?),
        //       but the current code can accept only one.
        let family_name = if f.families.is_empty() {
            "".to_string()
        } else {
            f.families[0].0.clone()
        };
        family.push(family_name);

        #[rustfmt::skip]
        weight.push(
            match f.weight {
                fontdb::Weight::THIN        => "thin",
                fontdb::Weight::EXTRA_LIGHT => "extra_light",
                fontdb::Weight::LIGHT       => "light",
                fontdb::Weight::NORMAL      => "normal",
                fontdb::Weight::MEDIUM      => "medium",
                fontdb::Weight::SEMIBOLD    => "semibold",
                fontdb::Weight::BOLD        => "bold",
                fontdb::Weight::EXTRA_BOLD  => "extra_bold",
                fontdb::Weight::BLACK       => "black",
                _                           => "unknown",
            }
            .to_string(),
        );

        #[rustfmt::skip]
        style.push(
            match f.style {
                fontdb::Style::Normal  => "normal",
                fontdb::Style::Italic  => "italic",
                fontdb::Style::Oblique => "oblique",
            }
            .to_string(),
        );
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
            .any(|&i| (0. - 0.1..=100. / 125. + 0.1).contains(&i)));
        assert!(result
            .y
            .iter()
            .any(|&i| (0. - 0.1..=100. / 125. + 0.1).contains(&i)));
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
