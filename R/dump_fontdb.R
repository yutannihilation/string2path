#' Dump the Font Database
#'
#' For debugging purposes, extract all font faces on the font database which
#' 'string2path' uses internally.
#'
#' @return A `tibble()` containing these columns:
#' \describe{
#'   \item{source}{The source file of the font face.}
#'   \item{index}{The index of the font face within the source.}
#'   \item{family}{The font family of the face.}
#'   \item{weight}{The weight of the face.}
#'   \item{style}{The style of the face.}
#' }
#'
#' @export
dump_fontdb <- function() {
  dump_fontdb_impl()
}
