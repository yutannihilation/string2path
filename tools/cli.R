# The following fields in DESCRIPTION can be used for customizing the behavior.
#
# Config/<package name>/MSRV (optional):
#   Minimum Supported Rust version (e.g. 1.41.0). If this is specified, errors
#   when the installed cargo is newer than the requirement.
#
# Config/<package name>/windows_toolchain (optional):
#   Expected toolchain of the Rust installation (e.g. stable-msvc). If this is
#   specified, errors when the specified toolchain is not available.
#
# Config/<package name>/github_repo:
#   Name of the GitHub repo (e.g. yutannihilation/string2path)
#
# Config/<package name>/crate_name (optional):
#   Name of the crate (e.g. string2path). If the crate name is the same as the
#   repository name, this can be omitted.
#
# Config/<package name>/github_tag:
#   Tag of the GitHub release of the precompiled binaries (e.g. build_20210921-1)
#
# Config/<package name>/binary_sha256sum:
#   The expected checksums of the precompiled binaries as an expression of a list.
#   Note: This needs to be an R expression rather than CSV or TSV, because the
#         DESCRIPTION gets auto-formatted when compiling, which introduces
#         unexpected line breaks.
#
#   Example:
#       list(
#           `aarch64-apple-darwin-libstring2path.a`       = "4a34f99cec66610746b20d456b1e11b346596c22ea1935c1bcb5ef1ab725f0e8",
#           `i686-pc-windows-gnu-libstring2path.a`        = "ceda54184fb3bf9e4cbba86848cb2091ff5b77870357f94319f9215fadfa5b25",
#           `ucrt-x86_64-pc-windows-gnu-libstring2path.a` = "26a05f6ee8c2f625027ffc77c97fc8ac9746a182f5bc53d64235999a02c0b0dc",
#           `x86_64-apple-darwin-libstring2path.a`        = "be65f074cb7ae50e5784e7650f48579fff35f30ff663d1c01eabdc9f35c1f87c",
#           `x86_64-pc-windows-gnu-libstring2path.a`      = "26a05f6ee8c2f625027ffc77c97fc8ac9746a182f5bc53d64235999a02c0b0dc"
#       )

SYSINFO_OS      <- tolower(Sys.info()[["sysname"]])
SYSINFO_MACHINE <- Sys.info()[["machine"]]
HAS_32BIT_R     <- dir.exists(file.path(R.home(), "bin", "i386"))
USE_UCRT        <- identical(R.version$crt, "ucrt")


# Utilities ---------------------------------------------------------------

#' Read a field of the package's DESCRIPTION file
#'
#' The field should have the prefix
#'
#' @param field
#'   Name of a field without prefix (e.g. `"check_cargo"`).
#' @param prefix
#'   Prefix of the field (e.g. `"Config/rextendr/`).
#' @param optional
#'   If `TRUE`, return `NA` when there's no field. Otherwise raise an error.
#'
get_desc_field <- function(field, prefix = DESC_FIELD_PREFIX, optional = TRUE) {
  field <- paste0(prefix, field)
  if (length(field) != 1) {
    stop("Field must be length one of character vector")
  }

  # `read.dcf()` always succeeds even when the field is missing.
  # Detect the failure by checking NA
  x <- read.dcf("DESCRIPTION", fields = field)[[1]]

  if (isTRUE(is.na(x)) && !isTRUE(optional)) {
    stop("Failed to get the field ", field, " from DESCRIPTION")
  }

  x
}

DESC_FIELD_PREFIX <- paste0("Config/", get_desc_field("Package", prefix = ""), "/")

# check_cargo -------------------------------------------------------------

