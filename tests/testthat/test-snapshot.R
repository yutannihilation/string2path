test_that("the extracted data are expected", {
  expect_snapshot(string2path("A", "Arial"))
  expect_snapshot(string2stroke("A", "Arial"))
  expect_snapshot(string2fill("A", "Arial"))
})
