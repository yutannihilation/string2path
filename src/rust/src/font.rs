use crate::builder::LyonPathBuilder;

use once_cell::sync::Lazy;

use std::{error::Error, fmt::Display};
use ttf_parser::GlyphId;

static FONTDB: Lazy<fontdb::Database> = Lazy::new(|| {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    db
});

#[derive(Debug)]
pub enum FontLoadingError {
    FaceParsingError(ttf_parser::FaceParsingError),
    IOError(std::io::Error),
    NoAvailableFonts,
}

impl Display for FontLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontLoadingError::FaceParsingError(err) => err.fmt(f),
            FontLoadingError::IOError(err) => err.fmt(f),
            FontLoadingError::NoAvailableFonts => {
                write!(f, "No available fonts is found on the machine")
            }
        }
    }
}

impl Error for FontLoadingError {}

impl From<ttf_parser::FaceParsingError> for FontLoadingError {
    fn from(err: ttf_parser::FaceParsingError) -> Self {
        Self::FaceParsingError(err)
    }
}

impl From<std::io::Error> for FontLoadingError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl LyonPathBuilder {
    pub fn outline(
        &mut self,
        text: &str,
        family: &str,
        weight: &str,
    ) -> Result<(), FontLoadingError> {
        // TODO
        #[rustfmt::skip]
        let weight = match weight {
            "thin"       => fontdb::Weight::THIN,
            "extra_thin" => fontdb::Weight::EXTRA_LIGHT,
            "light"      => fontdb::Weight::LIGHT,
            "normal"     => fontdb::Weight::NORMAL,
            "medium"     => fontdb::Weight::MEDIUM,
            "semibold"   => fontdb::Weight::SEMIBOLD,
            "bold"       => fontdb::Weight::BOLD,
            "extra_bold" => fontdb::Weight::EXTRA_BOLD,
            "black"      => fontdb::Weight::BLACK,
            _            => fontdb::Weight::NORMAL,
        };

        // 1. Try the user-supplied query first

        let query = fontdb::Query {
            families: &[fontdb::Family::Name(family)],
            weight,
            ..Default::default()
        };

        if let Some(id) = FONTDB.query(&query) {
            let result = FONTDB.with_face_data(id, |font_data, face_index| {
                self.outline_inner(text, font_data, face_index)
            });

            return result.unwrap_or(Ok(()));
        }

        // 2. If not found, try the fallback query which should hit at least one font

        let fallback_query = fontdb::Query {
            families: &[fontdb::Family::SansSerif, fontdb::Family::Serif],
            ..Default::default()
        };

        if let Some(id) = FONTDB.query(&fallback_query) {
            let result = FONTDB.with_face_data(id, |font_data, face_index| {
                self.outline_inner(text, font_data, face_index)
            });

            return result.unwrap_or(Ok(()));
        }

        // 3. When no fonts are available, return an error.

        Err(FontLoadingError::NoAvailableFonts)
    }

    pub fn outline_from_file(
        &mut self,
        text: &str,
        font_file: &str,
    ) -> Result<(), FontLoadingError> {
        let font_data_raw = std::fs::read(font_file)?;
        self.outline_inner(text, font_data_raw.as_slice(), 0)?;

        Ok(())
    }

    fn outline_inner(
        &mut self,
        text: &str,
        font_data: &[u8],
        face_index: u32,
    ) -> Result<(), FontLoadingError> {
        // TODO: handle error
        let font = ttf_parser::Face::from_slice(font_data, face_index)?;
        let facetables = font.tables();

        let height = font.height() as f32;
        self.scale_factor = 1. / height;
        let line_height = height + font.line_gap() as f32;

        let mut prev_glyph: Option<GlyphId> = None;
        for c in text.chars() {
            // Skip control characters
            if c.is_control() {
                // If the character is a line break, move to the next line
                if c == '\n' {
                    self.offset_y -= line_height;
                    self.offset_x = 0.;
                }
                prev_glyph = None;
                continue;
            }
            // Even when we cannot find glyph_id, fill it with 0.
            let cur_glyph = font.glyph_index(c).unwrap_or(GlyphId(0));

            if let Some(prev_glyph) = prev_glyph {
                self.offset_x += find_kerning(facetables, prev_glyph, cur_glyph) as f32;
            }

            font.outline_glyph(cur_glyph, self);

            if let Some(ha) = font.glyph_hor_advance(cur_glyph) {
                self.offset_x += ha as f32;
            }

            prev_glyph = Some(cur_glyph);
            self.cur_glyph_id += 1;
        }

        Ok(())
    }
}

// Return kerning between two glyphs. When no kerning information is available,
// return 0.
fn find_kerning(
    facetables: &ttf_parser::FaceTables,
    left: ttf_parser::GlyphId,
    right: ttf_parser::GlyphId,
) -> i16 {
    let kern_table = if let Some(kern_table) = facetables.kern {
        kern_table
    } else {
        return 0;
    };

    for st in kern_table.subtables {
        if !st.horizontal {
            continue;
        }

        if let Some(kern) = st.glyphs_kerning(left, right) {
            return kern;
        }
    }

    0
}
