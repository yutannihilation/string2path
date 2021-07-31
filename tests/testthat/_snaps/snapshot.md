# the extracted data are expected

    Code
      string2path("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <dbl>   <dbl>
      1 0     0            0       0
      2 0.800 0.800        0       0
      3 0     0.800        0       0
      4 0     0            0       0

---

    Code
      string2stroke("A", "./font/test.ttf")
    Output
      # A tibble: 18 x 5
                x       y glyph_id path_id triangle_id
            <dbl>   <dbl>    <dbl>   <dbl>       <dbl>
       1  0.812    0.805         0       0           0
       2  0.788    0.795         0       0           0
       3  0.00500  0.795         0       0           0
       4  0.812    0.805         0       0           1
       5  0.00500  0.795         0       0           1
       6 -0.00500  0.805         0       0           1
       7 -0.00500  0.805         0       0           2
       8  0.00500  0.795         0       0           2
       9  0.00500  0.0121        0       0           2
      10 -0.00500  0.805         0       0           3
      11  0.00500  0.0121        0       0           3
      12 -0.00500 -0.0121        0       0           3
      13 -0.00500 -0.0121        0       0           4
      14  0.00500  0.0121        0       0           4
      15  0.812    0.805         0       0           4
      16  0.00500  0.0121        0       0           5
      17  0.788    0.795         0       0           5
      18  0.812    0.805         0       0           5

---

    Code
      string2fill("A", "./font/test.ttf")
    Output
      # A tibble: 3 x 5
            x     y glyph_id path_id triangle_id
        <dbl> <dbl>    <dbl>   <dbl>       <dbl>
      1 0     0            0       0           0
      2 0     0.800        0       0           0
      3 0.800 0.800        0       0           0
