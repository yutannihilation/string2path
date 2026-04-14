This is a maintainance release to address failures on CRAN machines.

- x86_64 macOS: added missing flags (e.g. `-framework Foundation`).
- gcc-ASAN: ignore `-flto=*` on compiling intermediate objects in order to prevent the symbol from getting omitted.
