SEXP string2path_family(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance);
SEXP string2path_file(SEXP text, SEXP font_file, SEXP tolerance);
SEXP string2stroke_family(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance, SEXP line_width);
SEXP string2stroke_file(SEXP text, SEXP font_file, SEXP tolerance, SEXP line_width);
SEXP string2fill_family(SEXP text, SEXP font_family, SEXP font_weight, SEXP font_style, SEXP tolerance);
SEXP string2fill_file(SEXP text, SEXP font_file, SEXP tolerance);
SEXP dump_fontdb_impl();
