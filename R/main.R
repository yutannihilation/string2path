#' @export
glyph2digit <- function(str, ttf_file) {
  .Call(string2path_glyph2digit, str, ttf_file)
}
