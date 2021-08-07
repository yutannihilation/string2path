## Test environments

* local Ubuntu Linux install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel
* win-builder: devel

## R CMD check results

0 errors | 0 warnings | 1 note

* This is a maintenance release in response to the request to fix the build
  failures on CRAN. I sincerely apologize for the trouble.
* This release contains these fixes:
    - macOS: Now the configure script detects if the external command (i.e.
        `cargo`) is available. If not, it downloads the necessary setups.
    - M1 mac: src/Makevars now correctly sets PATH so that `rustc` command can
        be found.
    - Add "GNU make" to SystemRequirements because the src/Makevars uses a
      GNU make extension.
* I would like to request to exclude Solaris from the build targets because
  Solaris is not a supported platform by Rust. This should be in line with the
  treatments of other CRAN packages that use Rust; gifski, baseflow, and
  salso are not built on Solaris.
* The build error on r-devel-windows-x86_64-gcc10-UCRT should be solved when the
  dependency package (i.e., tibble) become available.
* There's no reverse dependency.
