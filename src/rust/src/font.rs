use std::sync::Mutex;

use crate::builder::{BuildPath, LyonPathBuilder};

use once_cell::sync::Lazy;
use skrifa::outline::DrawSettings;
use skrifa::prelude::{LocationRef, Size};
use skrifa::{FontRef, GlyphId, MetadataProvider};

pub(crate) static FONT_COLLECTION: Lazy<Mutex<fontique::Collection>> = Lazy::new(|| {
    Mutex::new(fontique::Collection::new(
        fontique::CollectionOptions::default(),
    ))
});

#[derive(Debug)]
pub enum FontLoadingError {
    ParseError(String),
    LoadError(String),
    IOError(std::io::Error),
    NoAvailableFonts,
}

impl From<std::io::Error> for FontLoadingError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<FontLoadingError> for savvy::Error {
    fn from(value: FontLoadingError) -> Self {
        let msg = match value {
            FontLoadingError::ParseError(s) => s,
            FontLoadingError::LoadError(s) => s,
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
        let weight = fontique::FontWeight::new(match font_weight {
            "thin"       => 100.0,
            "extra_thin" => 200.0,
            "light"      => 300.0,
            "normal"     => 400.0,
            "medium"     => 500.0,
            "semibold"   => 600.0,
            "bold"       => 700.0,
            "extra_bold" => 800.0,
            "black"      => 900.0,
            _            => 400.0,
        });

        #[rustfmt::skip]
        let style = match font_style {
            "italic"  => fontique::FontStyle::Italic,
            "oblique" => fontique::FontStyle::Oblique(None),
            _         => fontique::FontStyle::Normal,
        };

        // 1. Try the user-supplied family name first.
        let named_result = {
            let mut collection = FONT_COLLECTION.lock().unwrap();
            let mut result = None;
            if let Some(family) = collection.family_by_name(font_family) {
                if let Some(font_info) =
                    family.match_font(fontique::FontWidth::from_ratio(1.0), style, weight, false)
                {
                    if let Some(data) = font_info.load(None) {
                        result = Some((data, font_info.index()));
                    }
                }
            }
            result
        };

        if let Some((font_data, index)) = named_result {
            return self.outline_inner(text, font_data.as_ref(), index);
        }

        savvy::r_eprint!(
            "No font face matched with the specified conditions. Falling back to the default font..."
        );

        // 2. Fallback: use any available system font.
        // fontique does not expose generic family names (SansSerif/Serif), so we
        // use the first family found in the collection.
        let fallback_result = {
            let mut collection = FONT_COLLECTION.lock().unwrap();
            let first_name = collection.family_names().next().map(|s| s.to_string());
            let mut result = None;
            if let Some(name) = first_name {
                if let Some(family) = collection.family_by_name(&name) {
                    if let Some(font_info) = family.fonts().first() {
                        if let Some(data) = font_info.load(None) {
                            result = Some((data, font_info.index()));
                        }
                    }
                }
            }
            result
        };

        if let Some((font_data, index)) = fallback_result {
            return self.outline_inner(text, font_data.as_ref(), index);
        }

        // 3. When no fonts are available, return an error.
        savvy::r_eprint!("No font is available!");

        Err(FontLoadingError::NoAvailableFonts.into())
    }

    pub fn outline_from_file(&mut self, text: &str, font_file: &str) -> savvy::Result<()> {
        let font_data_raw =
            std::fs::read(font_file).map_err(|e| savvy::Error::new(e.to_string()))?;
        self.outline_inner(text, font_data_raw.as_slice(), 0)?;
        Ok(())
    }

    fn outline_inner(
        &mut self,
        text: &str,
        font_data: &[u8],
        face_index: u32,
    ) -> savvy::Result<()> {
        let font = FontRef::from_index(font_data, face_index)
            .map_err(|e| savvy::Error::new(e.to_string()))?;

        let metrics = font.metrics(Size::unscaled(), LocationRef::default());
        // In TrueType, descent is negative, so height = ascent - descent gives total cell height.
        let height = (metrics.ascent - metrics.descent) as f32;
        self.set_scale_factor(1. / height);
        let line_height = height + metrics.leading as f32;

        let outlines = font.outline_glyphs();
        let charmap = font.charmap();
        let glyph_metrics = font.glyph_metrics(Size::unscaled(), LocationRef::default());

        let mut prev_glyph: Option<GlyphId> = None;
        for c in text.chars() {
            // Skip control characters.
            if c.is_control() {
                if c == '\n' {
                    self.sub_offset_y(line_height);
                    self.reset_offset_x();
                }
                prev_glyph = None;
                continue;
            }

            // Increment glyph ID for consistency.
            self.cur_glyph_id += 1;

            // Even when we cannot find a glyph, fall back to .notdef (GlyphId 0).
            let cur_glyph = charmap.map(c).unwrap_or(GlyphId::new(0));

            // TODO: Implement kerning using skrifa's kern table access.
            // Currently returning 0 for all kerning pairs.
            let _ = prev_glyph;

            if !c.is_whitespace() {
                // TODO: Implement COLR color glyph support via skrifa::color::ColorPainter.
                // Currently falling back to regular outline rendering only.
                if let Some(glyph) = outlines.get(cur_glyph) {
                    glyph
                        .draw(
                            DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
                            self,
                        )
                        .map_err(|e| savvy::Error::new(e.to_string()))?;
                }
            }

            if let Some(advance) = glyph_metrics.advance_width(cur_glyph) {
                self.add_offset_x(advance);
            }

            prev_glyph = Some(cur_glyph);
        }

        Ok(())
    }
}
