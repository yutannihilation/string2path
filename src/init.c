#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

#include "string2path/api.h"

#include <strings.h>

SEXP string2path_impl(SEXP str, SEXP ttf_file) {
  Result res = string2path(
    Rf_translateCharUTF8(STRING_ELT(str, 0)),
    Rf_translateCharUTF8(STRING_ELT(ttf_file, 0))
  );

  // Convert the result to SEXP vectors
  SEXP x = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(x), res.x, res.length * sizeof(double));
  SEXP y = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(y), res.y, res.length * sizeof(double));
  SEXP id = PROTECT(Rf_allocVector(INTSXP, res.length));
  memcpy(INTEGER(id), res.id, res.length * sizeof(uint32_t));

  free_result(res);

  // bundle the result to one list
  SEXP out = PROTECT(allocVector(VECSXP, 3));
  SET_VECTOR_ELT(out, 0, x);
  SET_VECTOR_ELT(out, 1, y);
  SET_VECTOR_ELT(out, 2, id);

  UNPROTECT(4);
  return out;
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"string2path_impl", (DL_FUNC) &string2path_impl, 2},
  {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
