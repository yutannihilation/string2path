#' @param str A string to convert to paths.
#' @param ttf_file A path to TTF file.
#' @export
string2path <- function(str, ttf_file) {
  ttf_file <- path.expand(ttf_file)
  if (!file.exists(ttf_file)) {
    stop(paste("No such file", ttf_file), call. = NULL)
  }

  out <- .Call(string2path_impl, str, ttf_file)
  names(out) <- c("x", "y", "id")
  tibble::as_tibble(out)
}
