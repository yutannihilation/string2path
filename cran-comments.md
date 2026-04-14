This is a maintenance release to address failures on CRAN check machines.

- x86_64 macOS: explicitly link the Apple frameworks (Foundation, CoreText,
  CoreFoundation, CoreGraphics, AppKit) used transitively by the Rust
  dependencies, to fix an unresolved `_kCTFontURLAttribute` at load time.

- gcc-ASAN: append `-fno-lto` to CFLAGS used for the Rust build so the C
  shim is not archived as LTO bitcode. This is a package-level workaround;
  the proper fix belongs upstream and will follow in a later release.
