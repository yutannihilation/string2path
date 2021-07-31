#' Convert strings to paths
#'
#' @name string2path
#' @param text
#'   A text to convert to paths.
#' @param
#'   font_file A path to TTF or OTF file.
#' @param tolerance
#'   Maximum distance allowed between the curve and its approximation. For more details,
#'   please refer to [lyon's doc](https://docs.rs/lyon/0.16.2/lyon/index.html#what-is-the-tolerance-variable-in-these-examples).
#' @param line_width
#'   Line width of strokes.
#' @param result_type
#'   Result types.
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