#' Check if the cargo command exists and the version is above the requirements
#'
#' @return
#'   `TRUE` invisibly if no error was found.
check_cargo <- function() {
  # Even when `cargo` is on `PATH`, `rustc` might not. We need to source
  # ~/.cargo/env to ensure PATH is configured correctly.
  # (c.f. https://github.com/yutannihilation/string2path/issues/4)
  cargo_env_file <- file.path(Sys.getenv("HOME"), ".cargo", "env")
  if (file.exists(cargo_env_file)) {
    cargo_cmd_tmpl <- sprintf(". %s && cargo %%s", cargo_env_file)
  } else {
    cargo_cmd_tmpl <- "cargo %s"
  }

  ### Check if cargo command works without error ###

  message("*** Checking if cargo is installed")

  cargo_cmd <- sprintf(cargo_cmd_tmpl, "version")

  # version variable might be used for checking MSRV later
  suppressWarnings(version <- system(cargo_cmd, intern = TRUE))

  if (!is.null(attr(version, "status"))) {
    stop(errorCondition("cargo command is not available", class = c("string2path_error_cargo_check", "error")))
  }

  # On Windows, check the toolchain as well.
  if (identical(SYSINFO_OS, "windows")) {
    message("*** Checking if the required Rust toolchain is installed")

    toolchain <- windows_toolchain()
    cargo_cmd <- sprintf(cargo_cmd_tmpl, paste0("+", toolchain, " version"))
    suppressWarnings(ret <- system(cargo_cmd, ignore.stdout = TRUE, ignore.stderr = TRUE))
    if (!identical(ret, 0L)) {
      stop(errorCondition(
        paste("cargo toolchain ", toolchain, " is not installed"),
        class = c("string2path_error_cargo_check", "error")
      ))
    }
  }

  ### Check the version ###

  msrv <- get_desc_field("MSRV", optional = TRUE)
  if (isTRUE(!is.na(msrv))) {
    msrv <- package_version(msrv)

    message("*** Checking if cargo is newer than the required version")

    ptn <- "cargo\\s+(\\d+\\.\\d+\\.\\d+)"
    m <- regmatches(version, regexec(ptn, version))[[1]]

    if (length(m) != 2) {
      stop(errorCondition("cargo version returned unexpected result", class = c("string2path_error_cargo_check", "error")))
    }

    if (package_version(m[2]) < msrv) {
      msg <- sprintf("The installed version of cargo (%s) is older than the requirement (%s)", m[2], msrv)
      stop(errorCondition(msg, class = c("string2path_error_cargo_check", "error")))
    }
  }

  ### Check the toolchains ###
  if (identical(SYSINFO_OS, "windows")) {
    message("*** Checking if the required Rust target is installed")

    expected_targets <- "x86_64-pc-windows-gnu"

    # If there is 32-bit version of R, check 32bit toolchain as well
    if (isTRUE(HAS_32BIT_R)) {
      expected_targets <- c(expected_targets, "i686-pc-windows-gnu")
    }

    suppressWarnings(targets <- system("rustup target list --installed", intern = TRUE))
    unavailable_targets <- setdiff(expected_targets, targets)
    if (length(unavailable_targets) != 0) {
      msg <- sprintf(
        "The required toolchain %s %s not installed",
        paste(unavailable_targets, collapse = " and "),
        if (length(unavailable_targets) == 1) "is" else "are"
      )
      stop(errorCondition(msg, class = c("string2path_error_cargo_check", "error")))
    }
  }

  invisible(TRUE)
}

#' Get the expected Windows toolchain.
#'
#' @return
#'   The expected windows toolchain as a length one of a character vector.
windows_toolchain <- function() {
  x <- get_desc_field("windows_toolchain", optional = TRUE)
  if (isTRUE(!is.na(x))) {
    x
  } else {
    "stable-msvc"
  }
}


# download_precompiled ----------------------------------------------------

