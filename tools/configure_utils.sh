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
#     check_cargo [TOOLCHAIN] [TARGET_1 TARGET_2...]
#
# ARGS:
#     TOOLCHAIN   Toolchain that must be installed (i.e. stable-msvc on Windows)
#     TARGET_n    Targets that must be installed
check_cargo() {
  TOOLCHAIN="$1"
  shift
  TARGETS="$@"

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

  if [ -n "${TOOLCHAIN}" ]; then
    check_cargo_toolchain "${TOOLCHAIN}"
    ret=$?
    if [ "${ret}" -ne 0 ]; then
      return ${ret}
    fi
  fi

  for TARGET in ${TARGETS}; do
    check_cargo_target ${TARGET}
    ret=$?
    if [ "${ret}" -ne 0 ]; then
      return ${ret}
    fi
  done

  return 0
}

check_cargo_toolchain() {
  EXPECTED_TOOLCHAIN="$1"

  cargo "+${EXPECTED_TOOLCHAIN}" version >/dev/null 2>&1
  if [ $? -ne 0 ]; then
    echo ""
    echo "WARN: ${EXPECTED_TOOLCHAIN} toolchain is not installed"
    echo ""
    return 3
  fi
}

check_cargo_target() {
  EXPECTED_TARGET="$1"

  if ! rustup target list --installed | grep -q "${EXPECTED_TARGET}"; then
    echo ""
    echo "WARN: target ${EXPECTED_TARGET} is not installed"
    echo ""
    return 4
  fi
}
