base_url <- "https://www.r-project.org/nosvn/winutf8/ucrt3/"
destfile <- file.path(Sys.getenv("TEMP", unset = "."), "R-devel-win-ucrt.exe")

# setup PATH
writeLines('PATH="${RTOOLS40_HOME}\\ucrt64\\bin;${RTOOLS40_HOME}\\usr\\bin;${PATH}"', con = "~/.Renviron")

l <- readLines(base_url)
# e.g.)
# ...>R-devel-win-80717-4617-4659.exe</a>...
ptn <- ".*>(R-devel-win-[0-9-]+\\.exe)</a>.*"
file <- unique(gsub(ptn, "\\1", grep(ptn, l, value = TRUE)))

url <- paste0(base_url, file)

message("Downloading ", url, " into ", destfile, "...")

download.file(url, destfile)
