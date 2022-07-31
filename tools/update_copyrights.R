library(RcppTOML)

manifests <- list.files("src/rust/vendor", pattern = "Cargo.toml", recursive = TRUE, full.names = TRUE)

l <- lapply(manifests, \(x) RcppTOML::parseTOML(x)$package)

names <- vapply(l, \(x) x[["name"]], FUN.VALUE = character(1L))

files <- paste0(dirname(manifests), "/*")

authors <- vapply(l, \(x) {
  # Remove email addresses
  authors <- stringr::str_remove(x[["authors"]], "\\s+<.+>")
  paste(authors, collapse = ", ")
}, FUN.VALUE = character(1L))

licenses <- vapply(l, \(x) x[["license"]], FUN.VALUE = character(1L))

dir.create("inst", showWarnings = FALSE)

cat("This package contains the Rust source code of the dependencies in src/rust/vendor.tar.xz
The copyright information (the authors and the licenses) are listed below.
'Files' field shows the paths after extraction.

===============================

", file = "inst/COPYRIGHTS")

cat(paste(
  "Files:   ", files,    "\n",
  "Name:    ", names,    "\n",
  "Authors: ", authors,  "\n",
  "License: ", licenses, "\n",
  sep = "",
  collapse = "\n------------------------------\n\n"
), file = "inst/COPYRIGHTS", append = TRUE)
