#' @useDynLib string2path, .registration = TRUE
#' @keywords internal
"_PACKAGE"


string2path_family <- function(text, font_family, font_weight, font_style, tolerance) {
  .Call(savvy_string2path_family, text, font_family, font_weight, font_style, tolerance)
}


string2path_file <- function(text, font_file, tolerance) {
  .Call(savvy_string2path_file, text, font_file, tolerance)
}


string2stroke_family <- function(text, font_family, font_weight, font_style, tolerance, line_width) {
  .Call(savvy_string2stroke_family, text, font_family, font_weight, font_style, tolerance, line_width)
}


string2stroke_file <- function(text, font_file, tolerance, line_width) {
  .Call(savvy_string2stroke_file, text, font_file, tolerance, line_width)
}


string2fill_family <- function(text, font_family, font_weight, font_style, tolerance) {
  .Call(savvy_string2fill_family, text, font_family, font_weight, font_style, tolerance)
}


string2fill_file <- function(text, font_file, tolerance) {
  .Call(savvy_string2fill_file, text, font_file, tolerance)
}


dump_fontdb_impl <- function() {
  .Call(savvy_dump_fontdb_impl, )
}


