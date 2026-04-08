# Dump the Font Database

For debugging purposes, extract all font faces on the font database
which 'string2path' uses internally.

## Usage

``` r
dump_fontdb()
```

## Value

A `tibble()` containing these columns:

- source:

  The source file of the font face.

- index:

  The index of the font face within the source.

- family:

  The font family of the face.

- weight:

  The weight of the face.

- style:

  The style of the face.
