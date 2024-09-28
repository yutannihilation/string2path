test_that("the data extracted from font file are as expected", {
  expect_snapshot(string2path("A", "./font/test.ttf"))
  expect_snapshot(string2stroke("A", "./font/test.ttf"))
  expect_snapshot(string2fill("A", "./font/test.ttf"))
})

test_that("the data extracted from installed font are as expected", {
  skip_if_not(isTRUE("Arial" %in% dump_fontdb()$family))

  expect_snapshot(string2path("A", "Arial"))
  expect_snapshot(string2stroke("A", "Arial"))
  expect_snapshot(string2fill("A", "Arial"))
})

test_that("path and fill match", {
  skip_if_not(isTRUE("Arial" %in% dump_fontdb()$family))
  skip_if_not_installed("vdiffr")

  do_plot_s <- function() {
    p <- string2path("s", "Arial", tolerance = 3e-3)
    f <- string2fill("s", "Arial", tolerance = 3e-3)
    
    plot(NULL, xlim = c(0, 0.45), ylim = c(0, 0.45))
    for (tri in split(f, f$triangle_id)) {
      polygon(tri$x, tri$y, col = "#ff3344", border = "transparent")
    }
    lines(p$x, p$y, lwd = 10)
  }
  vdiffr::expect_doppelganger("path and fill match", do_plot_s())
})
