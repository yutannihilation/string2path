
#include <stdint.h>
#include <Rinternals.h>
#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_error() directly.
            Rf_error("%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}


SEXP savvy_string2path_family_wrapper(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance) {
    SEXP res = savvy_string2path_family(text, font_family, font_weight, font_style, tolerance);
    return handle_result(res);
}

SEXP savvy_string2path_file_wrapper(SEXP text, SEXP font_file, SEXP tolerance) {
    SEXP res = savvy_string2path_file(text, font_file, tolerance);
    return handle_result(res);
}

SEXP savvy_string2stroke_family_wrapper(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance, SEXP line_width) {
    SEXP res = savvy_string2stroke_family(text, font_family, font_weight, font_style, tolerance, line_width);
    return handle_result(res);
}

SEXP savvy_string2stroke_file_wrapper(SEXP text, SEXP font_file, SEXP tolerance, SEXP line_width) {
    SEXP res = savvy_string2stroke_file(text, font_file, tolerance, line_width);
    return handle_result(res);
}

SEXP savvy_string2fill_family_wrapper(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance) {
    SEXP res = savvy_string2fill_family(text, font_family, font_weight, font_style, tolerance);
    return handle_result(res);
}

SEXP savvy_string2fill_file_wrapper(SEXP text, SEXP font_file, SEXP tolerance) {
    SEXP res = savvy_string2fill_file(text, font_file, tolerance);
    return handle_result(res);
}

SEXP savvy_dump_fontdb_impl_wrapper() {
    SEXP res = savvy_dump_fontdb_impl();
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_string2path_family", (DL_FUNC) &savvy_string2path_family_wrapper, 5},
    {"savvy_string2path_file", (DL_FUNC) &savvy_string2path_file_wrapper, 3},
    {"savvy_string2stroke_family", (DL_FUNC) &savvy_string2stroke_family_wrapper, 6},
    {"savvy_string2stroke_file", (DL_FUNC) &savvy_string2stroke_file_wrapper, 4},
    {"savvy_string2fill_family", (DL_FUNC) &savvy_string2fill_family_wrapper, 5},
    {"savvy_string2fill_file", (DL_FUNC) &savvy_string2fill_file_wrapper, 3},
    {"savvy_dump_fontdb_impl", (DL_FUNC) &savvy_dump_fontdb_impl_wrapper, 0},

    {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
