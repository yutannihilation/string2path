#' @useDynLib string2path, .registration = TRUE
#' @keywords internal
NULL


string2path_family <- function(text, font_family, font_weight, font_style, tolerance) {
  .Call(string2path_family__impl, text, font_family, font_weight, font_style, tolerance)
}


string2path_file <- function(text, font_file, tolerance) {
  .Call(string2path_file__impl, text, font_file, tolerance)
}


string2stroke_family <- function(text, font_family, font_weight, font_style, tolerance, line_width) {
  .Call(string2stroke_family__impl, text, font_family, font_weight, font_style, tolerance, line_width)
}


string2stroke_file <- function(text, font_file, tolerance, line_width) {
  .Call(string2stroke_file__impl, text, font_file, tolerance, line_width)
}


string2fill_family <- function(text, font_family, font_weight, font_style, tolerance) {
  .Call(string2fill_family__impl, text, font_family, font_weight, font_style, tolerance)
}


string2fill_file <- function(text, font_file, tolerance) {
  .Call(string2fill_file__impl, text, font_file, tolerance)
}


dump_fontdb_impl <- function() {
  .Call(dump_fontdb_impl__impl)
}


