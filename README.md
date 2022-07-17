
<!-- README.md is generated from README.Rmd. Please edit that file -->

# string2path

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/string2path/workflows/R-CMD-check/badge.svg)](https://github.com/yutannihilation/string2path/actions)
[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
[![CRAN
status](https://www.r-pkg.org/badges/version/string2path)](https://CRAN.R-project.org/package=string2path)
[![string2path status
badge](https://yutannihilation.r-universe.dev/badges/string2path)](https://yutannihilation.r-universe.dev)
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

If you are using macOS or Windows, you are lucky. Since this repository
provides the pre-compiled binary for you, you don’t need to install Rust
toolchains!

Otherwise (i.e. Linux), you need to have Rust toolchain installed before
trying to install this package. See
<https://www.rust-lang.org/tools/install> for the installation
instructions.

``` r
install.packages("string2path")

# Or the development version from r-universe:
install.packages("string2path",
  repos = c(
    yutannihilation = "https://yutannihilation.r-universe.dev",
    CRAN = "https://cloud.r-project.org"
  )
)
```

## Example

### `string2path()`

``` r
library(string2path)
library(ggplot2)

d <- string2path("カラテが\n高まる。", "Noto Sans JP", font_weight = "bold")

d <- tibble::rowid_to_column(d)

ggplot(d) +
  geom_path(aes(x, y, group = path_id, colour = factor(glyph_id)),
            linewidth = 1.5) +
  theme_minimal() +
  coord_equal() +
  theme(legend.position = "top") +
  scale_colour_viridis_d(option = "H")
```

<img src="man/figures/README-example-1.png" width="50%" />

``` r

library(gganimate)
d <- string2path("蹴", "Noto Sans JP")
d <- tibble::rowid_to_column(d)

ggplot(d) +
  geom_path(aes(x, y, group = path_id),
            linewidth = 2, colour = "purple2", lineend = "round") +
  theme_minimal() +
  coord_equal() +
  transition_reveal(rowid)
```

<img src="man/figures/README-example-1.gif" width="50%" />

#### `dump_fontdb()`

Note that `"Noto Sans JP"` above (and `"Iosevka SS08"` below) is the
font installed on my local machine, so the same code might not run on
your environment. You can use `dump_fontdb()` to see the available
combination of font family (e.g. `"Arial"`), weight (e.g. `"bold"`), and
style (e.g. `"italic"`).

``` r
dump_fontdb()
#> # A tibble: 447 × 5
#>    source                                  index family        weight style 
#>    <chr>                                   <dbl> <chr>         <chr>  <chr> 
#>  1 "C:\\Windows\\Fonts\\arial.ttf"             0 Arial         normal normal
#>  2 "C:\\Windows\\Fonts\\arialbd.ttf"           0 Arial         bold   normal
#>  3 "C:\\Windows\\Fonts\\arialbi.ttf"           0 Arial         bold   italic
#>  4 "C:\\Windows\\Fonts\\ariali.ttf"            0 Arial         normal italic
#>  5 "C:\\Windows\\Fonts\\ariblk.ttf"            0 Arial Black   black  normal
#>  6 "C:\\Windows\\Fonts\\bahnschrift.ttf"       0 Bahnschrift   normal normal
#>  7 "C:\\Windows\\Fonts\\BIZ-UDGothicB.ttc"     0 BIZ UDGothic  bold   normal
#>  8 "C:\\Windows\\Fonts\\BIZ-UDGothicB.ttc"     1 BIZ UDPGothic bold   normal
#>  9 "C:\\Windows\\Fonts\\BIZ-UDGothicR.ttc"     0 BIZ UDGothic  normal normal
#> 10 "C:\\Windows\\Fonts\\BIZ-UDGothicR.ttc"     1 BIZ UDPGothic normal normal
#> # … with 437 more rows
```

You can also specify the font file directly. Pomicons is a font
available on
[gabrielelana/pomicons](https://github.com/gabrielelana/pomicons),
licensed under SIL OFL 1.1.

``` r
pomicons_file <- here::here("fonts", "Pomicons.ttf")
if (!file.exists(pomicons_file)) {
  dir.create(dirname(pomicons_file))
  curl::curl_download(
    "https://github.com/gabrielelana/pomicons/blob/master/fonts/Pomicons.ttf?raw=true",
    destfile = pomicons_file
  )
}

d_tmp <- string2path("\uE007", pomicons_file)

ggplot(d_tmp) +
  geom_path(aes(x, y, group = path_id), linewidth = 5, colour = "#26d1a9") +
  theme_minimal() +
  coord_equal()
```

<img src="man/figures/README-unnamed-chunk-2-1.png" width="50%" />

### `string2fill()`

``` r
d <- string2fill("abc", "Iosevka SS08", font_weight = "bold", font_style = "italic")

ggplot(d) +
  geom_polygon(aes(x, y, group = triangle_id, fill = factor(triangle_id %% 7)),
               colour = "grey", linewidth = 0.1) +
  theme_minimal() +
  coord_equal() +
  theme(legend.position = "none") +
  scale_fill_viridis_d(option = "H")
```

<img src="man/figures/README-example2-1.png" width="50%" />

### `string2stroke()`

``` r
for (w in 1:9 * 0.01) {
  d <- string2stroke("abc","Iosevka SS08", font_weight = "bold", font_style = "italic", line_width = w)
  
  p <- ggplot(d) +
    geom_polygon(aes(x, y, group = triangle_id, fill = factor(triangle_id %% 2)),
                 colour = "grey", linewidth = 0.1) +
    theme_minimal() +
    coord_equal() +
    theme(legend.position = "none") +
    scale_fill_manual(values = c("purple", "pink"))
  plot(p)
}
```

<img src="man/figures/README-string2stroke-.gif" width="50%" />

## `tolerance`

`tolerance` controls resolution of the tessellation. You can reduce
tolerance to get higher resolutions.

``` r
for (tolerance in c(1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7)) {
  d <- string2fill("abc", "Iosevka SS08", font_weight = "bold", font_style = "italic", tolerance = tolerance)
  
  p <- ggplot(d) +
    geom_polygon(aes(x, y, group = triangle_id),
                 fill = "transparent", colour = "black", linewidth = 0.5) +
    theme_minimal() +
    coord_equal() +
    ggtitle(paste0("tolerance: ", tolerance))
  plot(p)
}
```

<img src="man/figures/README-example3-.gif" width="50%" />

Note that `tolerance` parameter behaves a bit differently on
`string2fill()` and `string2stroke()`. But, in either case, 1e-5 \~ 1e-6
should be enough.

``` r
for (tolerance in c(1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7)) {
  d <- string2path("abc", "Iosevka SS08", font_weight = "bold", font_style = "italic", tolerance = tolerance)
  
  p <- ggplot(d) +
    geom_path(aes(x, y, group = path_id), colour = "black", linewidth = 0.5) +
    geom_point(aes(x, y, group = path_id), colour = "black", size = 1.5) +
    theme_minimal() +
    coord_equal() +
    ggtitle(paste0("tolerance: ", tolerance))
  plot(p)
}
```

<img src="man/figures/README-example4-.gif" width="50%" />
