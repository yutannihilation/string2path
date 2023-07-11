# string2path (development version)

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
