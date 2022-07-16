use extendr_api::prelude::*;

use std::convert::TryFrom;

/// An intermediate form of the extracted path information to convert to a tibble.
pub struct PathTibble {
    // Unscaled position of x.
    pub x: Vec<f32>,
    // Unscaled position of y.
    pub y: Vec<f32>,
    // IDs to distinguish the glyphs. Note that this is a different ID than [ttf_parser::GlyphId].
    pub glyph_id: Vec<u32>,
    // IDs to distinguish the groups of paths (i.e., `Begin` path event to `End` path event).
    pub path_id: Vec<u32>,
    // IDs to distinguish the triangles. This field is `None` for `ConversionType::Path`.
    pub triangle_id: Option<Vec<u32>>,
}

impl TryFrom<PathTibble> for Robj {
    type Error = extendr_api::Error;

    fn try_from(value: PathTibble) -> std::result::Result<Self, Self::Error> {
        // Find tibble
        let tibble_robj = R!("tibble::tibble")?;
        let tibble = match tibble_robj.as_function() {
            Some(fun) => fun,
            None => {
                return Err(extendr_api::Error::ExpectedFunction(tibble_robj));
            }
        };

        let triangle_id: Robj = if let Some(triangle_id) = value.triangle_id {
            triangle_id.into()
        } else {
            NULL.into()
        };

        let result = tibble.call(pairlist!(
            x = value.x,
            y = value.y,
            glyph_id = value.glyph_id,
            path_id = value.path_id,
            triangle_id = triangle_id
        ))?;

        Ok(result)
    }
}

/// An intermediate form of the content of the fontdb to convert to a tibble.
pub struct FontDBTibble {
    pub source: Vec<String>,
    pub index: Vec<u32>,
    pub family: Vec<String>,
    pub weight: Vec<String>,
    pub style: Vec<String>,
}

impl TryFrom<FontDBTibble> for Robj {
    type Error = extendr_api::Error;

    fn try_from(value: FontDBTibble) -> std::result::Result<Self, Self::Error> {
        // Find tibble
        let tibble_robj = R!("tibble::tibble")?;
        let tibble = match tibble_robj.as_function() {
            Some(fun) => fun,
            None => {
                return Err(extendr_api::Error::ExpectedFunction(tibble_robj));
            }
        };

        let result = tibble.call(pairlist!(
            source = value.source,
            index = value.index,
            family = value.family,
            weight = value.weight,
            style = value.style
        ))?;

        Ok(result)
    }
}
