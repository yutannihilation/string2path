use savvy::{OwnedIntegerSxp, OwnedRealSxp, OwnedStringSxp};

/// An intermediate form of the extracted path information to convert to a tibble.
pub struct PathTibble {
    // Unscaled position of x.
    pub x: Vec<f64>,
    // Unscaled position of y.
    pub y: Vec<f64>,
    // IDs to distinguish the glyphs. Note that this is a different ID than [ttf_parser::GlyphId].
    pub glyph_id: Vec<i32>,
    // IDs to distinguish the groups of paths (i.e., `Begin` path event to `End` path event).
    pub path_id: Vec<i32>,
    // IDs to distinguish the triangles. This field is `None` for `ConversionType::Path`.
    pub triangle_id: Option<Vec<i32>>,
}

impl From<PathTibble> for savvy::SEXP {
    fn from(value: PathTibble) -> Self {
        let len = if value.triangle_id.is_none() { 4 } else { 5 };
        let mut out = savvy::OwnedListSxp::new(len, true);

        out.set_name_and_value(0, "x", <OwnedRealSxp>::from(value.x.as_slice()));
        out.set_name_and_value(1, "y", <OwnedRealSxp>::from(value.y.as_slice()));
        out.set_name_and_value(
            2,
            "glyph_id",
            <OwnedIntegerSxp>::from(value.glyph_id.as_slice()),
        );
        out.set_name_and_value(
            3,
            "path_id",
            <OwnedIntegerSxp>::from(value.path_id.as_slice()),
        );

        if let Some(triangle_id) = value.triangle_id {
            out.set_name_and_value(
                4,
                "triangle_id",
                <OwnedIntegerSxp>::from(triangle_id.as_slice()),
            );
        }

        out.into()
    }
}

/// An intermediate form of the content of the fontdb to convert to a tibble.
pub struct FontDBTibble {
    pub source: Vec<String>,
    pub index: Vec<i32>,
    pub family: Vec<String>,
    pub weight: Vec<String>,
    pub style: Vec<String>,
}

impl From<FontDBTibble> for savvy::SEXP {
    fn from(value: FontDBTibble) -> Self {
        let mut out = savvy::OwnedListSxp::new(5, true);

        out.set_name_and_value(0, "x", <OwnedStringSxp>::from(value.source.as_slice()));
        out.set_name_and_value(1, "y", <OwnedIntegerSxp>::from(value.index.as_slice()));
        out.set_name_and_value(2, "family", <OwnedStringSxp>::from(value.family.as_slice()));
        out.set_name_and_value(3, "weight", <OwnedStringSxp>::from(value.weight.as_slice()));
        out.set_name_and_value(4, "style", <OwnedStringSxp>::from(value.style.as_slice()));

        out.into()
    }
}
