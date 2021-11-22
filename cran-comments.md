## Test environments

* local Ubuntu Linux install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel, ucrt-devel
* win-builder: devel
* R Mac builder

## R CMD check results

0 errors | 0 warnings | 2 note

* This is a maintenance release to address the CRAN maintainer's comment that
  download.file() needs longer timeout when the download file is more than few
  GB.
* There's no reverse dependency.
