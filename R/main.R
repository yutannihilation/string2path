#' Convert a string to paths.
#'
#' `string2path()` converts a text to the paths of the width-less outlines of
#' each glyph. `string2stroke()` converts a text to the paths of the outlines,
#' with the specified line width, of each glyph. `string2fill()` converts a text
#' to the paths of the filled polygon of each glyph.
#'
#' @name string2path
#' @param text A text to convert to paths.
#' @param font_file A path to a 'TrueType' font file ('.ttf') or an 'OpenType'
#'   font file ('.otf').
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
#' if (requireNamespace("systemfonts", quietly = TRUE)) {
#'   available_fonts <- systemfonts::system_fonts()$path
#'
#'   # string2path supports only TrueType or OpenType formats
#'   ttf_or_otf <- available_fonts[grepl("\\.(ttf|otf)$", available_fonts)]
#'
#'   # string2path() converts a text to paths
#'   d_path <- string2path("TEXT", ttf_or_otf[1])
#'   plot(d_path$x, d_path$y)
#'   for (p in split(d_path, d_path$path_id)) {
#'     lines(p$x, p$y)
#'   }
#'
#'   # string2stroke() converts a text to strokes
#'   d_stroke <- string2stroke("TEXT", ttf_or_otf[1])
#'   plot(d_stroke$x, d_stroke$y)
#'
#'   # The stroke is split into triangles, which can be distinguished by `triangle_id`
#'   set.seed(2)
#'   for (p in split(d_stroke, d_stroke$triangle_id)) {
#'     polygon(p$x, p$y, col = rgb(runif(1), runif(1), runif(1), 0.8))
#'   }
#'
#'   # string2fill() converts a text to filled polygons
#'   d_fill <- string2fill("TEXT", ttf_or_otf[1])
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
string2path <- function(text, font_file, tolerance = 0.00005) {
  string2path_impl(text, font_file, tolerance)
}

#' @rdname string2path
#' @export
string2stroke <- function(text, font_file, tolerance = 0.00005, line_width = 0.03) {
  string2stroke_impl(text, font_file, tolerance, line_width)
}

#' @rdname string2path
#' @export
string2fill <- function(text, font_file, tolerance = 0.00005) {
  string2fill_impl(text, font_file, tolerance)
}
