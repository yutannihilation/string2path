if [ -z "${RSCRIPT}" ]; then
  echo ""
  echo "ERROR: RSCRIPT variable needs to be set before sourcing configure_utils.sh"
  echo ""
  exit 100
fi

# Use these system information to do some extra checks
SYSINFO_MACHINE=`"${RSCRIPT}" -e 'cat(Sys.info()[["machine"]])'`
SYSINFO_OS=`"${RSCRIPT}" -e 'cat(tolower(Sys.info()[["sysname"]]))'`

echo ""
echo "SYSINFO_MACHINE:   ${SYSINFO_MACHINE}"
echo "SYSINFO_OS:        ${SYSINFO_OS}"
echo ""

# "true" if there's 32bit version of R installed
if [ -d "${R_HOME}/bin/i386/" ]; then
  HAS_32BIT_R="true"
fi


# Show error messages and exit
#
# USAGE:
#     show_error MSG EXIT_CODE
#
# ARGS:
#     MSG         Additional error message to show
#     EXIT_CODE   Exit code to exit with
show_error() {
  echo "-------------- ERROR: CONFIGURATION FAILED --------------------"
  echo ""
  echo "$1"
  echo "Please refer to <https://www.rust-lang.org/tools/install> to install Rust."
  echo ""
  echo "---------------------------------------------------------------"
  echo ""

  exit $2
}



# Check if cargo is installed and set up as expected
#
# USAGE:
#     check_cargo
#
# VARIABLES:
#     MIN_RUST_VERSION   If this is set, check if the installed version is newer
#                        than the requirement
#
check_cargo() {
  echo "*** Checking if cargo is installed"

  cargo version >/dev/null 2>&1
  if [ $? -ne 0 ]; then
    echo ""
    echo "WARN: cargo command is not available"
    echo ""
    return 1
  fi

  if [ -n "${MIN_RUST_VERSION}" ]; then
    echo "*** Checking if cargo is newer than the required version"

    # Check if the version is minimum required one. `-V` option of `sort` does
    # version sort, and `-C` is for silently checking if the input is already
    # sorted; so, if RUST_VERSION is smaller than MIN_RUST_VERSION, it fails.
    RUST_VERSION="`cargo --version | cut -d' ' -f2`"
    if ! printf '%s\n' "${MIN_RUST_VERSION}" "${RUST_VERSION}" | sort -C -V; then
      echo ""
      echo "WARN: The installed version of cargo (${RUST_VERSION}) is older than the requirement (${MIN_RUST_VERSION})"
      echo ""
      return 2
    fi
  fi

  # On Windows, there should be installed an specific toolchains
  if [ "${SYSINFO_OS}" = "windows" ]; then

    # Check toolchain ------

    _check_cargo_toolchain stable-msvc
    ret=$?
    if [ "${ret}" -ne 0 ]; then
      return ${ret}
    fi

    # Check targets ------

    # If there is 32-bit version of R, check the corresponding target is installed already
    if [ "${HAS_32BIT_R}" = "true" ]; then
      TARGETS="x86_64-pc-windows-gnu i686-pc-windows-gnu"
    else
      TARGETS="x86_64-pc-windows-gnu"
    fi

    for TARGET in ${TARGETS}; do
      _check_cargo_target ${TARGET}
      ret=$?
      if [ "${ret}" -ne 0 ]; then
        return ${ret}
      fi
    done
  fi

  echo "cargo is ok"
  echo ""

  return 0
}

# Check if the installed cargo has a specific toolchain
#
# (This is intended to be used in check_cargo)
#
# USAGE:
#     _check_cargo_toolchain TOOLCHAIN
#
# ARGS:
#     TOOLCHAIN   Toolchain that must be installed (i.e. stable-msvc on Windows)
_check_cargo_toolchain() {
  EXPECTED_TOOLCHAIN="$1"

  cargo "+${EXPECTED_TOOLCHAIN}" version >/dev/null 2>&1
  if [ $? -ne 0 ]; then
    echo ""
    echo "WARN: ${EXPECTED_TOOLCHAIN} toolchain is not installed"
    echo ""
    return 3
  fi
}

