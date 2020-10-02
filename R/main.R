#' @param str A string to convert to paths.
#' @param ttf_file A path to TTF file.
#' @param tolerance Maximum distance allowed between the curve and its approximation. For more details,
#'     please refer to [lyon's doc](https://docs.rs/lyon/0.16.2/lyon/index.html#what-is-the-tolerance-variable-in-these-examples).
#' @export
string2path <- function(str, ttf_file, tolerance =  0.001) {
  ttf_file <- path.expand(ttf_file)
  if (!file.exists(ttf_file)) {
    stop(paste("No such file", ttf_file), call. = NULL)
  }

  out <- .Call(string2path_impl, str, ttf_file, tolerance)
  names(out) <- c("x", "y", "id", "glyph_id")
  tibble::as_tibble(out)
}

#' @param str A string to convert to paths.
#' @param ttf_file A path to TTF file.
#' @param tolerance Maximum distance allowed between the curve and its approximation. For more details,
#'     please refer to [lyon's doc](https://docs.rs/lyon/0.16.2/lyon/index.html#what-is-the-tolerance-variable-in-these-examples).
#' @export
string2vertex <- function(str, ttf_file, tolerance =  0.001) {
  ttf_file <- path.expand(ttf_file)
  if (!file.exists(ttf_file)) {
    stop(paste("No such file", ttf_file), call. = NULL)
  }

  out <- .Call(string2vertex_impl, str, ttf_file, tolerance)
  names(out) <- c("x", "y", "id", "glyph_id")
  tibble::as_tibble(out)
}
