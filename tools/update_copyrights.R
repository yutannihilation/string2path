library(RcppTOML)

VENDOR_PATH <- "src/rust/vendor"
manifests <- list.files(VENDOR_PATH, pattern = "Cargo.toml", recursive = TRUE)

l <- lapply(manifests, \(x) RcppTOML::parseTOML(file.path(VENDOR_PATH, x))$package)

names <- vapply(l, \(x) x[["name"]], FUN.VALUE = character(1L))

authors <- vapply(l, \(x) {
  # Remove email addresses
  authors <- stringr::str_remove(x[["authors"]], "\\s+<.+>")
  paste(authors, collapse = ", ")
}, FUN.VALUE = character(1L))

licenses <- vapply(l, \(x) x[["license"]], FUN.VALUE = character(1L))

files <- paste0("vendor/", dirname(manifests), "/*")

dir.create("inst", showWarnings = FALSE)

cat("This package contains the Rust source code of the dependencies in src/rust/vendor.tar.xz
The authorships and the licenses are listed below. In summary, all libraries are
distributed either under the MIT license or under MIT/Apache-2.0 dual license [1].

Note that, when Cargo (Rust’s build system and package manager) is not installed
on the machine, the pre-compiled binary is downloaded on building this package.
The binary is compiled using the same Rust code, so the authorships and the
licenses are the same as listed here.

[1]: The unicode-indent library shows 'Unicode-DFS-2016', but it's not about the
    Rust code in the library. Please refer to the License section of the README
    (https://crates.io/crates/unicode-ident) for the details.

===============================

", file = "inst/COPYRIGHTS")

cat(paste(
  "Name:    ", names,    "\n",
  "Authors: ", authors,  "\n",
  "License: ", licenses, "\n",
  "Files:   ", files,    "\n",
  sep = "",
  collapse = "\n------------------------------\n\n"
), file = "inst/COPYRIGHTS", append = TRUE)
