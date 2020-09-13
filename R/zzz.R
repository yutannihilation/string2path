.onUnload <- function(libpath) {
  library.dynam.unload("string2path", libpath)
}
