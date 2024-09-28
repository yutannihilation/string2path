# the data extracted from font file are as expected

    Code
      string2path("A", "./font/test.ttf")
    Output
      # A tibble: 4 x 4
            x     y glyph_id path_id
        <dbl> <dbl>    <int>   <int>
      1 0     0            1       1
      2 0.800 0.800        1       1
      3 0     0.800        1       1
      4 0     0            1       1

---

    Code
      string2stroke("A", "./font/test.ttf")
    Output
      # A tibble: 18 x 5
               x       y glyph_id path_id triangle_id
           <dbl>   <dbl>    <int>   <int>       <int>
       1  0.836   0.815         1       1           0
       2  0.764   0.785         1       1           0
       3  0.0150  0.785         1       1           0
       4  0.836   0.815         1       1           1
       5  0.0150  0.785         1       1           1
       6 -0.0150  0.815         1       1           1
       7 -0.0150  0.815         1       1           2
       8  0.0150  0.785         1       1           2
       9  0.0150  0.0362        1       1           2
      10 -0.0150  0.815         1       1           3
      11  0.0150  0.0362        1       1           3
      12 -0.0150 -0.0362        1       1           3
      13 -0.0150 -0.0362        1       1           4
      14  0.0150  0.0362        1       1           4
      15  0.764   0.785         1       1           4
      16 -0.0150 -0.0362        1       1           5
      17  0.764   0.785         1       1           5
      18  0.836   0.815         1       1           5

---

    Code
      string2fill("A", "./font/test.ttf")
    Output
      # A tibble: 3 x 5
            x     y glyph_id path_id triangle_id
        <dbl> <dbl>    <int>   <int>       <int>
      1 0     0            1       1           0
      2 0     0.800        1       1           0
      3 0.800 0.800        1       1           0

# the data extracted from installed font are as expected

    Code
      string2path("A", "Arial")
    Output
      # A tibble: 27 x 4
                x     y glyph_id path_id
            <dbl> <dbl>    <int>   <int>
       1 -0.00131 0            1       1
       2  0.245   0.641        1       1
       3  0.336   0.641        1       1
       4  0.598   0            1       1
       5  0.502   0            1       1
       6  0.427   0.194        1       1
       7  0.159   0.194        1       1
       8  0.0887  0            1       1
       9 -0.00131 0            1       1
      10  0.184   0.263        1       2
      # i 17 more rows

---

    Code
      string2stroke("A", "Arial")
    Output
      # A tibble: 150 x 5
             x       y glyph_id path_id triangle_id
         <dbl>   <dbl>    <int>   <int>       <int>
       1 0.255  0.626         1       1           0
       2 0.234  0.656         1       1           0
       3 0.346  0.656         1       1           0
       4 0.255  0.626         1       1           1
       5 0.346  0.656         1       1           1
       6 0.326  0.626         1       1           1
       7 0.326  0.626         1       1           2
       8 0.346  0.656         1       1           2
       9 0.621 -0.0150        1       1           2
      10 0.326  0.626         1       1           3
      # i 140 more rows

---

    Code
      string2fill("A", "Arial")
    Output
      # A tibble: 75 x 5
                x     y glyph_id path_id triangle_id
            <dbl> <dbl>    <int>   <int>       <int>
       1  0.0887  0            1       1           0
       2 -0.00131 0            1       1           0
       3  0.159   0.194        1       1           0
       4  0.427   0.194        1       1           1
       5  0.159   0.194        1       1           1
       6  0.184   0.263        1       2           1
       7  0.159   0.194        1       1           2
       8 -0.00131 0            1       1           2
       9  0.184   0.263        1       2           2
      10  0.184   0.263        1       2           3
      # i 65 more rows

