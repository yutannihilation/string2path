## Test environments

* local Ubuntu Linux install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel, ucrt-devel
* win-builder: devel
* R Mac builder

## R CMD check results

0 errors | 0 warnings | 2 note

* This is a maintenance release to improve configure scripts to detect the Rust
  installation, and handle the case when it's not available.
* Regarding the current CRAN check results,
      - The ERROR on r-devel-windows-x86_64-gcc10-UCRT ("curl: (60) SSL 
        certificate problem") might indicate something is wrong with the curl
        installation on the CRAN machine. But, this version switches to use
        the standard R function `download.file()` instead of curl, so hopefully
        the error will disappear.
      - On some platform, it NOTEs that "All declared Imports should be used"
        about tibble package. However, tibble is surely used in the Rust code
        so I believe these NOTEs are false-positive and can be ignored.
* There's no reverse dependency.
