## Test environments

* local Windows install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel
* win-builder: devel

## R CMD check results

0 errors | 0 warnings | 2 note

This release is a maintenance release to comply with the CRAN Repository Policy
more strictly. In the last release, the package had a mechanism to download the
precompiled binary as a fallback when no Rust toolchain is available. This was 
prepared on the assumption the CRAN machines don't have the Rust toolchain
installed, but it seems this is not true. Therefore, I removed the mechanism.
