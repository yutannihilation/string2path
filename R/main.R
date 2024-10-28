#' Convert a String to Paths
#'
#' `string2path()` converts a text to the paths of the width-less outlines of
#' each glyph. `string2stroke()` converts a text to the paths of the outlines,
#' with the specified line width, of each glyph. `string2fill()` converts a text
#' to the paths of the filled polygon of each glyph.
#'
#' @name string2path
#' @param text A text to convert to paths.
#' @param font A font family (e.g. `"Arial"`) or a path to a font file (e.g.
#'   `"path/to/font.ttf"`).
#' @param font_weight A font weight.
#' @param font_style A font style.
#' @param tolerance Maximum distance allowed between the curve and its
#'   approximation. For more details, please refer to [the documentation of the
#'   underlying Rust
#'   library](https://docs.rs/lyon_geom/latest/lyon_geom/#flattening).
#'
#' @param line_width Line width of strokes.
#'
#' @return A `tibble()` containing these columns:
#' \describe{
#'   \item{x}{x position of the point on the path, scaled to x / line height. The left side of the first glyph is at x = 0.}
#'   \item{y}{Y position of the point on the path, scaled to y / line height. The baseline of the first line is at y = 0.}
#'   \item{glyph_id}{IDs to distinguish the glyphs.}
#'   \item{path_id}{IDs to distinguish the groups of paths.}
#'   \item{triangle_id}{IDs to distinguish the triangles. `string2path()` doesn't contain this column.}
#' }
#'
#' @examples
#' available_fonts <- dump_fontdb()
#'
#' if (nrow(available_fonts) > 0) {
#'   family <- available_fonts$family[1]
#'   weight <- available_fonts$weight[1]
#'   style  <- available_fonts$style[1]
#'
#'   # string2path() converts a text to paths
#'   d_path <- string2path("TEXT", family, weight, style)
#'   if (nrow(d_path) > 0) {
#'     plot(d_path$x, d_path$y)
#'     for (p in split(d_path, d_path$path_id)) {
#'       lines(p$x, p$y)
#'     }
#'   }
#'
#'   # string2stroke() converts a text to strokes
#'   d_stroke <- string2stroke("TEXT", family, weight, style)
#'   if (nrow(d_stroke) > 0) {
#'     plot(d_stroke$x, d_stroke$y)
#'
#'     # The stroke is split into triangles, which can be distinguished by `triangle_id`
#'     set.seed(2)
#'     for (p in split(d_stroke, d_stroke$triangle_id)) {
#'       polygon(p$x, p$y, col = rgb(runif(1), runif(1), runif(1), 0.8))
#'     }
#'   }
#'
#'   # string2fill() converts a text to filled polygons
#'   d_fill <- string2fill("TEXT", family, weight, style)
#'   if (nrow(d_fill) > 0) {
#'     plot(d_fill$x, d_fill$y)
#'
#'     # The polygon is split into triangles, which can be distinguished by `triangle_id`
#'     set.seed(2)
#'     for (p in split(d_fill, d_fill$triangle_id)) {
#'       polygon(p$x, p$y, col = rgb(runif(1), runif(1), runif(1), 0.8))
#'     }
#'   }
#' }
#'
#' @export
string2path <- function(
    text,
    font,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005
) {
  if (is_font_file(font)) {
    if (!missing(font_weight) || !missing(font_style)) {
      cli::cli_warn("{.arg font_weight} and {.arg font_style} are ignored when extracting a font file.")
    }

    font <- path.expand(font)
    tibble::as_tibble(string2path_file(text, font, tolerance))
  } else {
    font_weight <- match.arg(font_weight)
    font_style <- match.arg(font_style)

    tibble::as_tibble(string2path_family(text, font, font_weight, font_style, tolerance))
  }
}

#' @rdname string2path
#' @export
string2stroke <- function(
    text,
    font,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005,
    line_width = 0.03
) {
  if (is_font_file(font)) {
    if (!missing(font_weight) || !missing(font_style)) {
      cli::cli_warn("{.arg font_weight} and {.arg font_style} are ignored when extracting a font file.")
    }

    font <- path.expand(font)
    tibble::as_tibble(string2stroke_file(text, font, tolerance, line_width))
  } else {
    font_weight <- match.arg(font_weight)
    font_style <- match.arg(font_style)

    tibble::as_tibble(string2stroke_family(text, font, font_weight, font_style, tolerance, line_width))
  }
}

#' @rdname string2path
#' @export
string2fill <- function(
    text,
    font,
    font_weight = c("normal", "thin", "extra_thin", "light", "medium", "semibold", "bold", "extra_bold", "black"),
    font_style = c("normal", "italic", "oblique"),
    tolerance = 0.00005
) {
  if (is_font_file(font)) {
    if (!missing(font_weight) || !missing(font_style)) {
      cli::cli_warn("{.arg font_weight} and {.arg font_style} are ignored when extracting a font file.")
    }

    font <- path.expand(font)
    tibble::as_tibble(string2fill_file(text, font, tolerance))
  } else {
    font_weight <- match.arg(font_weight)
    font_style <- match.arg(font_style)

    tibble::as_tibble(string2fill_family(text, font, font_weight, font_style, tolerance))
  }
}

# Hope there's no fonts whose family name ends with .ttf or .otf!
is_font_file <- function(x) {
  isTRUE(endsWith(x, ".ttf") || endsWith(x, ".otf"))
}