#' Download the precompiled binary if available.
download_precompiled <- function() {

  ### Get URLs of precompiled binaries from DESCRIPTION ###

  github_repo <- get_desc_field("github_repo")
  github_tag  <- get_desc_field("github_tag")
  crate_name  <- get_desc_field("crate_name")
  if (isTRUE(is.na(crate_name))) {
    crate_name  <- get_desc_field("Package", prefix = "")
  }

  if (isTRUE(is.na(github_repo) || is.na(github_tag) || is.na(crate_name))) {
    msg <- "No precompiled binary is available as GitHub repository is not specified on the DESCRIPTION file"
    stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
  }

  ### Get checksums from DESCRIPTION ###

  checksums <- get_desc_field("binary_sha256sum")
  if (isTRUE(is.na(checksums))) {
    msg <- sprintf("No precompiled binary is available; the DESCRIPTION file doesn't have %sbinary_sha256sum", DESC_FIELD_PREFIX)
    stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
  }

  tryCatch(
    {
      checksums <- eval(parse(text = checksums))
      stopifnot(is.list(checksums))
    },
    error = function(e) {
      msg <- sprintf("The %sbinary_sha256sum field on the DESCRIPTION file is malformed.", DESC_FIELD_PREFIX)
      stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
    }
  )

  checksums <- data.frame(
    filename  = names(checksums),
    sha256sum = as.character(checksums)
  )

  # For UCRT Windows, add ucrt- prefix
  crt_prefix <- if (isTRUE(USE_UCRT)) "ucrt-" else ""

  download_targets <- character(0)

  if (identical(SYSINFO_OS, "windows")) {
    download_targets <- "x86_64-pc-windows-gnu"

    # If there is 32-bit version installation, download the binary
    if (isTRUE(HAS_32BIT_R)) {
      download_targets <- c(download_targets, "i686-pc-windows-gnu")
    }

    sha256sum_cmd_tmpl <- "sha256sum %s"
  } else if (identical(SYSINFO_OS, "darwin")) {
    download_targets <- switch (SYSINFO_MACHINE,
      x86_64 = "x86_64-apple-darwin",
      arm64  = "aarch64-apple-darwin"
    )
    sha256sum_cmd_tmpl <- "shasum -a 256 %s"
  }

  if (length(download_targets) > 0) {
    # restrict only the available ones
    download_targets <- download_targets[sprintf("%s%s-lib%s.a", crt_prefix, download_targets, crate_name) %in% checksums$filename]
  }

  # If there's no checksum available for the platform, it means there's no binary
  if (length(download_targets) == 0) {
    msg <- sprintf("No precompiled binary is available for { os: %s, arch: %s }",
                   SYSINFO_OS, SYSINFO_MACHINE)
    stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
  }

  ### Construct string templates for download URLs and destfiles ###


  if (identical(SYSINFO_OS, "windows")) {
    # On Windows, --target is specified, so the dir is nested one level deeper
    destfile_tmpl <- paste0("./src/rust/target/%s/release/lib", crate_name, ".a")
  } else {
    destfile_tmpl <- paste0("./src/rust/target/release/lib", crate_name, ".a")
  }

  ### Download the files ###

  for (target in download_targets) {
    src_file <- sprintf("%s%s-lib%s.a", crt_prefix, download_targets, crate_name)
    checksum_expected <- checksums$sha256sum[checksums$filename == src_file]

    src_url <- paste0("https://github.com/", github_repo, "/releases/download/", github_tag, "/", src_file)

    if (identical(SYSINFO_OS, "windows")) {
      # On Windows, --target is specified, so the dir is nested one level deeper
      destfile <- paste0("./src/rust/target/", target, "/release/lib", crate_name, ".a")
    } else {
      destfile <- paste0("./src/rust/target/release/lib", crate_name, ".a")
    }

    message(sprintf("
***
*** Download URL: %s
*** Dest file: %s
***
", src_url, destfile))

    dir.create(dirname(destfile), showWarnings = FALSE, recursive = TRUE)

    # Download the file
    tryCatch(
      download.file(src_url, destfile = destfile, mode = "wb", quiet = TRUE),
      error = function(e) {
        msg <- "Failed to download a precompiled binary"
        stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
      }
    )

    # Get the checksum
    checksum_actual <- system(sprintf(sha256sum_cmd_tmpl, destfile), intern = TRUE)
    if (!is.null(attr(checksum_actual, "status"))) {
      msg <- paste("Failed to get the checksum of", destfile)
      stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
    }

    if (!identical(checksum_expected, checksum_expected)) {
      msg <- paste("Checksum mismatch for the pre-compiled binary: ", target)
      stop(errorCondition(msg, class = c("string2path_error_download_precompiled", "error")))
    }

  } ### End of for loop

  invisible(TRUE)
}

# MAIN --------------------------------------------------------------------

### Check cargo toolchain ###

cargo_check_result <- tryCatch(
  check_cargo(),
  # Defer errors if it's raised by functions here
  string2path_error_cargo_check = function(e) e$message
)

# If cargo is confirmed fine, exit here. But, even if the cargo is not available
# or too old, it's not the end of the world. There might be a pre-compiled
# binary available for the platform.
if (isTRUE(cargo_check_result)) {
  message("\n*** cargo is ok\n")
  quit("no", status = 0)
}

# If ABORT_WHEN_NO_CARGO is set, abort immediately without trying the binaries
if (identical(Sys.getenv("ABORT_WHEN_NO_CARGO"), "true")) {
  message(sprintf("
-------------- ERROR: CONFIGURATION FAILED --------------------

%s and ABORT_WHEN_NO_CARGO is set to true

Please refer to <https://www.rust-lang.org/tools/install> to install Rust.

---------------------------------------------------------------
", cargo_check_result))
  quit("no", status = 100)
}

### Try downloading precompiled binaries ###

download_precompiled_result <- tryCatch(
  download_precompiled(),
  # Defer errors if it's raised by functions here
  string2path_error_download_precompiled = function(e) e$message
)

if (isTRUE(download_precompiled_result)) {
  message("\n*** Successfully downloaded the precompied binary\n")
  quit("no", status = 1)
}


message(sprintf("
-------------- ERROR: CONFIGURATION FAILED --------------------

%s and the precompiled binary is not available

Please refer to <https://www.rust-lang.org/tools/install> to install Rust.

---------------------------------------------------------------
", download_precompiled_result))
quit("no", status = 101)
