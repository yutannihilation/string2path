#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

// Import C headers for rust API
#include "string2path/api.h"

SEXP glyph2digit_impl(SEXP str, SEXP ttf_file) {
  return Rf_ScalarInteger(
    glyph2digit(
      Rf_translateCharUTF8(STRING_ELT(str, 0)),
      Rf_translateCharUTF8(STRING_ELT(ttf_file, 0))
    )
  );
}

SEXP string2path_impl(SEXP str, SEXP ttf_file) {
  Result res = string2path(
    Rf_translateCharUTF8(STRING_ELT(str, 0)),
    Rf_translateCharUTF8(STRING_ELT(ttf_file, 0))
  );

  SEXP out = PROTECT(Rf_allocVector(REALSXP, res.length));
  for (int i = 0; i < res.length; i++) {
    SET_REAL_ELT(out, i, res.data[i]);
  }
  UNPROTECT(1);
  return out;
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"glyph2digit_impl", (DL_FUNC) &glyph2digit_impl, 2},
  {"string2path_impl", (DL_FUNC) &string2path_impl, 2},
  {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
