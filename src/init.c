
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

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
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP savvy_string2path_family__impl(SEXP c_arg__text, SEXP c_arg__font_family, SEXP c_arg__font_weight, SEXP c_arg__font_style, SEXP c_arg__tolerance) {
    SEXP res = savvy_string2path_family__ffi(c_arg__text, c_arg__font_family, c_arg__font_weight, c_arg__font_style, c_arg__tolerance);
    return handle_result(res);
}

SEXP savvy_string2path_file__impl(SEXP c_arg__text, SEXP c_arg__font_file, SEXP c_arg__tolerance) {
    SEXP res = savvy_string2path_file__ffi(c_arg__text, c_arg__font_file, c_arg__tolerance);
    return handle_result(res);
}

SEXP savvy_string2stroke_family__impl(SEXP c_arg__text, SEXP c_arg__font_family, SEXP c_arg__font_weight, SEXP c_arg__font_style, SEXP c_arg__tolerance, SEXP c_arg__line_width) {
    SEXP res = savvy_string2stroke_family__ffi(c_arg__text, c_arg__font_family, c_arg__font_weight, c_arg__font_style, c_arg__tolerance, c_arg__line_width);
    return handle_result(res);
}

SEXP savvy_string2stroke_file__impl(SEXP c_arg__text, SEXP c_arg__font_file, SEXP c_arg__tolerance, SEXP c_arg__line_width) {
    SEXP res = savvy_string2stroke_file__ffi(c_arg__text, c_arg__font_file, c_arg__tolerance, c_arg__line_width);
    return handle_result(res);
}

SEXP savvy_string2fill_family__impl(SEXP c_arg__text, SEXP c_arg__font_family, SEXP c_arg__font_weight, SEXP c_arg__font_style, SEXP c_arg__tolerance) {
    SEXP res = savvy_string2fill_family__ffi(c_arg__text, c_arg__font_family, c_arg__font_weight, c_arg__font_style, c_arg__tolerance);
    return handle_result(res);
}

SEXP savvy_string2fill_file__impl(SEXP c_arg__text, SEXP c_arg__font_file, SEXP c_arg__tolerance) {
    SEXP res = savvy_string2fill_file__ffi(c_arg__text, c_arg__font_file, c_arg__tolerance);
    return handle_result(res);
}

SEXP savvy_dump_fontdb_impl__impl(void) {
    SEXP res = savvy_dump_fontdb_impl__ffi();
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_string2path_family__impl", (DL_FUNC) &savvy_string2path_family__impl, 5},
    {"savvy_string2path_file__impl", (DL_FUNC) &savvy_string2path_file__impl, 3},
    {"savvy_string2stroke_family__impl", (DL_FUNC) &savvy_string2stroke_family__impl, 6},
    {"savvy_string2stroke_file__impl", (DL_FUNC) &savvy_string2stroke_file__impl, 4},
    {"savvy_string2fill_family__impl", (DL_FUNC) &savvy_string2fill_family__impl, 5},
    {"savvy_string2fill_file__impl", (DL_FUNC) &savvy_string2fill_file__impl, 3},
    {"savvy_dump_fontdb_impl__impl", (DL_FUNC) &savvy_dump_fontdb_impl__impl, 0},
    {NULL, NULL, 0}
};

void R_init_string2path(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
