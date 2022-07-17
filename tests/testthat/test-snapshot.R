test_that("the extracted data are expected", {
  skip_if_not(isTRUE("Arial" %in% dump_fontdb()$family))

  expect_snapshot(string2path("A", "Arial"))
  expect_snapshot(string2stroke("A", "Arial"))
  expect_snapshot(string2fill("A", "Arial"))
})