# Check if the installed cargo has a specific target
#
# (This is intended to be used in check_cargo)
#
# USAGE:
#     _check_cargo_target TARGET
#
# ARGS:
#     TARGET      Targets that must be installed
_check_cargo_target() {
  EXPECTED_TARGET="$1"

  if ! rustup target list --installed | grep -q "${EXPECTED_TARGET}"; then
    echo ""
    echo "WARN: target ${EXPECTED_TARGET} is not installed"
    echo ""
    return 4
  fi
}




SRC_URL_TMPL="https://github.com/${GITHUB_REPO}/releases/download/${GITHUB_TAG}/${CRT_PREFIX}%s-lib${CRATE_NAME}.a"
if [ "${SYSINFO_OS}" = "windows" ]; then
  # On Windows, --target is specified
  DESTFILE_TMPL="./src/rust/target/%s/release/lib${CRATE_NAME}.a"
else
  DESTFILE_TMPL="./src/rust/target/release/lib${CRATE_NAME}.a"
fi

# Download a binary
#
# (This is intended to be used in check_cargo)
#
# USAGE:
#     _check_cargo_target RUST_TARGET SHA256SUM_EXPECTED
#
# ARGS:
#     RUST_TARGET          Build target
#     SHA256SUM_EXPECTED   Checksum of the downloaded binary
_download_binary() {
  RUST_TARGET="$1"
  SHA256SUM_EXPECTED="$2"

  SRC_URL=`printf "${SRC_URL_TMPL}" "${RUST_TARGET}"`
  DESTFILE=`printf "${DESTFILE_TMPL}" "${RUST_TARGET}"`

  echo "*** Download URL: ${SRC_URL}"
  echo "*** Dest file: ${DESTFILE}"

  # curl or wget might not be portable, so use R to download the file
  "${RSCRIPT}" ./tools/download_precompiled_binary.R "${SRC_URL}" "${DESTFILE}"

  if [ $? -ne 0 ]; then
    show_error "Failed to download the pre-compiled binary" 12
  fi

  # Verify the checksum
  SHA256SUM_ACTUAL=`sha256sum "${DST}" | cut -d' ' -f1`
  if [ -z "${SHA256SUM_ACTUAL}" ]; then
    show_error "Failed to get the checksum" 13
  fi

  if [ "${SHA256SUM_ACTUAL}" != "${SHA256SUM_EXPECTED}" ]; then
    show_error "Checksum mismatch for the pre-compiled binary" 14
  fi
}

# Download the precompiled binaries
#
# USAGE:
#     download_binaries
download_binaries() {
  echo "*** Trying to download the precompiled binary"

  # For debugging purpose
  if [ "${DEBUG_MUST_COMPILE}" = "true" ]; then
    echo "Should not reach download_binaries"
    exit 100
  fi

  case "${SYSINFO_OS}" in

    windows) ##################################################

      # Download 64-bit binary
      _download_binary "x86_64-pc-windows-gnu" "${SHA256SUM_WIN_64}"

      # If there are 32-bit version installation, download the binary
      if [ "${HAS_32BIT_R}" = "true" ]; then
        _download_binary "i686-pc-windows-gnu" "${SHA256SUM_WIN_32}"
      fi
    ;; # end of windows case ##################################

    darwin) ###################################################
      case "${SYSINFO_MACHINE}" in
      x86_64)
        _download_binary "x86_64-apple-darwin" "${SHA256SUM_MAC_INTEL}"
        ;;
      arm64)
        _download_binary "aarch64-apple-darwin" "${SHA256SUM_MAC_ARM}"
        ;;
      *)
        show_error "ERROR: No precompiled binary is available for arch ${SYSINFO_MACHINE}" 11
      esac
    ;; # end of macOS case ####################################

    *)
        show_error "ERROR: No precompiled binary is available for OS ${SYSINFO_OS}" 11
  esac

  echo "Successfully downloaded the precompied binary"
  echo ""
}
