## Test environments

* local Windows install: release
* GitHub Actions CI (Windows, macOS, Linux): release and devel
* win-builder: devel, release

## R CMD check results

0 errors | 0 warnings | 2 note

This release is a maintenance release in response to the email from the CRAN
maintainer. I sincerely apologize for the trouble. This version fixed the
following points:


1. Bundle the sources of all the dependency Rust libraries

All the sources are now bundled in src/rust/vendor.tar.xz [1]. So, no download
will happen if the Rust compiler is installed (more specifically, if cargo
command can be found on PATH).

Note that, only when it is found that no Rust compiler is available on the
machine, it falls back to downloading the pre-compiled binary. I believe the
pre-compiled binary in this case can be considered as a "last resort" because
otherwise there's no option other than to fail.


2. Describe all the authorship and copyright of the dependencies

Now the DESCRIPTION includes "The authors of the dependency Rust crates" in 
Authors@R field, and inst/AUTHORS file lists up all the authors. Also, the new
file LICENSE.note describes the details of the licenses of the sources. In
summary, all libraries are distributed either under the MIT license or under
MIT/Apache-2.0 dual license.


In addition, this version also addresses the CRAN check failures:


3. Drop support for R 4.1

The CRAN check on r-oldrel-windows-ix86+x86_64 keeps failing. My assumption is
that Windows 2008 is a bit too old to run the binary compiled with some recent
version of Rust; the error message indicates the OS lacks some system API which
is expected to be available. So, while I still believe string2path runs fine
with R 4.1, the support needs to be dropped (I tried to compile with some
previous versions of Rust, but it cannot compile some of the dependencies).


4. Fix the example code

The CRAN check on r-devel-linux-x86_64-debian-gcc also keeps failing. This was
because of my misconsideration about the case when no appropriate font is
available. The example is now skipped in such cases.


[1]: This has to be  compressed to a tarball because the directory structure is
     so deep that otherwise we would see the warning "storing paths of more than
     100 bytes is not portable."
