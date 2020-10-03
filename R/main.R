#' Convert strings to paths
#'
#' @rdname string2path
#' @param str
#'   A string to convert to paths.
#' @param
#'   ttf_file A path to TTF file.
#' @param tolerance
#'   Maximum distance allowed between the curve and its approximation. For more details,
#'   please refer to [lyon's doc](https://docs.rs/lyon/0.16.2/lyon/index.html#what-is-the-tolerance-variable-in-these-examples).
#' @param line_width
#'   Line width of strokes.
#' @param result_type
#'   Result types.
#' @export
string2path <- function(str, ttf_file, tolerance = 0.001) {
  .string2path(str, ttf_file, tolerance, result_type = "path")
}

#' @rdname string2path
#' @export
string2stroke <- function(str, ttf_file, tolerance = 0.001, line_width = 0.1) {
  .string2path(str, ttf_file, tolerance, line_width, result_type = "stroke")
}

#' @rdname string2path
#' @export
string2fill <- function(str, ttf_file, tolerance = 0.001) {
  .string2path(str, ttf_file, tolerance, result_type = "fill")
}


.string2path <- function(str, ttf_file, tolerance =  0.001, line_width = 0.1, ..., result_type = c("fill", "stroke", "path")) {
  result_type <- match.arg(result_type)
  result_type <- match(result_type, c("fill", "stroke", "path"))

  ttf_file <- path.expand(ttf_file)
  if (!file.exists(ttf_file)) {
    stop(paste("No such file", ttf_file), call. = NULL)
  }

  out <- .Call(string2path_impl, str, ttf_file, tolerance, line_width, as.integer(result_type) - 1L)

  if (is.null(out)) {
    warning("Failed to convert", call. = FALSE)
    return(out)
  }

  names(out) <- c("x", "y", "id", "glyph_id")
  tibble::as_tibble(out)
}
