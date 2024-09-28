use crate::builder::{BuildPath, LyonPathBuilder, LyonPathBuilderForPaint};

use once_cell::sync::Lazy;

use ttf_parser::{GlyphId, RgbaColor};

pub(crate) static FONTDB: Lazy<fontdb::Database> = Lazy::new(|| {
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

impl From<FontLoadingError> for savvy::Error {
    fn from(value: FontLoadingError) -> Self {
        let msg = match value {
            FontLoadingError::FaceParsingError(err) => err.to_string(),
            FontLoadingError::IOError(err) => err.to_string(),
            FontLoadingError::NoAvailableFonts => {
                "No available fonts is found on the machine".to_string()
            }
        };

        savvy::Error::new(&msg)
    }
}

impl<T: BuildPath> LyonPathBuilder<T> {
    pub fn outline(
        &mut self,
        text: &str,
        font_family: &str,
        font_weight: &str,
        font_style: &str,
    ) -> savvy::Result<()> {
        #[rustfmt::skip]
        let weight = match font_weight {
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

        #[rustfmt::skip]
        let style = match font_style {
            "normal"  => fontdb::Style::Normal,
            "italic"  => fontdb::Style::Italic,
            "oblique" => fontdb::Style::Oblique,
            _         => fontdb::Style::Normal,
        };

        // 1. Try the user-supplied query first

        let query = fontdb::Query {
            families: &[fontdb::Family::Name(font_family)],
            weight,
            style,
            ..Default::default()
        };

        if let Some(id) = FONTDB.query(&query) {
            let result = FONTDB.with_face_data(id, |font_data, face_index| {
                self.outline_inner(text, font_data, face_index)
            });

            return result.unwrap_or(Ok(()));
        }

        savvy::r_eprint!("No font face matched with the specified conditions. Falling back to the default font...");

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

        savvy::r_eprint!("No font is available!");

        Err(FontLoadingError::NoAvailableFonts.into())
    }

    pub fn outline_from_file(&mut self, text: &str, font_file: &str) -> savvy::Result<()> {
        // NOTE: Technically, fontdb can load file data with .load_font_file().
        //       It might simplify the implimentation, but it would require us
        //       to specify a query, but we don't know how to query the exact
        //       information in the file. So, having another implimentation is
        //       probably reasonable for now.
        let font_data_raw =
            std::fs::read(font_file).map_err(|e| savvy::Error::new(&e.to_string()))?;
        self.outline_inner(text, font_data_raw.as_slice(), 0)?;

        Ok(())
    }

    fn outline_inner(
        &mut self,
        text: &str,
        font_data: &[u8],
        face_index: u32,
    ) -> savvy::Result<()> {
        // TODO: handle error
        let font = ttf_parser::Face::parse(font_data, face_index)
            .map_err(|e| savvy::Error::new(&e.to_string()))?;
        let facetables = font.tables();

        let height = font.height() as f32;
        self.set_scale_factor(1. / height);
        let line_height = height + font.line_gap() as f32;

        let mut prev_glyph: Option<GlyphId> = None;
        for c in text.chars() {
            // Skip control characters
            if c.is_control() {
                // If the character is a line break, move to the next line
                if c == '\n' {
                    self.sub_offset_y(line_height);
                    self.reset_offset_x();
                }
                prev_glyph = None;
                continue;
            }

            // increment glyph ID for consistency
            self.cur_glyph_id += 1;

            // Even when we cannot find glyph_id, fill it with 0.
            let cur_glyph = font.glyph_index(c).unwrap_or(GlyphId(0));

            if let Some(prev_glyph) = prev_glyph {
                self.add_offset_x(find_kerning(facetables, prev_glyph, cur_glyph) as f32);
            }

            if font.is_color_glyph(cur_glyph) {
                let mut painter = LyonPathBuilderForPaint::new(self, &font);
                let fg_color = RgbaColor::new(0, 0, 0, 255);
                let res = font.paint_color_glyph(cur_glyph, 0, fg_color, &mut painter);
                res.ok_or_else(|| savvy::Error::new(&format!("Failed to outline char '{c}'")))?;
            } else {
                let res = font.outline_glyph(cur_glyph, self);
                res.ok_or_else(|| savvy::Error::new(&format!("Failed to outline char '{c}'")))?;
            }

            if let Some(ha) = font.glyph_hor_advance(cur_glyph) {
                self.add_offset_x(ha as f32);
            }

            prev_glyph = Some(cur_glyph);
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
