This is a third submission. I'm sorry that the previous submission didn't
succeed to fix a linker warning on M1 macOS about an object file built for a
newer macOS version than being linked. This is now fixed by setting
`MACOSX_DEPLOYMENT_TARGET` in `configure` before invoking cargo.
