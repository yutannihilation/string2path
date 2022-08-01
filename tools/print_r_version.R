devel_prefix <- if(isTRUE(grepl("devel", R.version$status, fixed = TRUE))) "-devel" else ""
r_version <- sprintf("%s.%s%s", R.version$major, R.version$minor, devel_prefix)
cat(r_version)
