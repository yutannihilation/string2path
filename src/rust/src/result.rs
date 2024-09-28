use savvy::{OwnedIntegerSexp, OwnedRealSexp, OwnedStringSexp};

/// An intermediate form of the extracted path information to convert to a tibble.
pub struct PathTibble {
    // Unscaled position of x.
    pub x: Vec<f64>,
    // Unscaled position of y.
    pub y: Vec<f64>,
    // IDs to distinguish the glyphs. Note that this is a different ID than [ttf_parser::GlyphId].
    pub glyph_id: Vec<i32>,
    // IDs to distinguish the groups of paths (i.e., `Begin` path event to `End` path event).
    pub path_id: Option<Vec<i32>>,
    // IDs to distinguish the triangles. This field is `None` for `ConversionType::Path`.
    pub triangle_id: Option<Vec<i32>>,
    // Color of color emoji font.
    pub color: Option<Vec<String>>,
}

impl PathTibble {
    fn len(&self) -> usize {
        let mut len = 3;
        if self.path_id.is_some() {
            len += 1
        };
        if self.triangle_id.is_some() {
            len += 1
        };
        if self.color.is_some() {
            len += 1
        };
        len
    }
}

impl TryFrom<PathTibble> for savvy::Sexp {
    type Error = savvy::Error;

    fn try_from(value: PathTibble) -> savvy::Result<Self> {
        let mut out = savvy::OwnedListSexp::new(value.len(), true)?;

        out.set_name_and_value(0, "x", <OwnedRealSexp>::try_from(value.x.as_slice())?)?;
        out.set_name_and_value(1, "y", <OwnedRealSexp>::try_from(value.y.as_slice())?)?;
        out.set_name_and_value(
            2,
            "glyph_id",
            <OwnedIntegerSexp>::try_from(value.glyph_id.as_slice())?,
        )?;

        // optional columns
        let mut idx = 2;

        if let Some(path_id) = value.path_id {
            idx += 1;
            let v = <OwnedIntegerSexp>::try_from(path_id.as_slice())?;
            out.set_name_and_value(idx, "path_id", v)?;
        }
        if let Some(triangle_id) = value.triangle_id {
            idx += 1;
            let v = <OwnedIntegerSexp>::try_from(triangle_id.as_slice())?;
            out.set_name_and_value(idx, "triangle_id", v)?;
        }
        if let Some(color) = value.color {
            idx += 1;
            let v = <OwnedStringSexp>::try_from(color.as_slice())?;
            out.set_name_and_value(idx, "color", v)?;
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

impl TryFrom<FontDBTibble> for savvy::Sexp {
    type Error = savvy::Error;
    fn try_from(value: FontDBTibble) -> savvy::Result<Self> {
        let mut out = savvy::OwnedListSexp::new(5, true)?;

        out.set_name_and_value(
            0,
            "source",
            <OwnedStringSexp>::try_from(value.source.as_slice())?,
        )?;
        out.set_name_and_value(
            1,
            "index",
            <OwnedIntegerSexp>::try_from(value.index.as_slice())?,
        )?;
        out.set_name_and_value(
            2,
            "family",
            <OwnedStringSexp>::try_from(value.family.as_slice())?,
        )?;
        out.set_name_and_value(
            3,
            "weight",
            <OwnedStringSexp>::try_from(value.weight.as_slice())?,
        )?;
        out.set_name_and_value(
            4,
            "style",
            <OwnedStringSexp>::try_from(value.style.as_slice())?,
        )?;

        out.into()
    }
}
