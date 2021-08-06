
<!-- README.md is generated from README.Rmd. Please edit that file -->

# string2path

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/string2path/workflows/R-CMD-check/badge.svg)](https://github.com/yutannihilation/string2path/actions)
[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
[![CRAN
status](https://www.r-pkg.org/badges/version/string2path)](https://CRAN.R-project.org/package=string2path)
<!-- badges: end -->

The string2path R package converts a text to paths of the outlines of
each glyph, based on a font data. Under the hood, this package is
powered by [extendr](https://extendr.github.io/) framework to use these
two Rust crates:

-   [ttf-parser](https://github.com/RazrFalcon/ttf-parser) for parsing
    font data. TrueType font (`.ttf`) and OpenType font (`.otf`) are
    supported.
-   [lyon](https://github.com/nical/lyon/) for tessellation of polygons
    and flattening the curves.

## Installation

If you are using Windows, you are lucky. Because this repository
provides pre-compiled binary for you, you don’t need to install Rust
toolchain.

Otherwise, you need to have Rust toolchain installed before trying to
install this package. See <https://www.rust-lang.org/tools/install> for
the installation instructions.

``` r
install.packages("string2path")

# Or the development version from GitHub:
# install.packages("devtools")
devtools::install_github("yutannihilation/string2path")
```

## Example

### `string2path()`

``` r
library(string2path)
library(ggplot2)

# This TTF file is downloaded from https://ipafont.ipa.go.jp/.
# For installed fonts, you can use systemfonts::system_fonts()
# to lookup the path.
d <- string2path("カラテが\n高まる。", "./fonts/ipaexg.ttf")

d <- tibble::rowid_to_column(d)

ggplot(d) +
  geom_path(aes(x, y, group = path_id, colour = factor(glyph_id)), size = 1.5) +
  theme_minimal() +
  coord_equal() +
  theme(legend.position = "top") +
  scale_colour_viridis_d(option = "H")
```

<img src="man/figures/README-example-1.png" width="100%" />

``` r
library(gganimate)
d <- string2path("蹴", "./fonts/ipaexg.ttf")
d <- tibble::rowid_to_column(d)

ggplot(d) +
  geom_path(aes(x, y, group = path_id), size = 2, colour = "purple2", lineend = "round") +
  theme_minimal() +
  coord_equal() +
  transition_reveal(rowid)
```

<img src="man/figures/README-example-1.gif" width="100%" />

### `string2fill()`

``` r
# Sorry for my laziness, please replace the font path to the appropriate location in your system...
ttf_file <- "./fonts/iosevka-heavyitalic.ttf"

d <- string2fill("abc", ttf_file)

ggplot(d) +
  geom_polygon(aes(x, y, group = triangle_id, fill = factor(triangle_id %% 7)), colour = "grey", size = 0.1) +
  theme_minimal() +
  coord_equal() +
  theme(legend.position = "none") +
  scale_fill_viridis_d(option = "H")
```

<img src="man/figures/README-example2-1.png" width="100%" />

### `string2stroke()`

``` r
for (w in 1:9 * 0.01) {
  d <- string2stroke("abc", ttf_file, line_width = w)
  
  p <- ggplot(d) +
    geom_polygon(aes(x, y, group = triangle_id, fill = factor(triangle_id %% 2)), colour = "grey", size = 0.1) +
    theme_minimal() +
    coord_equal() +
    theme(legend.position = "none") +
    scale_fill_manual(values = c("purple", "pink"))
  plot(p)
}
```

<img src="man/figures/README-string2stroke-.gif" width="100%" />

## `tolerance`

`tolerance` controls resolution of the tessellation. You can reduce
tolerance to get higher resolutions.

``` r
for (tolerance in c(1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7)) {
  d <- string2fill("abc", ttf_file, tolerance = tolerance)
  
  p <- ggplot(d) +
    geom_polygon(aes(x, y, group = triangle_id), fill = "transparent", colour = "black", size = 0.5) +
    theme_minimal() +
    coord_equal() +
    ggtitle(paste0("tolerance: ", tolerance))
  plot(p)
}
```

<img src="man/figures/README-example3-.gif" width="100%" />

Note that `tolerance` parameter behaves a bit differently on
`string2fill()` and `string2stroke()`. But, in either case, 1e-5 \~ 1e-6
should be enough.

``` r
for (tolerance in c(1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7)) {
  d <- string2path("abc", ttf_file, tolerance = tolerance)
  
  p <- ggplot(d) +
    geom_path(aes(x, y, group = path_id), colour = "black", size = 0.5) +
    geom_point(aes(x, y, group = path_id), colour = "black", size = 1.5) +
    theme_minimal() +
    coord_equal() +
    ggtitle(paste0("tolerance: ", tolerance))
  plot(p)
}
```

<img src="man/figures/README-example4-.gif" width="100%" />
