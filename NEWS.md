# string2path (development version)

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
