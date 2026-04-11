# Changelog

## string2path 0.3.0

- Fix a CRAN warning about macOS deployment target mismatch on M1.

- Migrate to fontique and skrifa
  ([\#99](https://github.com/yutannihilation/string2path/issues/99)).

  - Support variable fonts.
  - Drop support for WASM for now.

- `font_weight` now accepts numeric values (e.g. `400`) in addition to
  character strings (e.g. `"normal"`).

## string2path 0.2.2

CRAN release: 2025-03-25

- Fix a CRAN warning
  ([\#149](https://github.com/yutannihilation/string2path/issues/149)).

## string2path 0.2.1

CRAN release: 2025-03-20

- Fix a regression about fill rule
  ([\#131](https://github.com/yutannihilation/string2path/issues/131)).

- Address new CRAN check on the compiled code in sub-directories
  ([\#144](https://github.com/yutannihilation/string2path/issues/144),
  [\#146](https://github.com/yutannihilation/string2path/issues/146)).

## string2path 0.2.0

CRAN release: 2025-02-08

- Partially support COLRv1 emoji fonts.

  - COLRv1 emoji font is a color emoji, but not all color emoji is
    COLRv1 format. For example, Noto Color Emoji has several variants,
    and it seems the primary one is CBDT/CBLC format.
  - Additional information are currently just discarded.
    - The clip and layer composition information are just discarded.
      While this can be useful, it’s not very easy to use these
      information in R.

- Fix
  [`string2fill()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  and
  [`string2stroke()`](https://yutannihilation.github.io/string2path/reference/string2path.md);
  when the second argument is a path to a file, these unintentionally
  worked as
  [`string2path()`](https://yutannihilation.github.io/string2path/reference/string2path.md).

- [`string2path()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  now generates the same outline as
  [`string2fill()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  and
  [`string2stroke()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  ([\#69](https://github.com/yutannihilation/string2path/issues/69)).

- `path_id` and `glyph_id` are now 1-origin.

- The result of
  [`string2fill()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  and
  [`string2stroke()`](https://yutannihilation.github.io/string2path/reference/string2path.md)
  now don’t contain a `path_id` column. I found the calculation of
  `path_id` had never been correct, and it’s probably better to remove
  it to avoid confusion.

## string2path 0.1.8

CRAN release: 2024-08-24

- This is a maintenance release to comply with the CRAN repository
  policy.

## string2path 0.1.7

CRAN release: 2024-05-31

- This is a maintenance release to fix a build error on ARM Linux.

## string2path 0.1.6

CRAN release: 2023-12-17

- This is a maintenance release to update the dependency Rust crates.

## string2path 0.1.5

CRAN release: 2023-07-11

- This is a maintenance release to fix a misassumption about R
  installation without shared libraries.

## string2path 0.1.4

CRAN release: 2023-07-09

- Fix build failure with Rust \>= 1.70 on Windows

## string2path 0.1.3

CRAN release: 2023-01-23

- This is a maintenance release to fix a misassumption about Rust
  installation on Windows.

## string2path 0.1.2

CRAN release: 2022-12-17

- This is a maintenance release to disable downloading pre-compiled
  binaries in order to comply with the CRAN Repository Policy.

## string2path 0.1.1

CRAN release: 2022-08-06

- This is a maintenance release to fix some mistakes pointed by the CRAN
  maintainer.

### Breaking changes

- Drop support for R \< 4.2.

## string2path 0.1.0

CRAN release: 2022-07-17

### Breaking changes

- Drop support for R \< 4.1.

- Now all functions accept **font family name**. This is to support TTC
  file properly, which contains more than one fonts. A new function
  [`dump_fontdb()`](https://yutannihilation.github.io/string2path/reference/dump_fontdb.md)
  is useful to check the actual family name (and the weight and the
  style) to specify. They also accept a file path, so the existing code
  should work, except when specifying `tolerance` without the named
  argument.

- The minimum supported Rust version is bumped to 1.56.0 for the 2021
  edition.

## string2path 0.0.4

CRAN release: 2021-11-22

- This is a maintenance release to make the installation work even on a
  slow internet connection.

## string2path 0.0.3

CRAN release: 2021-09-26

- This is a maintenance release to improve configure scripts to detect
  Rust installations correctly. No new features are added.

## string2path 0.0.2

CRAN release: 2021-08-09

- Fix CRAN build errors.
- Support “open path”-type glyphs
  ([\#7](https://github.com/yutannihilation/string2path/issues/7)).

## string2path 0.0.1

CRAN release: 2021-08-04

- Added a `NEWS.md` file to track changes to the package.
