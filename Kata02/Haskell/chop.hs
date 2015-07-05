import Data.Vector
import Test.HUnit

chopRecurse :: (Ord a) => Vector a -> a -> Int -> Int -> Int
chopRecurse xs item low width
  | width <= 0 = -1
  | xs ! index > item = chopRecurse xs item low (width `quot` 2)
  | xs ! index < item = chopRecurse xs item (index + 1) ((width - 1) `quot` 2)
  | otherwise = index
  where index = low + width `quot` 2

chop :: (Ord a) => Vector a -> a -> Int
chop xs item = chopRecurse xs item 0 (Data.Vector.length xs)

-- Tests
test1 = TestCase (do let vec = empty
                     assertEqual "chop 3 vec[]" (-1) (chop vec 3))
test2 = TestCase (do let vec = generate (1) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1]" (-1) (chop vec 3))
test3 = TestCase (do let vec = generate  (1) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1]" 0 (chop vec 1))
test4 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1, 3, 5]" 0 (chop vec 1))
test5 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1, 3, 5]" 1 (chop vec 3))
test6 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 5 vec[1, 3, 5]" 2 (chop vec 5))
test7 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 0 vec[1, 3, 5]" (-1) (chop vec 0))
test8 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 2 vec[1, 3, 5]" (-1) (chop vec 2))
test9 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 4 vec[1, 3, 5]" (-1) (chop vec 4))
test10 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                      assertEqual "chop 6 vec[1, 3, 5]" (-1) (chop vec 6))
test11 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 1 vec[1, 3, 5, 7]" 0 (chop vec 1))
test12 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 3 vec[1, 3, 5, 7]" 1 (chop vec 3))
test13 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 5 vec[1, 3, 5, 7]" 2 (chop vec 5))
test14 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 7 vec[1, 3, 5, 7]" 3 (chop vec 7))
test15 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 0 vec[1, 3, 5, 7]" (-1) (chop vec 0))
test16 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 2 vec[1, 3, 5, 7]" (-1) (chop vec 2))
test17 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 4 vec[1, 3, 5, 7]" (-1) (chop vec 4))
test18 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 6 vec[1, 3, 5, 7]" (-1) (chop vec 6))
test19 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 8 vec[1, 3, 5, 7]" (-1) (chop vec 8))

tests = TestList [TestLabel "test1" test1,
                  TestLabel "test2" test2,
                  TestLabel "test3" test3,
                  TestLabel "test4" test4,
                  TestLabel "test5" test5,
                  TestLabel "test6" test6,
                  TestLabel "test7" test7,
                  TestLabel "test8" test8,
                  TestLabel "test9" test9,
                  TestLabel "test10" test10,
                  TestLabel "test11" test11,
                  TestLabel "test12" test12,
                  TestLabel "test13" test13,
                  TestLabel "test14" test14,
                  TestLabel "test15" test15,
                  TestLabel "test16" test16,
                  TestLabel "test17" test17,
                  TestLabel "test18" test18,
                  TestLabel "test19" test19]

main = runTestTT tests
