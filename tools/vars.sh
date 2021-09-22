GITHUB_REPO="yutannihilation/string2path"
CRATE_NAME="string2path"

# These need to be updated if the binary is changed
GITHUB_TAG="build_20210921-1"
SHA256SUM_MAC_INTEL="be65f074cb7ae50e5784e7650f48579fff35f30ff663d1c01eabdc9f35c1f87c"
SHA256SUM_MAC_ARM="4a34f99cec66610746b20d456b1e11b346596c22ea1935c1bcb5ef1ab725f0e8"
SHA256SUM_WIN_64="26a05f6ee8c2f625027ffc77c97fc8ac9746a182f5bc53d64235999a02c0b0dc"
SHA256SUM_WIN_32="ceda54184fb3bf9e4cbba86848cb2091ff5b77870357f94319f9215fadfa5b25"

# 1.41.0 is the min version among Debian and Ubuntu at the time of writing this
# (c.f. https://github.com/ron-rs/ron/issues/256#issuecomment-657999081)
# This might need bumped depending on the features the crate uses. cargo-msrv
# might be helpful to find the actual MSRV.
MIN_RUST_VERSION="1.56.0"
