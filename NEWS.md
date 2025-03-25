# string2path (development version)

* Fix a CRAN warning (#149).

# string2path 0.2.1

* Fix a regression about fill rule (#131).

* Address new CRAN check on the compiled code in sub-directories (#144, #146).

# string2path 0.2.0

* Partially support COLRv1 emoji fonts.
  * COLRv1 emoji font is a color emoji, but not all color emoji is COLRv1
    format. For example, Noto Color Emoji has several variants, and it seems
    the primary one is CBDT/CBLC format.
  * Additional information are currently just discarded.
    * The clip and layer composition information are just discarded. While
      this can be useful, it's not very easy to use these information in R.

* Fix `string2fill()` and `string2stroke()`; when the second argument is a path
  to a file, these unintentionally worked as `string2path()`.

* `string2path()` now generates the same outline as `string2fill()` and 
  `string2stroke()` (#69).

* `path_id` and `glyph_id` are now 1-origin.

* The result of `string2fill()` and `string2stroke()` now don't contain a
  `path_id` column. I found the calculation of `path_id` had never been
  correct, and it's probably better to remove it to avoid confusion.

# string2path 0.1.8

* This is a maintenance release to comply with the CRAN repository policy.

# string2path 0.1.7

* This is a maintenance release to fix a build error on ARM Linux.

# string2path 0.1.6

* This is a maintenance release to update the dependency Rust crates.

# string2path 0.1.5

* This is a maintenance release to fix a misassumption about R installation
  without shared libraries.

# string2path 0.1.4

* Fix build failure with Rust >= 1.70 on Windows

# string2path 0.1.3

* This is a maintenance release to fix a misassumption about Rust installation
  on Windows.

# string2path 0.1.2

* This is a maintenance release to disable downloading pre-compiled binaries in
  order to comply with the CRAN Repository Policy.

# string2path 0.1.1

* This is a maintenance release to fix some mistakes pointed by the CRAN
  maintainer.

## Breaking changes

* Drop support for R < 4.2.

# string2path 0.1.0

## Breaking changes

* Drop support for R < 4.1.

* Now all functions accept **font family name**. This is to support TTC file
  properly, which contains more than one fonts. A new function `dump_fontdb()`
  is useful to check the actual family name (and the weight and the style) to
  specify.
  They also accept a file path, so the existing code should work, except when
  specifying `tolerance` without the named argument.

* The minimum supported Rust version is bumped to 1.56.0 for the 2021 edition.

# string2path 0.0.4

* This is a maintenance release to make the installation work even on a slow
  internet connection.

# string2path 0.0.3

* This is a maintenance release to improve configure scripts to detect Rust
  installations correctly. No new features are added.

# string2path 0.0.2

* Fix CRAN build errors.
* Support "open path"-type glyphs (#7).

# string2path 0.0.1

* Added a `NEWS.md` file to track changes to the package.
