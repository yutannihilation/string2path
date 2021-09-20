args <- commandArgs(TRUE)

if (length(args) != 2) {
  stop("Usage: download_precompiled_binary.R URL DESTFILE")
}

URL <- args[1]
DESTFILE <- args[2]

dir.create(dirname(DESTFILE), showWarnings = FALSE, recursive = TRUE)

download.file(URL, destfile = DESTFILE, mode = "wb")
