# the data extracted from font file are as expected

    Code
      string2path("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <int>   <int>
      1 0     0            0       0
      2 0.800 0.800        0       0
      3 0     0.800        0       0
      4 0     0            0       0

---

    Code
      string2stroke("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <int>   <int>
      1 0     0            0       0
      2 0.800 0.800        0       0
      3 0     0.800        0       0
      4 0     0            0       0

---

    Code
      string2fill("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <int>   <int>
      1 0     0            0       0
      2 0.800 0.800        0       0
      3 0     0.800        0       0
      4 0     0            0       0

# the data extracted from installed font are as expected

    Code
      string2path("A", "Arial")
    Output
      # A tibble: 25 x 4
                x     y glyph_id path_id
            <dbl> <dbl>    <int>   <int>
       1 -0.00131 0            0       0
       2  0.245   0.641        0       0
       3  0.336   0.641        0       0
       4  0.598   0            0       0
       5  0.502   0            0       0
       6  0.427   0.194        0       0
       7  0.159   0.194        0       0
       8  0.0887  0            0       0
       9 -0.00131 0            0       0
      10  0.184   0.263        0       1
      # i 15 more rows

---

    Code
      string2stroke("A", "Arial")
    Output
      # A tibble: 150 x 5
             x       y glyph_id path_id triangle_id
         <dbl>   <dbl>    <int>   <int>       <int>
       1 0.255  0.626         0       0           0
       2 0.234  0.656         0       0           0
       3 0.346  0.656         0       0           0
       4 0.255  0.626         0       0           1
       5 0.346  0.656         0       0           1
       6 0.326  0.626         0       0           1
       7 0.326  0.626         0       0           2
       8 0.346  0.656         0       0           2
       9 0.621 -0.0150        0       0           2
      10 0.326  0.626         0       0           3
      # i 140 more rows

---

    Code
      string2fill("A", "Arial")
    Output
      # A tibble: 75 x 5
                x     y glyph_id path_id triangle_id
            <dbl> <dbl>    <int>   <int>       <int>
       1  0.0887  0            0       0           0
       2 -0.00131 0            0       0           0
       3  0.159   0.194        0       0           0
       4  0.427   0.194        0       0           1
       5  0.159   0.194        0       0           1
       6  0.184   0.263        0       1           1
       7  0.159   0.194        0       0           2
       8 -0.00131 0            0       0           2
       9  0.184   0.263        0       1           2
      10  0.184   0.263        0       1           3
      # i 65 more rows

