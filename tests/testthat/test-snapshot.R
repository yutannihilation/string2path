test_that("the extracted data are expected", {
  expect_snapshot(string2path("A", "./font/test.ttf"))
  expect_snapshot(string2stroke("A", "./font/test.ttf"))
  expect_snapshot(string2fill("A", "./font/test.ttf"))
})
