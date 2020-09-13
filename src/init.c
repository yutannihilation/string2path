#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

// Import C headers for rust API
#include "string2path/api.h"

// Actual Wrappers
// SEXP hello_wrapper(){
//   return Rf_ScalarString(Rf_mkCharCE(string_from_rust(), CE_UTF8));
// }

SEXP string2path_glyph2digit(SEXP str, SEXP ttf_file) {
  return Rf_ScalarInteger(
    glyph2digit(
      Rf_translateCharUTF8(STRING_ELT(str, 0)),
      Rf_translateCharUTF8(STRING_ELT(ttf_file, 0))
    )
  );
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"string2path_glyph2digit", (DL_FUNC) &string2path_glyph2digit, 2},
  {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
