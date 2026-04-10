use std::sync::Mutex;

use crate::builder::{BuildPath, LyonPathBuilder};

use once_cell::sync::Lazy;
use skrifa::instance::Location;
use skrifa::outline::DrawSettings;
use skrifa::prelude::{LocationRef, Size, Tag};
use skrifa::raw::TableProvider;
use skrifa::raw::tables::kern::SubtableKind;
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
        let weight_value: f32 = match font_weight {
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
        };

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
                if let Some(font_info) = family.match_font(
                    fontique::FontWidth::from_ratio(1.0),
                    style,
                    fontique::FontWeight::new(weight_value),
                    false,
                ) {
                    if let Some(data) = font_info.load(None) {
                        result = Some((data, font_info.index()));
                    }
                }
            }
            result
        };

        if let Some((font_data, index)) = named_result {
            return self.outline_inner(text, font_data.as_ref(), index, weight_value, font_style);
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
            return self.outline_inner(text, font_data.as_ref(), index, weight_value, font_style);
        }

        // 3. When no fonts are available, return an error.
        savvy::r_eprint!("No font is available!");

        Err(FontLoadingError::NoAvailableFonts.into())
    }

    pub fn outline_from_file(&mut self, text: &str, font_file: &str) -> savvy::Result<()> {
        let font_data_raw =
            std::fs::read(font_file).map_err(|e| savvy::Error::new(e.to_string()))?;
        // Weight/style are unknown for file-loaded fonts; use defaults so variable
        // fonts render at their default design position.
        self.outline_inner(text, font_data_raw.as_slice(), 0, 400.0, "normal")?;
        Ok(())
    }

    /// Detects whether `font_data` is a static or variable font and dispatches
    /// to the appropriate rendering path.
    fn outline_inner(
        &mut self,
        text: &str,
        font_data: &[u8],
        face_index: u32,
        weight: f32,
        style: &str,
    ) -> savvy::Result<()> {
        let font = FontRef::from_index(font_data, face_index)
            .map_err(|e| savvy::Error::new(e.to_string()))?;

        if font.axes().is_empty() {
            self.outline_static(&font, text)
        } else {
            self.outline_variable(&font, text, weight, style)
        }
    }

    /// Renders `text` using a static (non-variable) font.
    ///
    /// The font face was already selected by fontique, so no variation axes
    /// need to be set; we render at the default location.
    fn outline_static(&mut self, font: &FontRef<'_>, text: &str) -> savvy::Result<()> {
        self.draw_glyphs(font, text, LocationRef::default())
    }

    /// Renders `text` using a variable font.
    ///
    /// Builds a [`Location`] from the requested `weight` (`wght` axis) and
    /// `style` (`ital` or `slnt` axis), then draws each glyph at that location.
    fn outline_variable(
        &mut self,
        font: &FontRef<'_>,
        text: &str,
        weight: f32,
        style: &str,
    ) -> savvy::Result<()> {
        let axes = font.axes();

        // Collect user-space axis settings. Tags that don't exist in the font
        // are silently ignored by axes.location().
        let mut settings: Vec<(&str, f32)> = vec![("wght", weight)];
        match style {
            "italic" => settings.push(("ital", 1.0)),
            "oblique" => {
                // Prefer a continuous slant axis; fall back to binary italic.
                if axes.get_by_tag(Tag::new(b"slnt")).is_some() {
                    settings.push(("slnt", -12.0));
                } else {
                    settings.push(("ital", 1.0));
                }
            }
            _ => {}
        }

        let location: Location = axes.location(settings);
        self.draw_glyphs(font, text, LocationRef::from(&location))
    }

    /// Core glyph rendering loop shared by both static and variable paths.
    fn draw_glyphs(
        &mut self,
        font: &FontRef<'_>,
        text: &str,
        location: LocationRef<'_>,
    ) -> savvy::Result<()> {
        let metrics = font.metrics(Size::unscaled(), location);
        // In TrueType, descent is negative, so height = ascent - descent gives total cell height.
        let height = (metrics.ascent - metrics.descent) as f32;
        self.set_scale_factor(1. / height);
        let line_height = height + metrics.leading as f32;

        let outlines = font.outline_glyphs();
        let charmap = font.charmap();
        let glyph_metrics = font.glyph_metrics(Size::unscaled(), location);

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

            if let Some(prev) = prev_glyph {
                self.add_offset_x(find_kerning(font, prev, cur_glyph));
            }

            if !c.is_whitespace() {
                // TODO: Implement COLR color glyph support via skrifa::color::ColorPainter.
                // Currently falling back to regular outline rendering only.
                if let Some(glyph) = outlines.get(cur_glyph) {
                    glyph
                        .draw(
                            DrawSettings::unhinted(Size::unscaled(), location),
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

/// Returns the kerning adjustment (in font design units) for the given glyph pair.
/// Iterates `kern` table subtables and returns the first matching horizontal kern value.
/// Returns 0.0 if no kerning information is available.
fn find_kerning(font: &FontRef<'_>, left: GlyphId, right: GlyphId) -> f32 {
    let Ok(kern) = font.kern() else { return 0.0 };
    for subtable in kern.subtables() {
        let Ok(subtable) = subtable else { continue };
        if !subtable.is_horizontal() {
            continue;
        }
        let Ok(kind) = subtable.kind() else { continue };
        let value = match kind {
            SubtableKind::Format0(f) => f.kerning(left, right),
            SubtableKind::Format2(f) => f.kerning(left, right),
            SubtableKind::Format3(f) => f.kerning(left, right),
            SubtableKind::Format1(_) => None, // state machine format, not supported
        };
        if let Some(v) = value {
            return v as f32;
        }
    }
    0.0
}
