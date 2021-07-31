# the extracted data are expected

    Code
      string2path("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <dbl>   <dbl>
      1     0     0        0       0
      2   100   100        0       0
      3     0   100        0       0
      4     0     0        0       0

---

    Code
      string2stroke("A", "./font/test.ttf")
    Output
      # A tibble: 18 x 5
             x     y glyph_id path_id triangle_id
         <dbl> <dbl>    <dbl>   <dbl>       <dbl>
       1 112.  105          0       0           0
       2  87.9  95          0       0           0
       3   5    95          0       0           0
       4 112.  105          0       0           1
       5   5    95          0       0           1
       6  -5   105          0       0           1
       7  -5   105          0       0           2
       8   5    95          0       0           2
       9   5    12.1        0       0           2
      10  -5   105          0       0           3
      11   5    12.1        0       0           3
      12  -5   -12.1        0       0           3
      13  -5   -12.1        0       0           4
      14   5    12.1        0       0           4
      15 112.  105          0       0           4
      16   5    12.1        0       0           5
      17  87.9  95          0       0           5
      18 112.  105          0       0           5

---

    Code
      string2fill("A", "./font/test.ttf")
    Output
      # A tibble: 3 x 5
            x     y glyph_id path_id triangle_id
        <dbl> <dbl>    <dbl>   <dbl>       <dbl>
      1     0     0        0       0           0
      2     0   100        0       0           0
      3   100   100        0       0           0

