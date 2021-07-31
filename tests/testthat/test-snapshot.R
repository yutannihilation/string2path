test_that("the extracted data are expected", {
  expect_snapshot(string2path("A", "../../src/rust/test/font/test.ttf"))
  expect_snapshot(string2stroke("A", "../../src/rust/test/font/test.ttf"))
  expect_snapshot(string2fill("A", "../../src/rust/test/font/test.ttf"))
})
