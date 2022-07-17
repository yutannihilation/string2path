## Test environments

* local Windows install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel
* win-builder: devel
* R Mac builder

## R CMD check results

0 errors | 0 warnings | 2 note

* This release is mainly for improving the internals by updating the versions of
  the dependencies. While this includes a small signature changes on the main
  functions, which is technically considered as a breaking change, there should
  be no significant impact on users.
* There's no reverse dependency.
