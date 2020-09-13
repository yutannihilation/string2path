#' @export
glyph2digit <- function(str, ttf_file) {
  .Call(glyph2digit_impl, str, ttf_file)
}

#' @export
string2path <- function(str, ttf_file) {
  .Call(string2path_impl, str, ttf_file)
}
