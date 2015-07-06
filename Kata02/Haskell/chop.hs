import Data.Vector
import Test.HUnit

chopRecurse :: (Ord a) => a -> Vector a -> Int -> Int -> Int
chopRecurse item xs low width
  | width <= 0 = -1
  | xs ! index > item = chopRecurse item xs low (width `quot` 2)
  | xs ! index < item = chopRecurse item xs (index + 1) ((width - 1) `quot` 2)
  | otherwise = index
  where index = low + width `quot` 2

chop :: (Ord a) => a -> Vector a -> Int
chop item xs = chopRecurse item xs 0 (Data.Vector.length xs)

-- Tests
test1 = TestCase (do let vec = empty
                     assertEqual "chop 3 vec[]" (-1) (chop 3 vec))
test2 = TestCase (do let vec = generate (1) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1]" (-1) (chop 3 vec))
test3 = TestCase (do let vec = generate  (1) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1]" 0 (chop 1 vec))
test4 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 1 vec[1, 3, 5]" 0 (chop 1 vec))
test5 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 3 vec[1, 3, 5]" 1 (chop 3 vec))
test6 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 5 vec[1, 3, 5]" 2 (chop 5 vec))
test7 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 0 vec[1, 3, 5]" (-1) (chop 0 vec))
test8 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 2 vec[1, 3, 5]" (-1) (chop 2 vec))
test9 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                     assertEqual "chop 4 vec[1, 3, 5]" (-1) (chop 4 vec))
test10 = TestCase (do let vec = generate  (3) (\i -> i * 2 + 1)
                      assertEqual "chop 6 vec[1, 3, 5]" (-1) (chop 6 vec))
test11 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 1 vec[1, 3, 5, 7]" 0 (chop 1 vec))
test12 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 3 vec[1, 3, 5, 7]" 1 (chop 3 vec))
test13 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 5 vec[1, 3, 5, 7]" 2 (chop 5 vec))
test14 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 7 vec[1, 3, 5, 7]" 3 (chop 7 vec))
test15 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 0 vec[1, 3, 5, 7]" (-1) (chop 0 vec))
test16 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 2 vec[1, 3, 5, 7]" (-1) (chop 2 vec))
test17 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 4 vec[1, 3, 5, 7]" (-1) (chop 4 vec))
test18 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 6 vec[1, 3, 5, 7]" (-1) (chop 6 vec))
test19 = TestCase (do let vec = generate  (5) (\i -> i * 2 + 1)
                      assertEqual "chop 8 vec[1, 3, 5, 7]" (-1) (chop 8 vec))

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
