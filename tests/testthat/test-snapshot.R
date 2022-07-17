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
