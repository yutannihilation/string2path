This is a resubmission. The previous submission had a linker warning on
M1 macOS about an object file built for a newer macOS version than being
linked. This was because the C compiler flags (including
`-mmacos-version-min`) were not passed through to a C file compiled
during the Rust build. This should be now fixed.
