#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

#include "string2path/api.h"

#include <strings.h>

SEXP string2path_impl(SEXP str, SEXP ttf_file, SEXP tolerance) {
  Result res = string2path(
    Rf_translateCharUTF8(STRING_ELT(str, 0)),
    Rf_translateCharUTF8(STRING_ELT(ttf_file, 0)),
    Rf_asReal(tolerance)
  );

  if (res.length == 0) {
    Rf_warning("Failed to convert to path");
    return NULL;
  }

  // Convert the result to SEXP vectors
  SEXP x = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(x), res.x, res.length * sizeof(double));
  SEXP y = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(y), res.y, res.length * sizeof(double));
  SEXP id = PROTECT(Rf_allocVector(INTSXP, res.length));
  memcpy(INTEGER(id), res.id, res.length * sizeof(uint32_t));
  SEXP glyph_id = PROTECT(Rf_allocVector(INTSXP, res.length));
  memcpy(INTEGER(glyph_id), res.glyph_id, res.length * sizeof(uint32_t));

  free_result(res);

  // bundle the result to one list
  SEXP out = PROTECT(allocVector(VECSXP, 4));
  SET_VECTOR_ELT(out, 0, x);
  SET_VECTOR_ELT(out, 1, y);
  SET_VECTOR_ELT(out, 2, id);
  SET_VECTOR_ELT(out, 3, glyph_id);

  UNPROTECT(5);
  return out;
}

SEXP string2vertex_impl(SEXP str, SEXP ttf_file, SEXP tolerance, SEXP result_type) {
  Result res = string2vertex(
    Rf_translateCharUTF8(STRING_ELT(str, 0)),
    Rf_translateCharUTF8(STRING_ELT(ttf_file, 0)),
    Rf_asReal(tolerance),
    Rf_asInteger(result_type)
  );

  if (res.length == 0) {
    // TODO: If the length is zero, no memory was mapped, so freeing res causes segfault.
    //       But, maybe Result struct itself is mapped? I'm not sure how to handle it properly.
    return NULL;
  }

  // Convert the result to SEXP vectors
  SEXP x = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(x), res.x, res.length * sizeof(double));
  SEXP y = PROTECT(Rf_allocVector(REALSXP, res.length));
  memcpy(REAL(y), res.y, res.length * sizeof(double));
  SEXP id = PROTECT(Rf_allocVector(INTSXP, res.length));
  memcpy(INTEGER(id), res.id, res.length * sizeof(uint32_t));
  SEXP glyph_id = PROTECT(Rf_allocVector(INTSXP, res.length));
  memcpy(INTEGER(glyph_id), res.glyph_id, res.length * sizeof(uint32_t));

  free_result(res);

  // bundle the result to one list
  SEXP out = PROTECT(allocVector(VECSXP, 4));
  SET_VECTOR_ELT(out, 0, x);
  SET_VECTOR_ELT(out, 1, y);
  SET_VECTOR_ELT(out, 2, id);
  SET_VECTOR_ELT(out, 3, glyph_id);

  UNPROTECT(5);
  return out;
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"string2path_impl", (DL_FUNC) &string2path_impl, 3},
  {"string2vertex_impl", (DL_FUNC) &string2vertex_impl, 3},
  {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
