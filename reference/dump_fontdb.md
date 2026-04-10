# Dump the Font Database

For debugging purposes, extract all font faces on the font database
which 'string2path' uses internally.

## Usage

``` r
dump_fontdb()
```

## Value

A `tibble()` containing these columns:

- index:

  The index of the font face within the source.

- family:

  The font family of the face.

- weight:

  The numeric weight of the face (e.g. 400 for normal, 700 for bold).

- style:

  The style of the face.
