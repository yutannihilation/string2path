#' Convert a String to Paths
#'
#' `string2path()` converts a text to the paths of the width-less outlines of
#' each glyph. `string2stroke()` converts a text to the paths of the outlines,
#' with the specified line width, of each glyph. `string2fill()` converts a text
#' to the paths of the filled polygon of each glyph.
#'
#' @name string2path
#' @param text A text to convert to paths.
#' @param font_family A font family.
#' @param font_weight A font weight.
#' @param font_style A font style.
#' @param tolerance Maximum distance allowed between the curve and its
#'   approximation. For more details, please refer to [the documentation of the
#'   underlying Rust
#'   library](https://docs.rs/lyon/0.17.5/lyon/#what-is-the-tolerance-variable-in-these-examples).
#'
#' @param line_width Line width of strokes.
#'
#' @return A `tibble()` containing these columns:
#' \describe{
#'   \item{x}{x position of the point on the path, scaled to x / line height. The left side of the first glyph is at x = 0.}
#'   \item{y}{Y position of the point on the path, scaled to y / line height. The baseline of the first line is at y = 0.}
#'   \item{glyph_id}{IDs to distinguish the glyphs.}
#'   \item{path_id}{IDs to distinguish the groups of paths.}
#'   \item{triangle_id}{IDs to distinguish the triangles. `string2path()` doesn't contain this column.}
#' }
#'
#' @examples
#' available_fonts <- dump_fontdb()
#'
#' if (nrow(available_fonts) > 0) {
#'   family <- available_fonts$family[1]
#'   weight <- available_fonts$weight[1]
#'   style  <- available_fonts$style[1]
#'
#'   # string2path() converts a text to paths
#'   d_path <- string2path("TEXT", family, weight, style)
#'   plot(d_path$x, d_path$y)
#'   for (p in split(d_path, d_path$path_id)) {
#'     lines(p$x, p$y)
#'   }
#'
#'   # string2stroke() converts a text to strokes
#'   d_stroke <- string2stroke("TEXT", family, weight, style)
#'   plot(d_stroke$x, d_stroke$y)
#'
#'   # The stroke is split into triangles, which can be distinguished by `triangle_id`
#'   set.seed(2)
#'   for (p in split(d_stroke, d_stroke$triangle_id)) {
#'     polygon(p$x, p$y, col = rgb(runif(1), runif(1), runif(1), 0.8))
#'   }
#'
#'   # string2fill() converts a text to filled polygons
#'   d_fill <- string2fill("TEXT", family, weight, style)
#'   plot(d_fill$x, d_fill$y)
#'
#'   # The polygon is split into triangles, which can be distinguished by `triangle_id`
#'   set.seed(2)
#'   for (p in split(d_fill, d_fill$triangle_id)) {
#'     polygon(p$x, p$y, col = rgb(runif(1), runif(1), runif(1), 0.8))
#'   }
#' }
#'
#' @export
string2path <- function(
    text,
    font_family,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005
) {
  validate_font_family(font_family, "string2path")

  font_weight <- match.arg(font_weight)
  font_style <- match.arg(font_style)

  string2path_impl(text, font_family, font_weight, font_style, tolerance)
}

#' @rdname string2path
#' @export
string2stroke <- function(
    text,
    font_family,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005,
    line_width = 0.03
) {
  validate_font_family(font_family, "string2stroke")

  font_weight <- match.arg(font_weight)
  font_style <- match.arg(font_style)

  string2stroke_impl(text, font_family, font_weight, font_style, tolerance, line_width)
}

#' @rdname string2path
#' @export
string2fill <- function(
    text,
    font_family,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005
) {
  validate_font_family(font_family, "string2fill")

  font_weight <- match.arg(font_weight)
  font_style <- match.arg(font_style)

  string2fill_impl(text, font_family, font_weight, font_style, tolerance)
}

validate_font_family <- function(font_family, call) {
  if (isTRUE(grepl("\\.(ttf|otf)$", font_family))) {
    cli::cli_abort(
      "{.fun {call}} now uses a font family name (e.g. `\"Arial\"`), instead of the path to the font file."
    )
  }
}
