#' Convert a string to paths.
#'
#' `string2path()` converts a text to the paths of the 'width-less' outlines of
#' each glyph. `string2stroke()` converts a text to the paths of the outlines,
#' with the specified line width, of each glyph. `string2fill()` converts a text
#' to the paths of the filled polygon of each glyph.
#'
#' @name string2path
#' @param text A text to convert to paths.
#' @param font_file A path to TTF or OTF file.
#' @param tolerance Maximum distance allowed between the curve and its
#'   approximation. For more details, please refer to [the documentation of the
#'   underlying Rust
#'   library](https://docs.rs/lyon/0.17.5/lyon/#what-is-the-tolerance-variable-in-these-examples).
#'
#' @param line_width Line width of strokes.
#'
#' @return A `tibble()` containing these columns: \itemize{ \item{x}{Unscaled
#'   position of x.} \item{y}{Unscaled position of y.} \item{glyph_id}{IDs to
#'   distinguish the glyphs.} \item{path_id}{IDs to distinguish the groups of
#'   paths.} \item{triangle_id}{IDs to distinguish the triangles.
#'   `string2path()` doesn't contain this column.} }
#'
#' @examples
#' if (requireNamespace("systemfonts", quietly = TRUE)) {
#'   available_fonts <- systemfonts::system_fonts()$path
#'
#'   ttf_or_otf <- available_fonts[grepl("\\.(ttf|otf)$", available_fonts)]
#'
#'   d_path <- string2path("TEXT", ttf_or_otf[1])
#'   plot(d_path$x, d_path$y)
#'   lines(d_path$x, d_path$y)
#' }
#'
#' @export
string2path <- function(text, font_file, tolerance = 0.01) {
  string2path_impl(text, font_file, tolerance)
}

#' @rdname string2path
#' @export
string2stroke <- function(text, font_file, tolerance = 0.01, line_width = 10) {
  string2stroke_impl(text, font_file, tolerance, line_width)
}

#' @rdname string2path
#' @export
string2fill <- function(text, font_file, tolerance = 0.01) {
  string2fill_impl(text, font_file, tolerance)
}
